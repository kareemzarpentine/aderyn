use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;

use std::fmt::Display;

impl Statement {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                Some(variable_declaration_statement.id)
            }
            Statement::IfStatement(if_statement) => Some(if_statement.id),
            Statement::ForStatement(for_statement) => Some(for_statement.id),
            Statement::WhileStatement(while_statement) => Some(while_statement.id),
            Statement::EmitStatement(emit_statement) => emit_statement.event_call.get_node_id(),
            Statement::UncheckedBlock(unchecked_statement) => Some(unchecked_statement.id),
            Statement::Return(return_statement) => Some(return_statement.id),
            Statement::RevertStatement(revert_statement) => Some(revert_statement.error_call.id),
            Statement::ExpressionStatement(expression_statement) => {
                expression_statement.expression.get_node_id()
            }
            Statement::InlineAssembly(inline_assembly) => Some(inline_assembly.id),
            Statement::TryStatement(_) => None,
            Statement::Block(block) => Some(block.id),
            Statement::Break(break_statement) => Some(break_statement.id),
            Statement::Continue(continue_statement) => Some(continue_statement.id),
            Statement::DoWhileStatement(do_while_statement) => Some(do_while_statement.id),
            Statement::PlaceholderStatement(placeholder) => Some(placeholder.id),
        }
    }
}

impl Node for Statement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            Statement::VariableDeclarationStatement(variable_declaration_statement) => {
                variable_declaration_statement.accept(visitor)
            }
            Statement::IfStatement(if_statement) => if_statement.accept(visitor),
            Statement::ForStatement(for_statement) => for_statement.accept(visitor),
            Statement::WhileStatement(while_statement) => while_statement.accept(visitor),
            Statement::EmitStatement(emit_statement) => emit_statement.accept(visitor),
            Statement::TryStatement(try_statement) => try_statement.accept(visitor),
            Statement::UncheckedBlock(unchecked_statement) => unchecked_statement.accept(visitor),
            Statement::Return(return_statement) => return_statement.accept(visitor),
            Statement::RevertStatement(revert_statement) => revert_statement.accept(visitor),
            Statement::ExpressionStatement(expression_statement) => {
                expression_statement.accept(visitor)
            }
            Statement::InlineAssembly(inline_assembly) => inline_assembly.accept(visitor),
            Statement::Block(block) => block.accept(visitor),
            Statement::Break(break_statement) => break_statement.accept(visitor),
            Statement::Continue(continue_statement) => continue_statement.accept(visitor),
            Statement::DoWhileStatement(do_while_statement) => do_while_statement.accept(visitor),
            Statement::PlaceholderStatement(placeholder_statement) => {
                placeholder_statement.accept(visitor)
            }
        }
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

impl Statement {
    pub fn is_return_statement(&self) -> bool {
        matches!(self, Statement::Return(_))
    }
}

impl Node for ExpressionStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_expression_statement(self)? {
            self.expression.accept(visitor)?;
        }
        visitor.end_visit_expression_statement(self)
    }
}

impl Node for VariableDeclarationStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_variable_declaration_statement(self)? {
            for declaration in &self.declarations {
                if declaration.is_some() {
                    declaration.as_ref().unwrap().accept(visitor)?;
                }
            }
            if self.initial_value.is_some() {
                self.initial_value.as_ref().unwrap().accept(visitor)?;
            }
            self.accept_metadata(visitor)?;
        }
        visitor.end_visit_variable_declaration_statement(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let declaration_ids = self
            .declarations
            .iter()
            .flatten()
            .map(|x| x.id)
            .collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, declaration_ids)?;
        if let Some(initial_value) = &self.initial_value {
            if let Some(id) = initial_value.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![id])?;
            }
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl BlockOrStatement {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            BlockOrStatement::Block(block) => Some(block.id),
            BlockOrStatement::Statement(statement) => statement.get_node_id(),
        }
    }
}

impl Node for BlockOrStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            BlockOrStatement::Block(block) => block.accept(visitor),
            BlockOrStatement::Statement(statement) => statement.accept(visitor),
        }
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

impl BlockOrStatement {
    pub fn contains_returns(&self) -> bool {
        match self {
            BlockOrStatement::Block(block) => block
                .statements
                .last()
                .map(|s| BlockOrStatement::Statement(Box::new(s.clone())).contains_returns())
                .unwrap_or(false),

            BlockOrStatement::Statement(statement) => match statement.as_ref() {
                Statement::Return(Return { .. }) => true,

                Statement::IfStatement(IfStatement {
                    true_body,
                    false_body,
                    ..
                }) => {
                    if !true_body.contains_returns() {
                        return false;
                    }

                    match false_body {
                        Some(false_body) => false_body.contains_returns(),
                        None => true,
                    }
                }

                _ => false,
            },
        }
    }
}

impl Node for IfStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_if_statement(self)? {
            self.condition.accept(visitor)?;
            self.true_body.accept(visitor)?;
            if self.false_body.is_some() {
                self.false_body.as_ref().unwrap().accept(visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_if_statement(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(cond_id) = self.condition.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![cond_id])?;
        }
        if let Some(true_body_id) = self.true_body.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![true_body_id])?;
        }
        if let Some(false_body) = &self.false_body {
            if let Some(false_body_id) = false_body.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![false_body_id])?;
            }
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for ForStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_for_statement(self)? {
            if self.initialization_expression.is_some() {
                self.initialization_expression
                    .as_ref()
                    .unwrap()
                    .accept(visitor)?;
            }
            if self.condition.is_some() {
                self.condition.as_ref().unwrap().accept(visitor)?;
            }
            if self.loop_expression.is_some() {
                self.loop_expression.as_ref().unwrap().accept(visitor)?;
            }
            self.body.accept(visitor)?;
            self.accept_metadata(visitor)?;
        }
        visitor.end_visit_for_statement(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(initialization_expr) = &self.initialization_expression {
            if let Some(expr_id) = initialization_expr.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![expr_id])?;
            }
        }
        if let Some(condition) = &self.condition {
            if let Some(cond_id) = condition.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![cond_id])?;
            }
        }
        if let Some(loop_expr) = &self.loop_expression {
            if let Some(loop_id) = loop_expr.expression.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![loop_id])?;
            }
        }
        if let Some(body_id) = self.body.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![body_id])?;
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for WhileStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_while_statement(self)? {
            self.condition.accept(visitor)?;
            self.body.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_while_statement(self)
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(cond_id) = self.condition.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![cond_id])?;
        }

        if let Some(body_id) = self.body.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![body_id])?;
        }
        Ok(())
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for DoWhileStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_do_while_statement(self)? {
            self.condition.accept(visitor)?;
            self.body.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_do_visit_while_statement(self)
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(cond_id) = self.condition.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![cond_id])?;
        }
        visitor.visit_immediate_children(self.id, vec![self.body.id])?;
        Ok(())
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for EmitStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_emit_statement(self)? {
            self.event_call.accept(visitor)?;
        }
        visitor.end_visit_emit_statement(self)
    }
}

impl Node for TryStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_try_statement(self)? {
            self.external_call.accept(visitor)?;
            list_accept(&self.clauses, visitor)?;
        }
        visitor.end_visit_try_statement(self)
    }
}

impl Node for RevertStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_revert_statement(self)? {
            self.error_call.accept(visitor)?;
        }
        visitor.end_visit_revert_statement(self)
    }
}

impl Node for TryCatchClause {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_try_catch_clause(self)? {
            if self.parameters.is_some() {
                self.parameters.as_ref().unwrap().accept(visitor)?;
            }
            self.block.accept(visitor)?;
        }
        visitor.end_visit_try_catch_clause(self)
    }
}

impl Node for Return {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_return(self)? && self.expression.is_some() {
            self.expression.as_ref().unwrap().accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_return(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(expr) = &self.expression {
            if let Some(expr_id) = expr.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![expr_id])?;
            }
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for InlineAssembly {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_inline_assembly(self)? && self.ast.is_some() {
            self.ast.as_ref().unwrap().accept(visitor)?;
        }
        visitor.end_visit_inline_assembly(self)
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for Break {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_break_statement(self)?;
        visitor.end_visit_break_statement(self)
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for Continue {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_continue_statement(self)?;
        visitor.end_visit_continue_statement(self)
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for PlaceholderStatement {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_placeholder_statement(self)?;
        visitor.end_visit_placeholder_statement(self)
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
