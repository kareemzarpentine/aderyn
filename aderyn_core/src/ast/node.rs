use serde::{Deserialize, Serialize};

use crate::context::workspace_context::WorkspaceContext;

pub type NodeID = i64;

pub type UniqueNodeID = (i64, i64); // Source Unit ID, Node ID

pub trait HasUniqueID {
    fn uid(&self, context: &WorkspaceContext) -> Option<UniqueNodeID>;
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum NodeType {
    SourceUnit,
    PragmaDirective,
    ImportDirective,
    UsingForDirective,
    ContractDefinition,
    InheritanceSpecifier,
    OverrideSpecifier,
    IdentifierPath,
    StructuredDocumentation,
    VariableDeclaration,
    Mapping,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    ArrayTypeName,
    TupleExpression,
    FunctionDefinition,
    ParameterList,
    Block,
    UncheckedBlock,
    Continue,
    Break,
    Return,
    Throw,
    Literal,
    Conditional,
    Identifier,
    IndexAccess,
    IndexRangeAccess,
    MemberAccess,
    Assignment,
    FunctionCall,
    FunctionCallOptions,
    FunctionTypeName,
    NewExpression,
    ExpressionStatement,
    VariableDeclarationStatement,
    IfStatement,
    TryCatchClause,
    UnaryOperation,
    BinaryOperation,
    EventDefinition,
    ErrorDefinition,
    EmitStatement,
    PlaceholderStatement,
    TryStatement,
    RevertStatement,
    ForStatement,
    WhileStatement,
    ModifierDefinition,
    ModifierInvocation,
    EnumDefinition,
    EnumValue,
    StructDefinition,
    UserDefinedTypeName,
    InlineAssembly,
    YulLiteral,
    YulTypedName,
    YulSwitch,
    YulCase,
    YulFunctionCall,
    YulExpressionStatement,
    YulAssignment,
    YulIdentifier,
    YulVariableDeclaration,
    YulBlock,
    UserDefinedValueTypeDefinition,
}
