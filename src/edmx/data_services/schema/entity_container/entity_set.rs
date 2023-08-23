use crate::sap_annotations::default_sap_content_version;
use crate::utils::{de_str_to_bool, default_true};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// EntitySet
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntitySet {
    pub name: String,
    pub entity_type: String,

    #[serde(rename = "sap:content-version", default = "default_sap_content_version")]
    pub sap_content_version: String,

    #[serde(
        rename = "sap:creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_creatable: bool,

    #[serde(
        rename = "sap:deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_deletable: bool,

    #[serde(
        rename = "sap:updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_updatable: bool,

    #[serde(
        rename = "sap:pageable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub sap_pageable: bool,
}

impl EntitySet {
    pub fn to_enum_entry(&self) -> &[u8] {
        if let Some(idx) = self.entity_type.find('.') {
            let (_prefix, enum_entry) = self.entity_type.split_at(idx);
            enum_entry.as_bytes()
        } else {
            self.entity_type.as_bytes()
        }
    }
}
