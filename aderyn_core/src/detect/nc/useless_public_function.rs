use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};

use crate::{
    ast::{FunctionKind, NodeID, Visibility},
    capture,
    context::workspace_context::WorkspaceContext,
    detect::detector::{Detector, DetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct UselessPublicFunctionDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl Detector for UselessPublicFunctionDetector {
    fn detect(
        &mut self,
        context: &WorkspaceContext,
        _: &[NodeID],
        _: &[NodeID],
    ) -> Result<bool, Box<dyn Error>> {
        // Collect the ids of all functions referenced by identifiers.
        let referenced_functions: HashSet<_> = context
            .identifiers
            .keys()
            .map(|i| i.referenced_declaration)
            .collect();

        let function_definitions = context.function_definitions.keys();

        // Collect all public FunctionDefinitions which are not in the referenced set.
        let unreferenced_public_functions = function_definitions
            .filter(|f| {
                f.visibility == Visibility::Public
                    && f.kind != FunctionKind::Constructor
                    && !referenced_functions.contains(&f.id)
            })
            .collect::<Vec<_>>();

        for unreferenced_public_function in unreferenced_public_functions {
            capture!(self, context, unreferenced_public_function);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Functions not used internally could be marked external")
    }

    fn description(&self) -> String {
        String::from("")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::NC
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("{}", DetectorNamePool::UselessPublicFunction)
    }
}

#[cfg(test)]
mod useless_public_function_tests {
    use crate::detect::detector::{detector_test_helpers::load_contract, Detector};

    use super::UselessPublicFunctionDetector;

    #[test]
    fn test_useless_public_functions() {
        let context =
            load_contract("../tests/contract-playground/out/Counter.sol/Counter.0.8.21.json");

        let mut detector = UselessPublicFunctionDetector::default();
        // assert that the detector finds the public function
        let found = detector.detect(&context, &[], &[]).unwrap();
        assert!(found);
        // assert that the detector returns the correct number of instances
        assert_eq!(detector.instances().len(), 1);
        // assert that the detector returns the correct severity
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::NC
        );
        // assert that the detector returns the correct title
        assert_eq!(
            detector.title(),
            String::from("Functions not used internally could be marked external")
        );
        // assert that the detector returns the correct description
        assert_eq!(detector.description(), String::from(""));
    }
}
