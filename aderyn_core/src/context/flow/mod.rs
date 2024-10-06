pub mod display;
pub mod error;
pub mod kind;
pub mod primitives;
pub mod reducibles;
pub mod utils;
pub mod visualizer;
pub mod voids;

use crate::{
    ast::*,
    context::flow::utils::{discover_jump_sources, Callibration},
};

pub use kind::CfgNodeKind;
pub use reducibles::CfgBlock;

use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use self::{
    primitives::*,
    reducibles::{
        CfgDoWhileStatement, CfgForStatement, CfgIfStatement, CfgUncheckedBlock, CfgWhileStatement,
    },
    utils::find_jump_dest,
    voids::{CfgEndNode, CfgStartNode},
};

use super::workspace_context::WorkspaceContext;

// This is done to differentiate AstNodeIDs from CfgNodeIDs
type AstNodeId = NodeID;

#[derive(Eq, Hash, Default, PartialEq, Clone, Copy, Debug)]
pub struct CfgNodeId(usize);

impl CfgNodeId {
    #[allow(dead_code)]
    fn peek(&self) -> usize {
        self.0
    }
    #[allow(dead_code)]
    fn peek_mut(&mut self) -> &mut usize {
        &mut self.0
    }
    fn advance(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Clone)]
pub enum CfgNodeDescriptor {
    // Void nodes
    Start(Box<CfgStartNode>),
    End(Box<CfgEndNode>),

    // Primitives
    VariableDeclarationStatement(Box<CfgVariableDeclarationStatement>),
    ExpressionStatement(Box<CfgExpressionStatement>),
    PlaceholderStatement(Box<CfgPlaceholderStatement>),
    Break(Box<CfgBreakStatement>),
    Continue(Box<CfgContinueStatement>),
    Return(Box<CfgReturnStatement>),
    EmitStatement(Box<CfgEmitStatement>),
    RevertStatement(Box<CfgRevertStatement>),
    InlineAssembly(Box<CfgInlineAssemblyStatement>),
    IfStatementCondition(Box<CfgIfStatementCondition>),
    WhileStatementCondition(Box<CfgWhileStatementCondition>),
    ForStatementCondition(Box<CfgForStatementCondition>),
    DoWhileStatementCondition(Box<CfgDoWhileStatementCondition>),

    // Reducibles
    Block(Box<CfgBlock>),
    UncheckedBlock(Box<CfgUncheckedBlock>),
    IfStatement(Box<CfgIfStatement>),
    WhileStatement(Box<CfgWhileStatement>),
    ForStatement(Box<CfgForStatement>),
    DoWhileStatement(Box<CfgDoWhileStatement>),
}

#[derive(Debug, Clone)]
pub struct CfgNode {
    /// Node ID
    pub id: CfgNodeId,
    /// Node descriptor
    pub nd: CfgNodeDescriptor,
}

/// Control fow graph
#[derive(Default, Debug)]
pub struct Cfg {
    /// Node registry
    pub nodes: HashMap<CfgNodeId, CfgNode>,

    /// Adjacency list representation of the Control Flow Graph
    pub adj_list: HashMap<CfgNodeId, Vec<CfgNodeId>>,

    /// ID to construct the next node
    next_available_id: CfgNodeId,

    /// Current reducibles
    reduction_queue: VecDeque<CfgNodeId>,

    /// Lookup the corresponding End node for any start node (Start*, End*)
    start_end_pairs: HashMap<CfgNodeId, CfgNodeId>,

    /// Lookup the Condition node for a given loop's start node
    start_cond_pairs: HashMap<CfgNodeId, CfgNodeId>,

    /// Lookup the loop_expression node for a given for loop's start node
    start_loop_expr: HashMap<CfgNodeId, CfgNodeId>,

    /// Lookup the StartFor given end of loop_expr
    loop_expr_start: HashMap<CfgNodeId, CfgNodeId>,

    /// Lookup the StartWhile, StartDoWhile of a loop condition given condition
    cond_start_pairs: HashMap<CfgNodeId, CfgNodeId>,
}

pub trait CfgReduce {
    fn reduce(&self, context: &WorkspaceContext, cfg: &mut Cfg) -> (CfgNodeId, CfgNodeId);
}

impl Cfg {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn total_nodes(&self) -> usize {
        debug_assert_eq!(self.next_available_id.peek(), self.nodes.len());
        self.next_available_id.peek()
    }
    pub fn total_edges(&self) -> usize {
        self.adj_list.values().map(|conn| conn.len()).sum()
    }
    fn add_raw_node(&mut self, id: CfgNodeId, nd: CfgNodeDescriptor) {
        let cfg_node = CfgNode { id, nd };
        self.nodes.insert(id, cfg_node);
    }
    fn add_raw_directed_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        match self.adj_list.entry(from) {
            Entry::Vacant(v) => {
                v.insert(vec![to]);
            }
            Entry::Occupied(mut o) => {
                o.get_mut().push(to);
            }
        };
    }
    fn remove_raw_directed_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        let existing_nodes = self.adj_list.get_mut(&from).expect("Relationship doesn't exist");
        existing_nodes.retain_mut(|x| *x != to);
    }
    fn raw_predecessors(&self, id: CfgNodeId) -> Vec<CfgNodeId> {
        let mut predecessors = vec![];
        for (from, to_list) in &self.adj_list {
            if to_list.contains(&id) {
                predecessors.push(*from);
            }
        }
        predecessors
    }
    fn raw_successors(&self, id: CfgNodeId) -> Vec<CfgNodeId> {
        let Some(successors) = self.adj_list.get(&id) else {
            return Default::default();
        };
        successors.to_vec()
    }
    fn remove_raw_edges_involving(&mut self, node_id: CfgNodeId) {
        // Remove all successors' edges starting from node_id
        self.adj_list.remove(&node_id);

        // Remove edges ending at node_id
        for (_, to_list) in self.adj_list.iter_mut() {
            to_list.retain_mut(|x| *x != node_id);
        }
    }
    fn reset_raw_successors(&mut self, node_id: CfgNodeId, to: CfgNodeId) {
        // Remove edges starting from node_id
        self.adj_list.remove(&node_id);
        self.add_raw_directed_edge(node_id, to);
    }
}

impl Cfg {
    /// Assigns a unique ID to the given node and adds it to the CFG
    #[must_use]
    pub fn add_node(&mut self, nd: CfgNodeDescriptor) -> CfgNodeId {
        // Grab the currently available id
        let node_id = self.next_available_id;

        // Increment the ID for next use
        self.next_available_id.advance();

        // Check that node with that ID has not already been inserted
        assert!(!self.nodes.contains_key(&node_id));

        // Queue the node to be reduced if it is reducible
        if nd.kind() == CfgNodeKind::Reducible {
            self.reduction_queue.push_back(node_id);
        }

        // Add the node to the CFG
        self.add_raw_node(node_id, nd);

        //Maintain map from ast to cfg
        //if let Some(ast_id) = nd.reflect() {
        //    self.ast_to_cfg.insert(ast_id, node_id);
        //}

        // Check that node has been inserted
        assert!(self.nodes.contains_key(&node_id));

        // Return the ID of the freshly inserted node
        node_id
    }

    /// Disconnects existing relationships (mostly used during reduction)
    pub fn remove_flow_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        self.remove_raw_directed_edge(from, to);
    }

    /// Connects the given two given nodes in the CFG
    pub fn add_flow_edge(&mut self, from: CfgNodeId, to: CfgNodeId) {
        self.add_raw_directed_edge(from, to);
    }

    /// Reduce the reducible nodes stored the queue at the time of adding nodes
    pub fn reduce(&mut self, context: &WorkspaceContext, reduction_candidate: CfgNodeId) {
        // Step 0: Remove the node that's being reduced
        let cfg_node =
            self.nodes.remove(&reduction_candidate).expect("Reduction candidate doesn't exist");

        // Step 1: Get the predecessors
        let predecessors = self.raw_predecessors(reduction_candidate);

        // Step 2: Get the successors
        let successors = self.raw_successors(reduction_candidate);

        // Step 3: Remove existing predecessor relationships with reduction candidate to build new
        // ones
        for pred in &predecessors {
            self.remove_flow_edge(*pred, cfg_node.id);
        }

        // Step 4: Remove existing predecessor relationships with reduction candidate to build new
        // ones
        for succ in &successors {
            self.remove_flow_edge(cfg_node.id, *succ);
        }

        // Step 5: Get the (start s, end e) of the reduced cfg
        let (start_id, end_id) = match cfg_node.nd {
            // Voids and Primitives
            CfgNodeDescriptor::Start(_)
            | CfgNodeDescriptor::End(_)
            | CfgNodeDescriptor::VariableDeclarationStatement(_)
            | CfgNodeDescriptor::Break(_)
            | CfgNodeDescriptor::Return(_)
            | CfgNodeDescriptor::Continue(_)
            | CfgNodeDescriptor::RevertStatement(_)
            | CfgNodeDescriptor::PlaceholderStatement(_)
            | CfgNodeDescriptor::InlineAssembly(_)
            | CfgNodeDescriptor::EmitStatement(_)
            | CfgNodeDescriptor::ExpressionStatement(_)
            | CfgNodeDescriptor::WhileStatementCondition(_)
            | CfgNodeDescriptor::ForStatementCondition(_)
            | CfgNodeDescriptor::DoWhileStatementCondition(_)
            | CfgNodeDescriptor::IfStatementCondition(_) => {
                unreachable!("Expect only reducible nodes")
            }

            // Reducibles
            CfgNodeDescriptor::Block(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::UncheckedBlock(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::IfStatement(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::WhileStatement(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::ForStatement(cfg_block) => cfg_block.reduce(context, self),
            CfgNodeDescriptor::DoWhileStatement(cfg_block) => cfg_block.reduce(context, self),
        };

        // Step 6: Connect all the predecessors to `s`
        for pred in &predecessors {
            self.add_flow_edge(*pred, start_id);
        }

        // Step 7: Connect `e` to all the successors
        for succ in &successors {
            self.add_flow_edge(end_id, *succ);
        }

        // Step 8: Remove existing connections with redution_candidate
        self.remove_raw_edges_involving(reduction_candidate);
    }

    /// Corrects the flow of continue, break and return statements
    ///
    /// This is hard to perform at the time of reduction so it must be done post-reduction.
    ///
    /// Continue CFG Nodes should flow back the parent loop's condition node in case of a `while` or
    /// `do while` and likewise to the the parent loop's update expression in case of `for`.
    ///
    /// Break CFG Nodes should always flow to the end of the parent loop
    ///
    /// Return CFG Nodes should flow to the end of the function body or a modifier body
    ///
    /// Arguments
    ///
    /// * start_node - Node discovery starts here at this point.
    ///
    /// * end_node - Return statements flow to here. It also serves as a fallback for break and
    ///   continue statements if parent loop is not found
    pub fn callibrate_jump_statements_in_function_body(
        &mut self,
        start_node: CfgNodeId,
        end_node: CfgNodeId,
    ) {
        // Jump sources
        let mut continue_statements = vec![];
        let mut break_statements = vec![];
        let mut return_statements = vec![];

        let mut visited: HashSet<CfgNodeId> = Default::default();

        // Start node sets the scope of discovery
        discover_jump_sources(
            self,
            start_node,
            &mut visited,
            &mut continue_statements,
            &mut break_statements,
            &mut return_statements,
        );

        // Proposed Callibrations
        let mut proposed_callibrations = vec![];

        for continue_statement in continue_statements {
            let mut visited = Default::default();
            let dest = find_jump_dest(self, continue_statement, &mut visited).unwrap_or_default();
            proposed_callibrations
                .push(Callibration::ContinueShouldFlowTo(continue_statement, dest));
        }

        for break_statement in break_statements {
            let mut visited = Default::default();
            let dest = find_jump_dest(self, break_statement, &mut visited).unwrap_or_default();
            proposed_callibrations.push(Callibration::BreakShouldFlowTo(break_statement, dest));
        }

        for return_statement in return_statements {
            proposed_callibrations.push(Callibration::ReturnShouldFlowToEndNode(return_statement));
        }

        // End node now comes into play
        self.callibrate(proposed_callibrations, end_node);
    }

    pub fn find_ending_counter_part(&self, start_node_id: CfgNodeId) -> CfgNodeId {
        *self.start_end_pairs.get(&start_node_id).expect("ending counter part not found!")
    }

    pub fn find_condition_node(&self, start_loop_id: CfgNodeId) -> CfgNodeId {
        *self.start_cond_pairs.get(&start_loop_id).expect("could not resolve condition!")
    }

    pub fn find_loop_expression_node(&self, start_loop_id: CfgNodeId) -> CfgNodeId {
        *self.start_loop_expr.get(&start_loop_id).expect("could not resolve loop_expression!")
    }
}

#[cfg(test)]
mod control_flow_tests {
    use super::*;
    use crate::{
        context::flow::visualizer::control_flow_tests::output_graph,
        detect::test_utils::load_solidity_source_unit,
    };
    use serial_test::serial;

    // Sample use of CFG
    impl Cfg {
        pub fn accept_function_body(
            &mut self,
            context: &WorkspaceContext,
            function_definition: &FunctionDefinition,
        ) {
            let Some(function_body_block) = function_definition.body.as_ref() else {
                return;
            };

            let start = self.add_start_function_body_node(function_definition.id);
            let end = self.add_end_function_body_node(function_definition.id);
            self.start_end_pairs.insert(start, end);

            let block = self.add_block_node(function_body_block);

            self.add_flow_edge(start, block);
            self.add_flow_edge(block, end);

            while let Some(reduction_candidate) = self.reduction_queue.pop_front() {
                self.reduce(context, reduction_candidate);
            }

            self.callibrate_jump_statements_in_function_body(start, end);
        }

        pub fn accept_block(&mut self, context: &WorkspaceContext, block: &Block) {
            let start = self.add_start_node();
            let end = self.add_end_node();
            let block = self.add_block_node(block);

            self.add_flow_edge(start, block);
            self.add_flow_edge(block, end);

            while let Some(reduction_candidate) = self.reduction_queue.pop_front() {
                self.reduce(context, reduction_candidate);
            }
        }
    }

    // Accept block (Pre callibration checks)
    #[test]
    #[serial]
    fn simple_program_function1() {
        /*

        First example
        --------------
        Consider
        ../tests/contract-playground/src/control_flow/SimpleProgram.sol
        SimpleProgram : function1

        Deconstruct the function step by step until we have a graph with only
        Every function has a body Block so we start with the following graph and reduce it to primitives

        Step 1:

            Let 'a be the ID node the CfgNode(Block b)

            reduction_queue : [ 'a ]

            Sn(Block) -> CfgNode(Block b) 'a -> En(Block)

            Short form:
            Sn -> CfgStartNode
            En -> CfgEndNode

        Step 2:

            reduction_queue: [ ]

            Sn ->
                Sn -> CfgNode(VariableDeclarationStatement v) -> En ->
                Sn -> CfgNode(ExpressionStatement e) -> En ->
                Sn -> CfgNode(ExpressionStatement e) -> En ->
            En

        */

        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function1");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function1 not to be defined"));

        assert_eq!(cfg.nodes.len(), 7);

        assert!(matches!(
            cfg.nodes.get(&CfgNodeId(3)).unwrap(),
            CfgNode { id: _, nd: CfgNodeDescriptor::Start(_) }
        ));

        assert!(matches!(
            cfg.nodes.get(&CfgNodeId(4)).unwrap(),
            CfgNode { id: _, nd: CfgNodeDescriptor::End(_) }
        ));

        output_graph(&context, &cfg, "SimpleProgram_function1");
    }

    #[test]
    #[serial]
    fn simple_program_function2() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function2");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function2 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function2");
        assert_eq!(cfg.nodes.len(), 14);
    }

    #[test]
    #[serial]
    fn simple_program_function3() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function3");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function3 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function3");
        assert_eq!(cfg.nodes.len(), 12);
    }

    #[test]
    #[serial]
    fn simple_program_function4() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function4");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function4 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function4");
        assert_eq!(cfg.nodes.len(), 48);
    }

    #[test]
    #[serial]
    fn simple_program_function5() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function5");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function5 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function5");
        assert_eq!(cfg.nodes.len(), 25);
    }

    #[test]
    #[serial]
    fn simple_program_function6() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function6");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function6 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function6");
        assert_eq!(cfg.nodes.len(), 31);
    }

    #[test]
    #[serial]
    fn simple_program_function7() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function7");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function7 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function7");
        assert_eq!(cfg.nodes.len(), 22);
    }

    #[test]
    #[serial]
    fn simple_program_function8() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function8");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function8 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function8");
        assert_eq!(cfg.nodes.len(), 48);
    }

    #[test]
    #[serial]
    fn simple_program_function9() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function9");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function9 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function9");
        assert_eq!(cfg.nodes.len(), 15);
    }

    #[test]
    #[serial]
    fn simple_program_function10() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function10");
        let mut cfg = Cfg::new();

        cfg.accept_block(&context, function.body.as_ref().expect("function10 not to be defined"));

        output_graph(&context, &cfg, "SimpleProgram_function10");
        assert_eq!(cfg.nodes.len(), 9);
    }

    // Accept-Function-Body (Post callibration checks)

    #[test]
    #[serial]
    fn simple_program_function11() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function11");
        let mut cfg = Cfg::new();

        cfg.accept_function_body(&context, function);

        output_graph(&context, &cfg, "SimpleProgram_function11");
        assert_eq!(cfg.nodes.len(), 26);
        assert_eq!(cfg.total_edges(), 27);
    }

    #[test]
    #[serial]
    fn simple_program_function12() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function12");
        let mut cfg = Cfg::new();

        cfg.accept_function_body(&context, function);

        output_graph(&context, &cfg, "SimpleProgram_function12");
        assert_eq!(cfg.nodes.len(), 42);
        assert_eq!(cfg.total_edges(), 44);
    }

    #[test]
    #[serial]
    fn simple_program_function13() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function13");
        let mut cfg = Cfg::new();

        cfg.accept_function_body(&context, function);

        output_graph(&context, &cfg, "SimpleProgram_function13");
        assert_eq!(cfg.nodes.len(), 36);
        assert_eq!(cfg.total_edges(), 38);
    }

    #[test]
    #[serial]
    fn simple_program_function14() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function14");
        let mut cfg = Cfg::new();

        cfg.accept_function_body(&context, function);

        output_graph(&context, &cfg, "SimpleProgram_function14");
        assert_eq!(cfg.nodes.len(), 46);
        assert_eq!(cfg.total_edges(), 49);
    }

    #[test]
    #[serial]
    fn simple_program_function15() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function15");
        let mut cfg = Cfg::new();

        cfg.accept_function_body(&context, function);

        output_graph(&context, &cfg, "SimpleProgram_function15");
        assert_eq!(cfg.nodes.len(), 70);
        assert_eq!(cfg.total_edges(), 75);
    }

    #[test]
    #[serial]
    fn simple_program_function16() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/control_flow/SimpleProgram.sol",
        );
        let contract = context.find_contract_by_name("SimpleProgram");
        let function = contract.find_function_by_name("function16");
        let mut cfg = Cfg::new();

        cfg.accept_function_body(&context, function);

        output_graph(&context, &cfg, "SimpleProgram_function16");
        assert_eq!(cfg.nodes.len(), 82);
        assert_eq!(cfg.total_edges(), 88);
    }
}
