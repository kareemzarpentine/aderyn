use crate::ast::{Expression, LiteralKind, NodeID};
use crate::issue_detector;
use eyre::Result;

issue_detector! {
    MisusedBooleanDetector;

    severity: High,
    title: "Misused boolean with logical operators",
    desc: "The patterns `if (… || true)` and `if (.. && false)` will always evaluate to true and false respectively.",
    name: MisusedBoolean,

    |context| {
        for binary_operation in context.binary_operations() {
            if binary_operation.operator == "||"
                && [
                    binary_operation.left_expression.as_ref(),
                    binary_operation.right_expression.as_ref(),
                ]
                .iter()
                .any(|&operand| {
                    if let Expression::Literal(literal) = operand {
                        if literal
                            .type_descriptions
                            .type_string
                            .as_ref()
                            .is_some_and(|type_string| type_string == "bool")
                        {
                            return literal.value.as_ref().is_some_and(|value| value == "true");
                        }
                    }
                    false
                })
            {
                grab!(binary_operation);
            }

            if binary_operation.operator == "&&"
                && [
                    binary_operation.left_expression.as_ref(),
                    binary_operation.right_expression.as_ref(),
                ]
                .iter()
                .any(|&operand| {
                    if let Expression::Literal(literal) = operand {
                        if literal
                            .type_descriptions
                            .type_string
                            .as_ref()
                            .is_some_and(|type_string| type_string == "bool")
                        {
                            return literal.value.as_ref().is_some_and(|value| value == "false");
                        }
                    }
                    false
                })
            {
                grab!(binary_operation);
            }
        }

        for if_statement in context.if_statements() {
            if let Expression::Literal(literal) = &if_statement.condition {
                if literal.kind == LiteralKind::Bool &&
                    literal.value.as_ref().is_some_and(|value| value == "false" || value == "true") {
                    grab!(literal);
                }
            }
        }

    }

}

#[cfg(test)]
mod misused_boolean_tests {
    use crate::detect::{detector::IssueDetector, high::misused_boolean::MisusedBooleanDetector};

    #[test]
    fn test_misused_boolean_by_loading_contract_directly() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/MisusedBoolean.sol",
        );

        let mut detector = MisusedBooleanDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 6);
    }
}
