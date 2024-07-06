use crate::{
    ast::*,
    context::macros::generate_ast_methods,
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    ArrayTypeName(ArrayTypeName),
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Block(Block),
    Conditional(Conditional),
    ContractDefinition(ContractDefinition),
    ElementaryTypeName(ElementaryTypeName),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    EmitStatement(EmitStatement),
    EnumDefinition(EnumDefinition),
    EnumValue(EnumValue),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    ExpressionStatement(ExpressionStatement),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    FunctionDefinition(FunctionDefinition),
    FunctionTypeName(FunctionTypeName),
    ForStatement(ForStatement),
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
    IfStatement(IfStatement),
    ImportDirective(ImportDirective),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    InheritanceSpecifier(InheritanceSpecifier),
    InlineAssembly(InlineAssembly),
    Literal(Literal),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
    Mapping(Mapping),
    ModifierDefinition(ModifierDefinition),
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ParameterList(ParameterList),
    PragmaDirective(PragmaDirective),
    Return(Return),
    RevertStatement(RevertStatement),
    SourceUnit(SourceUnit),
    StructDefinition(StructDefinition),
    StructuredDocumentation(StructuredDocumentation),
    TryStatement(TryStatement),
    TryCatchClause(TryCatchClause),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
    UserDefinedTypeName(UserDefinedTypeName),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    Break(Break),
    Continue(Continue),
    PlaceholderStatement(PlaceholderStatement),
}

impl ASTNode {
    pub fn node_type(&self) -> NodeType {
        match self {
            ASTNode::ArrayTypeName(_) => NodeType::ArrayTypeName,
            ASTNode::Assignment(_) => NodeType::Assignment,
            ASTNode::BinaryOperation(_) => NodeType::BinaryOperation,
            ASTNode::Block(_) => NodeType::Block,
            ASTNode::Conditional(_) => NodeType::Conditional,
            ASTNode::ContractDefinition(_) => NodeType::ContractDefinition,
            ASTNode::ElementaryTypeName(_) => NodeType::ElementaryTypeName,
            ASTNode::ElementaryTypeNameExpression(_) => NodeType::ElementaryTypeNameExpression,
            ASTNode::EmitStatement(_) => NodeType::EmitStatement,
            ASTNode::EnumDefinition(_) => NodeType::EnumDefinition,
            ASTNode::EnumValue(_) => NodeType::EnumValue,
            ASTNode::EventDefinition(_) => NodeType::EventDefinition,
            ASTNode::ErrorDefinition(_) => NodeType::ErrorDefinition,
            ASTNode::ExpressionStatement(_) => NodeType::ExpressionStatement,
            ASTNode::FunctionCall(_) => NodeType::FunctionCall,
            ASTNode::FunctionCallOptions(_) => NodeType::FunctionCallOptions,
            ASTNode::FunctionDefinition(_) => NodeType::FunctionDefinition,
            ASTNode::FunctionTypeName(_) => NodeType::FunctionTypeName,
            ASTNode::ForStatement(_) => NodeType::ForStatement,
            ASTNode::Identifier(_) => NodeType::Identifier,
            ASTNode::IdentifierPath(_) => NodeType::IdentifierPath,
            ASTNode::IfStatement(_) => NodeType::IfStatement,
            ASTNode::ImportDirective(_) => NodeType::ImportDirective,
            ASTNode::IndexAccess(_) => NodeType::IndexAccess,
            ASTNode::IndexRangeAccess(_) => NodeType::IndexRangeAccess,
            ASTNode::InheritanceSpecifier(_) => NodeType::InheritanceSpecifier,
            ASTNode::InlineAssembly(_) => NodeType::InlineAssembly,
            ASTNode::Literal(_) => NodeType::Literal,
            ASTNode::MemberAccess(_) => NodeType::MemberAccess,
            ASTNode::NewExpression(_) => NodeType::NewExpression,
            ASTNode::Mapping(_) => NodeType::Mapping,
            ASTNode::ModifierDefinition(_) => NodeType::ModifierDefinition,
            ASTNode::ModifierInvocation(_) => NodeType::ModifierInvocation,
            ASTNode::OverrideSpecifier(_) => NodeType::OverrideSpecifier,
            ASTNode::ParameterList(_) => NodeType::ParameterList,
            ASTNode::PragmaDirective(_) => NodeType::PragmaDirective,
            ASTNode::Return(_) => NodeType::Return,
            ASTNode::RevertStatement(_) => NodeType::RevertStatement,
            ASTNode::SourceUnit(_) => NodeType::SourceUnit,
            ASTNode::StructDefinition(_) => NodeType::StructDefinition,
            ASTNode::StructuredDocumentation(_) => NodeType::StructuredDocumentation,
            ASTNode::TryStatement(_) => NodeType::TryStatement,
            ASTNode::TryCatchClause(_) => NodeType::TryCatchClause,
            ASTNode::TupleExpression(_) => NodeType::TupleExpression,
            ASTNode::UnaryOperation(_) => NodeType::UnaryOperation,
            ASTNode::UserDefinedTypeName(_) => NodeType::UserDefinedTypeName,
            ASTNode::UserDefinedValueTypeDefinition(_) => NodeType::UserDefinedValueTypeDefinition,
            ASTNode::UsingForDirective(_) => NodeType::UsingForDirective,
            ASTNode::VariableDeclaration(_) => NodeType::VariableDeclaration,
            ASTNode::VariableDeclarationStatement(_) => NodeType::VariableDeclarationStatement,
            ASTNode::WhileStatement(_) => NodeType::WhileStatement,
            ASTNode::DoWhileStatement(_) => NodeType::DoWhileStatement,
            ASTNode::Break(_) => NodeType::Break,
            ASTNode::Continue(_) => NodeType::Continue,
            ASTNode::PlaceholderStatement(_) => NodeType::PlaceholderStatement,
        }
    }

    pub fn id(&self) -> Option<NodeID> {
        match self {
            ASTNode::ArrayTypeName(n) => Some(n.id),
            ASTNode::Assignment(n) => Some(n.id),
            ASTNode::BinaryOperation(n) => Some(n.id),
            ASTNode::Block(n) => Some(n.id),
            ASTNode::Conditional(n) => Some(n.id),
            ASTNode::ContractDefinition(n) => Some(n.id),
            ASTNode::ElementaryTypeName(n) => Some(n.id),
            ASTNode::ElementaryTypeNameExpression(n) => Some(n.id),
            ASTNode::EmitStatement(n) => Some(n.id),
            ASTNode::EnumDefinition(n) => Some(n.id),
            ASTNode::EnumValue(n) => Some(n.id),
            ASTNode::EventDefinition(n) => Some(n.id),
            ASTNode::ErrorDefinition(n) => Some(n.id),
            ASTNode::ExpressionStatement(n) => Some(n.id),
            ASTNode::FunctionCall(n) => Some(n.id),
            ASTNode::FunctionCallOptions(n) => Some(n.id),
            ASTNode::FunctionDefinition(n) => Some(n.id),
            ASTNode::FunctionTypeName(n) => Some(n.id),
            ASTNode::ForStatement(n) => Some(n.id),
            ASTNode::Identifier(n) => Some(n.id),
            ASTNode::IdentifierPath(n) => Some(n.id),
            ASTNode::IfStatement(n) => Some(n.id),
            ASTNode::ImportDirective(n) => Some(n.id),
            ASTNode::IndexAccess(n) => Some(n.id),
            ASTNode::IndexRangeAccess(n) => Some(n.id),
            ASTNode::InheritanceSpecifier(n) => Some(n.id),
            ASTNode::InlineAssembly(n) => Some(n.id),
            ASTNode::Literal(n) => Some(n.id),
            ASTNode::MemberAccess(n) => Some(n.id),
            ASTNode::NewExpression(n) => Some(n.id),
            ASTNode::Mapping(n) => Some(n.id),
            ASTNode::ModifierDefinition(n) => Some(n.id),
            ASTNode::ModifierInvocation(n) => Some(n.id),
            ASTNode::OverrideSpecifier(n) => Some(n.id),
            ASTNode::ParameterList(n) => Some(n.id),
            ASTNode::PragmaDirective(n) => Some(n.id),
            ASTNode::Return(n) => Some(n.id),
            ASTNode::RevertStatement(n) => Some(n.id),
            ASTNode::SourceUnit(n) => Some(n.id),
            ASTNode::StructDefinition(n) => Some(n.id),
            ASTNode::StructuredDocumentation(n) => Some(n.id),
            ASTNode::TryStatement(n) => Some(n.id),
            ASTNode::TryCatchClause(n) => Some(n.id),
            ASTNode::TupleExpression(n) => Some(n.id),
            ASTNode::UnaryOperation(n) => Some(n.id),
            ASTNode::UserDefinedTypeName(n) => Some(n.id),
            ASTNode::UserDefinedValueTypeDefinition(n) => Some(n.id),
            ASTNode::UsingForDirective(n) => Some(n.id),
            ASTNode::VariableDeclaration(n) => Some(n.id),
            ASTNode::VariableDeclarationStatement(n) => Some(n.id),
            ASTNode::WhileStatement(n) => Some(n.id),
            ASTNode::DoWhileStatement(n) => Some(n.id),
            ASTNode::Break(n) => Some(n.id),
            ASTNode::Continue(n) => Some(n.id),
            ASTNode::PlaceholderStatement(n) => Some(n.id),
        }
    }

    pub fn src(&self) -> Option<&str> {
        match self {
            ASTNode::ArrayTypeName(node) => Some(&node.src),
            ASTNode::Assignment(node) => Some(&node.src),
            ASTNode::BinaryOperation(node) => Some(&node.src),
            ASTNode::Block(node) => Some(&node.src),
            ASTNode::Conditional(node) => Some(&node.src),
            ASTNode::ContractDefinition(node) => Some(&node.src),
            ASTNode::ElementaryTypeName(node) => Some(&node.src),
            ASTNode::ElementaryTypeNameExpression(node) => Some(&node.src),
            ASTNode::EmitStatement(node) => Some(&node.src),
            ASTNode::EnumDefinition(node) => Some(&node.src),
            ASTNode::EnumValue(node) => Some(&node.src),
            ASTNode::EventDefinition(node) => Some(&node.src),
            ASTNode::ErrorDefinition(node) => Some(&node.src),
            ASTNode::ExpressionStatement(node) => Some(&node.src),
            ASTNode::FunctionCall(node) => Some(&node.src),
            ASTNode::FunctionCallOptions(node) => Some(&node.src),
            ASTNode::FunctionDefinition(node) => Some(&node.src),
            ASTNode::FunctionTypeName(node) => Some(&node.src),
            ASTNode::ForStatement(node) => Some(&node.src),
            ASTNode::Identifier(node) => Some(&node.src),
            ASTNode::IdentifierPath(node) => Some(&node.src),
            ASTNode::IfStatement(node) => Some(&node.src),
            ASTNode::ImportDirective(node) => Some(&node.src),
            ASTNode::IndexAccess(node) => Some(&node.src),
            ASTNode::IndexRangeAccess(node) => Some(&node.src),
            ASTNode::InheritanceSpecifier(node) => Some(&node.src),
            ASTNode::InlineAssembly(node) => Some(&node.src),
            ASTNode::Literal(node) => Some(&node.src),
            ASTNode::MemberAccess(node) => Some(&node.src),
            ASTNode::NewExpression(node) => Some(&node.src),
            ASTNode::Mapping(node) => Some(&node.src),
            ASTNode::ModifierDefinition(node) => Some(&node.src),
            ASTNode::ModifierInvocation(node) => Some(&node.src),
            ASTNode::OverrideSpecifier(node) => Some(&node.src),
            ASTNode::ParameterList(node) => Some(&node.src),
            ASTNode::PragmaDirective(node) => Some(&node.src),
            ASTNode::Return(node) => Some(&node.src),
            ASTNode::RevertStatement(node) => Some(&node.src),
            ASTNode::SourceUnit(_) => None,
            ASTNode::StructDefinition(node) => Some(&node.src),
            ASTNode::StructuredDocumentation(node) => Some(&node.src),
            ASTNode::TryStatement(node) => Some(&node.src),
            ASTNode::TryCatchClause(node) => Some(&node.src),
            ASTNode::TupleExpression(node) => Some(&node.src),
            ASTNode::UnaryOperation(node) => Some(&node.src),
            ASTNode::UserDefinedTypeName(node) => Some(&node.src),
            ASTNode::UserDefinedValueTypeDefinition(node) => Some(&node.src),
            ASTNode::UsingForDirective(node) => Some(&node.src),
            ASTNode::VariableDeclaration(node) => Some(&node.src),
            ASTNode::VariableDeclarationStatement(node) => Some(&node.src),
            ASTNode::WhileStatement(node) => Some(&node.src),
            ASTNode::DoWhileStatement(node) => Some(&node.src),
            ASTNode::Break(node) => Some(&node.src),
            ASTNode::Continue(node) => Some(&node.src),
            ASTNode::PlaceholderStatement(node) => Some(&node.src),
        }
    }
}
impl Node for ASTNode {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
        match self {
            ASTNode::ArrayTypeName(n) => n.accept(visitor),
            ASTNode::Assignment(n) => n.accept(visitor),
            ASTNode::BinaryOperation(n) => n.accept(visitor),
            ASTNode::Block(n) => n.accept(visitor),
            ASTNode::Conditional(n) => n.accept(visitor),
            ASTNode::ContractDefinition(n) => n.accept(visitor),
            ASTNode::ElementaryTypeName(n) => n.accept(visitor),
            ASTNode::ElementaryTypeNameExpression(n) => n.accept(visitor),
            ASTNode::EmitStatement(n) => n.accept(visitor),
            ASTNode::EnumDefinition(n) => n.accept(visitor),
            ASTNode::EnumValue(n) => n.accept(visitor),
            ASTNode::EventDefinition(n) => n.accept(visitor),
            ASTNode::ErrorDefinition(n) => n.accept(visitor),
            ASTNode::ExpressionStatement(n) => n.accept(visitor),
            ASTNode::FunctionCall(n) => n.accept(visitor),
            ASTNode::FunctionCallOptions(n) => n.accept(visitor),
            ASTNode::FunctionDefinition(n) => n.accept(visitor),
            ASTNode::FunctionTypeName(n) => n.accept(visitor),
            ASTNode::ForStatement(n) => n.accept(visitor),
            ASTNode::Identifier(n) => n.accept(visitor),
            ASTNode::IdentifierPath(n) => n.accept(visitor),
            ASTNode::IfStatement(n) => n.accept(visitor),
            ASTNode::ImportDirective(n) => n.accept(visitor),
            ASTNode::IndexAccess(n) => n.accept(visitor),
            ASTNode::IndexRangeAccess(n) => n.accept(visitor),
            ASTNode::InheritanceSpecifier(n) => n.accept(visitor),
            ASTNode::InlineAssembly(n) => n.accept(visitor),
            ASTNode::Literal(n) => n.accept(visitor),
            ASTNode::MemberAccess(n) => n.accept(visitor),
            ASTNode::NewExpression(n) => n.accept(visitor),
            ASTNode::Mapping(n) => n.accept(visitor),
            ASTNode::ModifierDefinition(n) => n.accept(visitor),
            ASTNode::ModifierInvocation(n) => n.accept(visitor),
            ASTNode::OverrideSpecifier(n) => n.accept(visitor),
            ASTNode::ParameterList(n) => n.accept(visitor),
            ASTNode::PragmaDirective(n) => n.accept(visitor),
            ASTNode::Return(n) => n.accept(visitor),
            ASTNode::RevertStatement(n) => n.accept(visitor),
            ASTNode::SourceUnit(n) => n.accept(visitor),
            ASTNode::StructDefinition(n) => n.accept(visitor),
            ASTNode::StructuredDocumentation(n) => n.accept(visitor),
            ASTNode::TryStatement(n) => n.accept(visitor),
            ASTNode::TryCatchClause(n) => n.accept(visitor),
            ASTNode::TupleExpression(n) => n.accept(visitor),
            ASTNode::UnaryOperation(n) => n.accept(visitor),
            ASTNode::UserDefinedTypeName(n) => n.accept(visitor),
            ASTNode::UserDefinedValueTypeDefinition(n) => n.accept(visitor),
            ASTNode::UsingForDirective(n) => n.accept(visitor),
            ASTNode::VariableDeclaration(n) => n.accept(visitor),
            ASTNode::VariableDeclarationStatement(n) => n.accept(visitor),
            ASTNode::WhileStatement(n) => n.accept(visitor),
            ASTNode::DoWhileStatement(n) => n.accept(visitor),
            ASTNode::Break(n) => n.accept(visitor),
            ASTNode::Continue(n) => n.accept(visitor),
            ASTNode::PlaceholderStatement(n) => n.accept(visitor),
        }
    }

    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> eyre::Result<()> {
        match self {
            ASTNode::ArrayTypeName(n) => n.accept_metadata(visitor),
            ASTNode::Assignment(n) => n.accept_metadata(visitor),
            ASTNode::BinaryOperation(n) => n.accept_metadata(visitor),
            ASTNode::Block(n) => n.accept_metadata(visitor),
            ASTNode::Conditional(n) => n.accept_metadata(visitor),
            ASTNode::ContractDefinition(n) => n.accept_metadata(visitor),
            ASTNode::ElementaryTypeName(n) => n.accept_metadata(visitor),
            ASTNode::ElementaryTypeNameExpression(n) => n.accept_metadata(visitor),
            ASTNode::EmitStatement(n) => n.accept_metadata(visitor),
            ASTNode::EnumDefinition(n) => n.accept_metadata(visitor),
            ASTNode::EnumValue(n) => n.accept_metadata(visitor),
            ASTNode::EventDefinition(n) => n.accept_metadata(visitor),
            ASTNode::ErrorDefinition(n) => n.accept_metadata(visitor),
            ASTNode::ExpressionStatement(n) => n.accept_metadata(visitor),
            ASTNode::FunctionCall(n) => n.accept_metadata(visitor),
            ASTNode::FunctionCallOptions(n) => n.accept_metadata(visitor),
            ASTNode::FunctionDefinition(n) => n.accept_metadata(visitor),
            ASTNode::FunctionTypeName(n) => n.accept_metadata(visitor),
            ASTNode::ForStatement(n) => n.accept_metadata(visitor),
            ASTNode::Identifier(n) => n.accept_metadata(visitor),
            ASTNode::IdentifierPath(n) => n.accept_metadata(visitor),
            ASTNode::IfStatement(n) => n.accept_metadata(visitor),
            ASTNode::ImportDirective(n) => n.accept_metadata(visitor),
            ASTNode::IndexAccess(n) => n.accept_metadata(visitor),
            ASTNode::IndexRangeAccess(n) => n.accept_metadata(visitor),
            ASTNode::InheritanceSpecifier(n) => n.accept_metadata(visitor),
            ASTNode::InlineAssembly(n) => n.accept_metadata(visitor),
            ASTNode::Literal(n) => n.accept_metadata(visitor),
            ASTNode::MemberAccess(n) => n.accept_metadata(visitor),
            ASTNode::NewExpression(n) => n.accept_metadata(visitor),
            ASTNode::Mapping(n) => n.accept_metadata(visitor),
            ASTNode::ModifierDefinition(n) => n.accept_metadata(visitor),
            ASTNode::ModifierInvocation(n) => n.accept_metadata(visitor),
            ASTNode::OverrideSpecifier(n) => n.accept_metadata(visitor),
            ASTNode::ParameterList(n) => n.accept_metadata(visitor),
            ASTNode::PragmaDirective(n) => n.accept_metadata(visitor),
            ASTNode::Return(n) => n.accept_metadata(visitor),
            ASTNode::RevertStatement(n) => n.accept_metadata(visitor),
            ASTNode::SourceUnit(n) => n.accept_metadata(visitor),
            ASTNode::StructDefinition(n) => n.accept_metadata(visitor),
            ASTNode::StructuredDocumentation(n) => n.accept_metadata(visitor),
            ASTNode::TryStatement(n) => n.accept_metadata(visitor),
            ASTNode::TryCatchClause(n) => n.accept_metadata(visitor),
            ASTNode::TupleExpression(n) => n.accept_metadata(visitor),
            ASTNode::UnaryOperation(n) => n.accept_metadata(visitor),
            ASTNode::UserDefinedTypeName(n) => n.accept_metadata(visitor),
            ASTNode::UserDefinedValueTypeDefinition(n) => n.accept_metadata(visitor),
            ASTNode::UsingForDirective(n) => n.accept_metadata(visitor),
            ASTNode::VariableDeclaration(n) => n.accept_metadata(visitor),
            ASTNode::VariableDeclarationStatement(n) => n.accept_metadata(visitor),
            ASTNode::WhileStatement(n) => n.accept_metadata(visitor),
            ASTNode::DoWhileStatement(n) => n.accept_metadata(visitor),
            ASTNode::Break(n) => n.accept_metadata(visitor),
            ASTNode::Continue(n) => n.accept_metadata(visitor),
            ASTNode::PlaceholderStatement(n) => n.accept_metadata(visitor),
        }
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.id())?;
        Ok(())
    }
}

generate_ast_methods!(
    ArrayTypeName,
    Assignment,
    BinaryOperation,
    Block,
    Conditional,
    ContractDefinition,
    ElementaryTypeName,
    ElementaryTypeNameExpression,
    EmitStatement,
    EnumDefinition,
    EnumValue,
    EventDefinition,
    ErrorDefinition,
    ExpressionStatement,
    FunctionCall,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionTypeName,
    ForStatement,
    Identifier,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    IndexAccess,
    IndexRangeAccess,
    InheritanceSpecifier,
    InlineAssembly,
    Literal,
    MemberAccess,
    NewExpression,
    Mapping,
    ModifierDefinition,
    ModifierInvocation,
    OverrideSpecifier,
    ParameterList,
    PragmaDirective,
    Return,
    RevertStatement,
    SourceUnit,
    StructDefinition,
    StructuredDocumentation,
    TryStatement,
    TryCatchClause,
    TupleExpression,
    UnaryOperation,
    UserDefinedTypeName,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration,
    VariableDeclarationStatement,
    WhileStatement,
    DoWhileStatement,
    Break,
    Continue,
    PlaceholderStatement,
);
