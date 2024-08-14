mod callgraph_tests;
pub mod graph;
mod workspace_callgraph;

pub use workspace_callgraph::*;

use derive_more::From;

use crate::ast::{ASTNode, NodeID};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    // region: -- standard::* errors
    WorkspaceCallGraphDFSError,
    CallgraphNotAvailable,
    UnidentifiedEntryPointNode(ASTNode),
    InvalidEntryPointId(NodeID),
    EntryPointVisitError,
    FunctionDefinitionVisitError,
    ModifierDefinitionVisitError,
    // endregion
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Custom(value.to_string())
    }
}

impl std::error::Error for Error {}
