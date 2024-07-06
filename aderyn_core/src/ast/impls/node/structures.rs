use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;
use std::fmt::Display;

impl Node for StructDefinition {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_struct_definition(self)? {
            list_accept(&self.members, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_struct_definition(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let members_ids = &self.members.iter().map(|x| x.id).collect::<Vec<_>>();
        visitor.visit_immediate_children(self.id, members_ids.clone())?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
