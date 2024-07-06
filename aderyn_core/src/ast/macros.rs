#![allow(unused_imports, unused_macros)]

/// Macro that expands to a struct with common AST node fields.
macro_rules! ast_node {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub id: $crate::ast::NodeID,
            pub src: String,
            $(
                $(#[$field_meta])*
                pub $field: $ty
            ),*
        }
    };
}

macro_rules! ast_node_no_partial_eq {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, Clone, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub id: $crate::ast::NodeID,
            pub src: String,
            $(
                $(#[$field_meta])*
                pub $field: $ty
            ),*
        }
    };
}

/// A macro that expands to a struct with common expression node fields.
macro_rules! expr_node {
    (
        $(#[$struct_meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $ty:ty
            ),* $(,)*
        }
    ) => {
        ast_node!(
            $(#[$struct_meta])*
            struct $name {
                argument_types: Option<Vec<TypeDescriptions>>,
                #[serde(default)]
                is_constant: bool,
                #[serde(default)]
                is_l_value: bool,
                #[serde(default)]
                is_pure: bool,
                #[serde(default)]
                l_value_requested: bool,
                type_descriptions: TypeDescriptions,
                $(
                    $(#[$field_meta])*
                    $field: $ty
                ),*
            }
        );
    }
}

/// A macro that expands to an enum where each variant also contains a struct of the same name.
///
/// The inner value of each variant is boxed since AST types are inherently recursive.
macro_rules! node_group {
    ($group:ident; $( $name:ident ),* $(,)*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(tag = "nodeType")]
        pub enum $group {
            $(
                $name(Box<$name>),
            )*
        }
    };
}

pub(crate) use ast_node;
pub(crate) use ast_node_no_partial_eq;
pub(crate) use expr_node;
pub(crate) use node_group;
