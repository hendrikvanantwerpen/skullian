use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EdgeLabel {
    DefinedBy,
    IsImplementationOf,
    IsChildOf,
    Includes,
    UsesType,
    AccessField,
    MethodCall,
    NestedTo
}

impl EdgeLabel {
    pub fn to_string(&self) -> String {
        match self {
            Self::DefinedBy => "definedBy".to_string(),
            Self::IsImplementationOf => "isImplementationOf".to_string(),
            Self::IsChildOf => "isChildOf".to_string(),
            Self::Includes => "includes".to_string(),
            Self::UsesType => "usesType".to_string(),
            Self::AccessField => "accessField".to_string(),
            Self::MethodCall => "methodCall".to_string(),
            Self::NestedTo => "nestedTo".to_string()
        }
    }
}

impl Display for EdgeLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}