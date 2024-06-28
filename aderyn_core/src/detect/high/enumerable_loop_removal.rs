use std::collections::BTreeMap;
use std::error::Error;

use crate::ast::{NodeID, NodeType, TypeName};

use crate::capture;
use crate::context::browser::ExtractMemberAccesses;
use crate::detect::detector::IssueDetectorNamePool;
use crate::{
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};
use eyre::Result;

// HOW TO USE THIS TEMPLATE:
// 1. Copy this file and rename it to the snake_case version of the issue you are detecting.
// 2. Rename the EnumerableLoopRemovalDetector struct and impl to your new issue name.
// 3. Add this file and detector struct to the mod.rs file in the same directory.
// 4. Implement the detect function to find instances of the issue.

#[derive(Default)]
pub struct EnumerableLoopRemovalDetector {
    // Keys are: [0] source file name, [1] line number, [2] character location of node.
    // Do not add items manually, use `capture!` to add nodes to this BTreeMap.
    found_instances: BTreeMap<(String, usize, String), NodeID>,
}

impl IssueDetector for EnumerableLoopRemovalDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Find MemberAccesses with name `remove` and typeDescriptions.typeString.contains(EnumerableSet)
        // for each one
        // Find the closest ancestor of a loop
        // if it exists, extract all `at` member accesses on the enumerableset
        // If the at memberaccess is before the remove memberaccess, add to found_instances

        context
            .member_accesses()
            .into_iter()
            .for_each(|member_access| {
                if member_access.member_name == "remove" {
                    if let Some(type_string) = &member_access.type_descriptions.type_string {
                        if type_string.contains("EnumerableSet") {
                            let parent_loop = context
                                .get_closest_ancestor(member_access.id, NodeType::ForStatement);
                            if let Some(parent_loop) = parent_loop {
                                ExtractMemberAccesses::from(parent_loop)
                                    .extracted
                                    .into_iter()
                                    .for_each(|at_member_access| {
                                        if at_member_access.member_name == "at" {
                                            capture!(self, context, member_access);
                                        }
                                    });
                            }
                        }
                    }
                }
            });

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("High Issue Title")
    }

    fn description(&self) -> String {
        String::from("Description of the high issue.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn name(&self) -> String {
        format!("high-issue-template")
    }
}

#[cfg(test)]
mod enuemrable_loop_removal_tests {
    use crate::detect::{detector::IssueDetector, high::EnumerableLoopRemovalDetector};

    #[test]
    fn test_enumerable_loop_detector() {
        let context = crate::detect::test_utils::load_solidity_source_unit(
            "../tests/contract-playground/src/EnumerableSetIteration.sol",
        );

        let mut detector = EnumerableLoopRemovalDetector::default();
        let found = detector.detect(&context).unwrap();
        // assert that the detector found an issue
        assert!(found);
        // assert that the detector found the correct number of instances
        assert_eq!(detector.instances().len(), 3);
        // assert the severity is high
        assert_eq!(
            detector.severity(),
            crate::detect::detector::IssueSeverity::High
        );
        // assert the title is correct
        assert_eq!(detector.title(), String::from("High Issue Title"));
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from("Description of the high issue.")
        );
    }
}
