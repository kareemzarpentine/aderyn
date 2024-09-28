use crate::context::{browser::Peek, workspace_context::WorkspaceContext};

use super::{primitives::*, voids::*, CfgBreakStatement, CfgNodeDescriptor};

impl CfgNodeDescriptor {
    pub fn display(&self, context: &WorkspaceContext) -> String {
        match self {
            // Voids
            CfgNodeDescriptor::Start(n) => n.peek(),
            CfgNodeDescriptor::End(n) => n.peek(),

            // Primitives
            CfgNodeDescriptor::VariableDeclarationStatement(n) => n.peek(context),
            CfgNodeDescriptor::ExpressionStatement(n) => n.peek(context),
            CfgNodeDescriptor::PlaceholderStatement(n) => n.peek(),
            CfgNodeDescriptor::Break(n) => n.peek(),
            CfgNodeDescriptor::Continue(n) => n.peek(),
            CfgNodeDescriptor::Return(n) => n.peek(context),
            CfgNodeDescriptor::EmitStatement(n) => n.peek(context),
            CfgNodeDescriptor::RevertStatement(n) => n.peek(context),
            CfgNodeDescriptor::InlineAssembly(n) => n.peek(context),
            CfgNodeDescriptor::IfStatementCondition(n) => n.peek(context),

            // Reducibles
            CfgNodeDescriptor::IfStatement(_) => String::from("REDUCIBLE IF-STATEMENT"),
            CfgNodeDescriptor::Block(_) => String::from("REDUCIBLE BLOCK"),
        }
    }
}

impl CfgStartNode {
    pub fn peek(&self) -> String {
        match self {
            CfgStartNode::Start => String::from("START"),
            CfgStartNode::StartBlock(ast_id) => format!("START BLOCK ({})", ast_id),
            CfgStartNode::StartIf(ast_id) => format!("START IF ({})", ast_id),
            CfgStartNode::StartIfCond => String::from("START IF COND"),
            CfgStartNode::StartIfTrue => String::from("START IF TRUE BRANCH"),
            CfgStartNode::StartIfFalse => String::from("START IF FALSE BRANCH"),
        }
    }
}

impl CfgEndNode {
    pub fn peek(&self) -> String {
        match self {
            CfgEndNode::End => String::from("END"),
            CfgEndNode::EndBlock(ast_id) => format!("END BLOCK ({})", ast_id),
            CfgEndNode::EndIf(ast_id) => format!("END IF ({})", ast_id),
            CfgEndNode::EndIfCond => String::from("END IF COND"),
            CfgEndNode::EndIfTrue => String::from("END IF TRUE BRANCH"),
            CfgEndNode::EndIfFalse => String::from("END IF FALSE BRANCH"),
        }
    }
}

impl CfgVariableDeclarationStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!(
            "Variable Decl. Stmt ({})",
            self.variable_declaration_statement
        );
        if let Some(node) = context.nodes.get(&self.variable_declaration_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgExpressionStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Expression Stmt ({})", self.expression_statement);
        if let Some(node) = context.nodes.get(&self.expression_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgBreakStatement {
    pub fn peek(&self) -> String {
        format!("BREAK ({})", &self.break_statement)
    }
}

impl CfgContinueStatement {
    pub fn peek(&self) -> String {
        format!("CONTINUE ({})", &self.continue_statement)
    }
}

impl CfgReturnStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Return Stmt ({})", self.return_statement);
        if let Some(node) = context.nodes.get(&self.return_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgEmitStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Emit Stmt ({})", self.emit_statement);
        if let Some(node) = context.nodes.get(&self.emit_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgRevertStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Revert Stmt ({})", self.revert_statement);
        if let Some(node) = context.nodes.get(&self.revert_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgInlineAssemblyStatement {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let mut content = format!("Inline Assembly Stmt ({})", self.inline_assembly_statement);
        if let Some(node) = context.nodes.get(&self.inline_assembly_statement) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}

impl CfgPlaceholderStatement {
    pub fn peek(&self) -> String {
        let mut content = format!("Placeholder statement ({})", self.placeholder_statement);
        content.push_str(": \n_");
        content
    }
}

impl CfgIfStatementCondition {
    pub fn peek(&self, context: &WorkspaceContext) -> String {
        let Some(if_cond) = self.if_stmt_condition else {
            return String::from("If Cond");
        };
        let mut content = format!("If Cond ({})", if_cond);
        if let Some(node) = context.nodes.get(&if_cond) {
            if let Some(inside) = node.peek(context) {
                content.push_str(&format!(": \n{}", inside));
            }
        }
        content
    }
}
