mod dependent;
mod principal;

use dependent::Dependent;
use principal::Principal;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// ReferentialConstraint
//
// Child Nodes:
//   1:1 Principal
//   1:1 Dependent
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub struct ReferentialConstraint {
    pub principal: Principal,
    pub dependent: Dependent,
}
