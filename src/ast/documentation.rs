use super::{node::*, *};
use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Documentation {
    String(Option<String>),
    Structured(Option<StructuredDocumentation>),
}

impl BaseNode for Documentation {
    fn accept(&self, visitor: &mut impl AstBaseVisitor) -> Result<()> {
        match self {
            Documentation::String(_opt_string) => {
                // TODO check if this is okay
                Ok(())
            }
            Documentation::Structured(opt_structured_documentation) => {
                if opt_structured_documentation.is_some() {
                    opt_structured_documentation
                        .as_ref()
                        .unwrap()
                        .accept(visitor)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub text: String,
    pub src: String,
    pub id: NodeID,
}

impl BaseNode for StructuredDocumentation {
    fn accept(&self, visitor: &mut impl AstBaseVisitor) -> Result<()> {
        visitor.visit_structured_documentation(self)?;
        visitor.end_visit_structured_documentation(self)
    }
}
