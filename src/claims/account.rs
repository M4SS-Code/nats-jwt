use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    Data, Export, GenericFields, Import, Info, IsEmpty, Mapping, Permissions, RevocationList,
    SigningKeys,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Account {
    #[serde(flatten, default, skip_serializing_if = "IsEmpty::is_empty")]
    pub claims_data: Data,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<Import>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exports: Vec<Export>,

    #[serde(default, skip_serializing_if = "IsEmpty::is_empty")]
    pub limits: OperatorLimits,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub signing_keys: SigningKeys,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub revocations: RevocationList,

    #[serde(default, skip_serializing_if = "IsEmpty::is_empty")]
    pub default_permissions: Permissions,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub mappings: Mapping,

    #[serde(default, skip_serializing_if = "IsEmpty::is_empty")]
    pub authorization: ExternalAuthorization,

    #[serde(flatten, default, skip_serializing_if = "IsEmpty::is_empty")]
    pub info: Info,

    #[serde(flatten, default, skip_serializing_if = "IsEmpty::is_empty")]
    pub generic_fields: GenericFields,
}

// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OperatorLimits {}

impl IsEmpty for OperatorLimits {
    fn is_empty(&self) -> bool {
        todo!()
    }
}

// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExternalAuthorization {}

impl IsEmpty for ExternalAuthorization {
    fn is_empty(&self) -> bool {
        todo!()
    }
}
