use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use crate::ast::NodeID;

use crate::capture;
use crate::context::browser::ExtractReferencedDeclarations;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UninitializedLocalVariableDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for UninitializedLocalVariableDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Assumption:
        // VariableDeclarationStatements consists of statements that look like `uint x;` `uint y, z;`, `uint p = 12;`
        // but are not declared at the contract level (state level) but rather within functions and modifiers

        let mut potentially_uninitialized_local_variables = HashSet::new();

        for variable_declaration_statement in context
            .variable_declaration_statements()
            .into_iter()
            .filter(|s| s.initial_value.is_none())
        {
            potentially_uninitialized_local_variables.extend(
                variable_declaration_statement
                    .declarations
                    .iter()
                    .flat_map(|s| {
                        if let Some(ref s) = s {
                            return Some(s.id);
                        }
                        None
                    }),
            );
        }

        // We can filter out the initialized variables by looking at LHS of assignments.
        // This trick works for local variables because it's not possible to have structs, mappings, dynamic arrays
        // declared local to the function.
        for assignment in context.assignments() {
            let references =
                ExtractReferencedDeclarations::from(assignment.left_hand_side.as_ref().into())
                    .extracted;
            potentially_uninitialized_local_variables.retain(|v| !references.contains(v));
        }

        for id in potentially_uninitialized_local_variables {
            if let Some(node) = context.nodes.get(&id) {
                capture!(self, context, node);
            }
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Uninitialized local variables.")
    }

    fn description(&self) -> String {
        String::from("Initialize all the variables. If a variable is meant to be initialized to zero, explicitly set it to zero to improve code readability.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::UninitializedLocalVariable)
    }
}

#[cfg(test)]
mod uninitialized_local_variables_detector_tests {
    use serial_test::serial;

    use crate::detect::{
        detector::IssueDetector,
        low::uninitialized_local_variables::UninitializedLocalVariableDetector,
    };

    #[test]
    #[serial]
    fn test_uninitialized_local_variables() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/UninitializedLocalVariables.sol",
        );

        let mut detector = UninitializedLocalVariableDetector::default();
        let found = detector.detect(&context).unwrap();

        println!(
            "Line numbers of uninitialized local variables: {:?}",
            detector
                .instances()
                .into_iter()
                .map(|(i, _)| i.1)
                .collect::<Vec<_>>()
        );

        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 12);
        // assert the severity is low
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::Low
        );
    }
}
