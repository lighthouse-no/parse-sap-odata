pub mod association_set;
pub mod entity_set;
pub mod function_import;

use crate::ms_annotations::MSAnnotationsEntityType;
use association_set::AssociationSet;
use serde::{Deserialize, Serialize};

use entity_set::EntitySet;
use function_import::FunctionImport;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// EntityContainer
//
// Child Nodes:
//   1:n EntitySet
//   1:n AssociationSet
//   0:n FunctionImport
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityContainer {
    pub name: String,

    #[serde(flatten)]
    pub ms_annotations: MSAnnotationsEntityType,

    #[serde(rename = "sap:supported-formats")]
    pub sap_supported_formats: Option<String>,

    #[serde(rename = "EntitySet", default)]
    pub entity_sets: Vec<EntitySet>,

    #[serde(rename = "AssociationSet", default)]
    pub association_sets: Vec<AssociationSet>,

    #[serde(rename = "FunctionImport", default)]
    pub function_imports: Option<Vec<FunctionImport>>,
}
