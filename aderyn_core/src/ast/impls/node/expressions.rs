use crate::ast::*;
use crate::visitor::ast_visitor::*;
use eyre::Result;

impl Node for Expression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        match self {
            Expression::Literal(literal) => literal.accept(visitor),
            Expression::Identifier(identifier) => identifier.accept(visitor),
            Expression::UnaryOperation(unary_operation) => unary_operation.accept(visitor),
            Expression::BinaryOperation(binary_operation) => binary_operation.accept(visitor),
            Expression::Conditional(conditional) => conditional.accept(visitor),
            Expression::Assignment(assignment) => assignment.accept(visitor),
            Expression::FunctionCall(function_call) => function_call.accept(visitor),
            Expression::FunctionCallOptions(function_call_options) => {
                function_call_options.accept(visitor)
            }
            Expression::IndexAccess(index_access) => index_access.accept(visitor),
            Expression::IndexRangeAccess(index_range_access) => index_range_access.accept(visitor),
            Expression::MemberAccess(member_access) => member_access.accept(visitor),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                elementary_type_name_expression.accept(visitor)
            }
            Expression::TupleExpression(tuple_expression) => tuple_expression.accept(visitor),
            Expression::NewExpression(new_expression) => new_expression.accept(visitor),
        }
    }

    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(self.get_node_id())?;
        Ok(())
    }
}

impl Expression {
    pub fn get_node_id(&self) -> Option<NodeID> {
        match self {
            Expression::Literal(literal) => Some(literal.id),
            Expression::Identifier(identifier) => Some(identifier.id),
            Expression::UnaryOperation(unary_operation) => Some(unary_operation.id),
            Expression::BinaryOperation(binary_operation) => Some(binary_operation.id),
            Expression::Conditional(conditional) => Some(conditional.id),
            Expression::Assignment(assignment) => Some(assignment.id),
            Expression::FunctionCall(function_call) => Some(function_call.id),
            Expression::FunctionCallOptions(function_call_options) => {
                Some(function_call_options.id)
            }
            Expression::IndexAccess(index_access) => Some(index_access.id),
            Expression::IndexRangeAccess(index_range_access) => Some(index_range_access.id),
            Expression::MemberAccess(member_access) => Some(member_access.id),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                Some(elementary_type_name_expression.id)
            }

            Expression::TupleExpression(tuple_expression) => Some(tuple_expression.id),
            Expression::NewExpression(new_expression) => Some(new_expression.id),
        }
    }

    pub fn root_expression(&self) -> Option<&Expression> {
        match self {
            Expression::Identifier(_) => Some(self),
            Expression::TupleExpression(_) => Some(self),
            Expression::Assignment(assignment) => assignment.left_hand_side.root_expression(),
            Expression::IndexAccess(index_access) => index_access.base_expression.root_expression(),
            Expression::IndexRangeAccess(index_range_access) => {
                index_range_access.base_expression.root_expression()
            }
            Expression::MemberAccess(_) => Some(self),
            _ => None,
        }
    }

    pub fn referenced_declarations(&self) -> Vec<NodeID> {
        let mut result = vec![];

        match self {
            Expression::Identifier(identifier) => {
                if let Some(reference_id) = identifier.referenced_declaration {
                    result.push(reference_id);
                }
            }

            Expression::Assignment(assignment) => {
                result.extend(assignment.left_hand_side.referenced_declarations());
                result.extend(assignment.right_hand_side.referenced_declarations());
            }

            Expression::IndexAccess(index_access) => {
                result.extend(index_access.base_expression.referenced_declarations());
            }

            Expression::IndexRangeAccess(index_range_access) => {
                result.extend(index_range_access.base_expression.referenced_declarations());
            }

            Expression::MemberAccess(member_access) => {
                result.extend(member_access.expression.referenced_declarations());

                if let Some(referenced_declaration) = member_access.referenced_declaration {
                    result.push(referenced_declaration);
                }
            }

            Expression::TupleExpression(tuple_expression) => {
                for component in tuple_expression.components.iter().flatten() {
                    result.extend(component.referenced_declarations());
                }
            }

            Expression::FunctionCall(function_call) => {
                result.extend(function_call.expression.referenced_declarations());

                for argument in function_call.arguments.iter() {
                    result.extend(argument.referenced_declarations());
                }
            }

            _ => {}
        }

        result
    }

    pub fn contains_operation(&self, operator: &str) -> bool {
        match self {
            Expression::UnaryOperation(unary_operation) => {
                unary_operation.contains_operation(operator)
            }
            Expression::BinaryOperation(binary_operation) => {
                binary_operation.contains_operation(operator)
            }
            Expression::Conditional(conditional) => conditional.contains_operation(operator),
            Expression::Assignment(assignment) => assignment.contains_operation(operator),
            Expression::FunctionCall(function_call) => function_call.contains_operation(operator),
            Expression::FunctionCallOptions(function_call_options) => {
                function_call_options.contains_operation(operator)
            }
            Expression::IndexAccess(index_access) => index_access.contains_operation(operator),
            Expression::IndexRangeAccess(index_range_access) => {
                index_range_access.contains_operation(operator)
            }
            Expression::MemberAccess(member_access) => member_access.contains_operation(operator),
            Expression::TupleExpression(tuple_expression) => {
                tuple_expression.contains_operation(operator)
            }
            _ => false,
        }
    }

    pub fn type_descriptions(&self) -> Option<&TypeDescriptions> {
        match self {
            Expression::Literal(Literal {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::Identifier(Identifier {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::UnaryOperation(UnaryOperation {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::BinaryOperation(BinaryOperation {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::Conditional(Conditional {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::Assignment(Assignment {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::FunctionCall(FunctionCall {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::FunctionCallOptions(FunctionCallOptions {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::IndexAccess(IndexAccess {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::IndexRangeAccess(IndexRangeAccess {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::MemberAccess(MemberAccess {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                type_descriptions,
                ..
            }) => Some(type_descriptions),
            Expression::TupleExpression(TupleExpression {
                type_descriptions, ..
            }) => Some(type_descriptions),
            Expression::NewExpression(NewExpression {
                type_descriptions, ..
            }) => Some(type_descriptions),
        }
    }

    pub fn source_line(&self, source_unit: &SourceUnit) -> Result<usize> {
        source_unit.source_line(match self {
            Expression::Literal(Literal { src, .. }) => src.as_str(),
            Expression::Identifier(Identifier { src, .. }) => src.as_str(),
            Expression::UnaryOperation(UnaryOperation { src, .. }) => src.as_str(),
            Expression::BinaryOperation(BinaryOperation { src, .. }) => src.as_str(),
            Expression::Conditional(Conditional { src, .. }) => src.as_str(),
            Expression::Assignment(Assignment { src, .. }) => src.as_str(),
            Expression::FunctionCall(FunctionCall { src, .. }) => src.as_str(),
            Expression::FunctionCallOptions(FunctionCallOptions { src, .. }) => src.as_str(),
            Expression::IndexAccess(IndexAccess { src, .. }) => src.as_str(),
            Expression::IndexRangeAccess(IndexRangeAccess { src, .. }) => src.as_str(),
            Expression::MemberAccess(MemberAccess { src, .. }) => src.as_str(),
            Expression::ElementaryTypeNameExpression(ElementaryTypeNameExpression {
                src, ..
            }) => src.as_str(),
            Expression::TupleExpression(TupleExpression { src, .. }) => src.as_str(),
            Expression::NewExpression(NewExpression { src, .. }) => src.as_str(),
        })
    }
}

impl Node for UnaryOperation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_unary_operation(self)? {
            self.sub_expression.accept(visitor)?
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_unary_operation(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(child_id) = self.sub_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![child_id])?;
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl UnaryOperation {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator || self.sub_expression.contains_operation(operator)
    }
}

impl Node for BinaryOperation {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_binary_operation(self)? {
            self.left_expression.accept(visitor)?;
            self.right_expression.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_binary_operation(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(left_node_id) = self.left_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![left_node_id])?;
        }
        if let Some(right_node) = self.right_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![right_node])?;
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl BinaryOperation {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator
            || self.left_expression.contains_operation(operator)
            || self.right_expression.contains_operation(operator)
    }
}

impl Node for Conditional {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_conditional(self)? {
            self.condition.accept(visitor)?;
            self.true_expression.accept(visitor)?;
            self.false_expression.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_conditional(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(condition_id) = self.condition.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![condition_id])?;
        }
        if let Some(true_expression_id) = self.true_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![true_expression_id])?;
        }
        if let Some(false_expression_id) = self.false_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![false_expression_id])?;
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Conditional {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.condition.contains_operation(operator)
            || self.true_expression.contains_operation(operator)
            || self.false_expression.contains_operation(operator)
    }
}

impl Node for Assignment {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_assignment(self)? {
            self.left_hand_side.accept(visitor)?;
            self.right_hand_side.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_assignment(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(left_hand_id) = self.left_hand_side.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![left_hand_id])?;
        }
        if let Some(right_hand_id) = self.right_hand_side.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![right_hand_id])?;
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Assignment {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.operator == operator || self.right_hand_side.contains_operation(operator)
    }
}

impl Node for FunctionCall {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_function_call(self)? {
            self.expression.accept(visitor)?;
            list_accept(&self.arguments, visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_function_call(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(expr_id) = self.expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![expr_id])?;
        }
        let mut argument_ids = vec![];
        for arg in &self.arguments {
            if let Some(arg_id) = arg.get_node_id() {
                argument_ids.push(arg_id);
            }
        }
        visitor.visit_immediate_children(self.id, argument_ids)?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl FunctionCall {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for argument in self.arguments.iter() {
            if argument.contains_operation(operator) {
                return true;
            }
        }

        false
    }
}

impl Node for FunctionCallOptions {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_function_call_options(self)? {
            self.expression.accept(visitor)?;
            if self.arguments.is_some() {
                list_accept(self.arguments.as_ref().unwrap(), visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_function_call_options(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(expr_id) = self.expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![expr_id])?;
        }
        let mut argument_ids = vec![];
        if let Some(arguments) = &self.arguments {
            for arg in arguments {
                if let Some(arg_id) = arg.get_node_id() {
                    argument_ids.push(arg_id);
                }
            }
            visitor.visit_immediate_children(self.id, argument_ids)?;
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl FunctionCallOptions {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for option in self.options.iter() {
            if option.contains_operation(operator) {
                return true;
            }
        }

        false
    }
}

impl Node for IndexAccess {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_index_access(self)? {
            self.base_expression.accept(visitor)?;
            if let Some(index_expression) = &self.index_expression {
                index_expression.accept(visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_index_access(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(base_expr_id) = self.base_expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![base_expr_id])?;
        }
        if let Some(index_expression) = &self.index_expression {
            if let Some(index_expr_id) = index_expression.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![index_expr_id])?;
            }
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl IndexAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        if let Some(index_expression) = &self.index_expression {
            index_expression.contains_operation(operator);
        }
        false
    }
}

impl Node for IndexRangeAccess {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_index_range_access(self)? {
            self.base_expression.accept(visitor)?;
            if self.start_expression.is_some() {
                self.start_expression.as_ref().unwrap().accept(visitor)?;
            }
            if self.end_expression.is_some() {
                self.end_expression.as_ref().unwrap().accept(visitor)?;
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_index_range_access(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(start_expression) = &self.start_expression {
            if let Some(start_expr_id) = start_expression.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![start_expr_id])?;
            }
        }
        if let Some(end_expression) = &self.end_expression {
            if let Some(end_expr_id) = end_expression.get_node_id() {
                visitor.visit_immediate_children(self.id, vec![end_expr_id])?;
            }
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl IndexRangeAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.start_expression
            .as_ref()
            .map(|expr| expr.contains_operation(operator))
            .unwrap_or(false)
            || self
                .end_expression
                .as_ref()
                .map(|expr| expr.contains_operation(operator))
                .unwrap_or(false)
    }
}

impl Node for MemberAccess {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_member_access(self)? {
            self.expression.accept(visitor)?;
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_member_access(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if let Some(expr_id) = self.expression.get_node_id() {
            visitor.visit_immediate_children(self.id, vec![expr_id])?;
        }
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl MemberAccess {
    pub fn contains_operation(&self, operator: &str) -> bool {
        self.expression.contains_operation(operator)
    }
}

impl Node for ElementaryTypeNameExpression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_elementary_type_name_expression(self)?;
        visitor.end_visit_elementary_type_name_expression(self)
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl Node for TupleExpression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_tuple_expression(self)? {
            for elem in &self.components {
                if elem.is_some() {
                    elem.as_ref().unwrap().accept(visitor)?;
                }
            }
        }
        self.accept_metadata(visitor)?;
        visitor.end_visit_tuple_expression(self)
    }
    fn accept_metadata(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        let mut comp_ids = vec![];
        for expr in self.components.iter().flatten() {
            if let Some(id) = expr.get_node_id() {
                comp_ids.push(id)
            }
        }
        visitor.visit_immediate_children(self.id, comp_ids)?;
        Ok(())
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}

impl TupleExpression {
    pub fn contains_operation(&self, operator: &str) -> bool {
        for component in self.components.iter() {
            if component
                .as_ref()
                .map(|expr| expr.contains_operation(operator))
                .unwrap_or(false)
            {
                return true;
            }
        }

        false
    }
}
impl Node for NewExpression {
    fn accept(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        if visitor.visit_new_expression(self)? {
            self.type_name.accept(visitor)?;
        }
        visitor.end_visit_new_expression(self)
    }
    fn accept_id(&self, visitor: &mut impl ASTConstVisitor) -> Result<()> {
        visitor.visit_node_id(Some(self.id))?;
        Ok(())
    }
}
