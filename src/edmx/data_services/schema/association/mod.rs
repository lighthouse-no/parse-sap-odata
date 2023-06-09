mod referential_constraint;

use referential_constraint::ReferentialConstraint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct End {
    pub role: Option<String>,
    pub entity_set: Option<String>,

    #[serde(rename = "Type")]
    pub entity_type: Option<String>,

    pub multiplicity: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Association
//
// Child Nodes:
//   2:2 End
//   0:n ReferentialConstraint
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Association {
    pub name: String,

    #[serde(rename = "End")]
    pub ends: [End; 2],

    pub referential_constraint: Option<ReferentialConstraint>,
}
