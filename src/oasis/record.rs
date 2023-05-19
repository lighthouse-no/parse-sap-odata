use super::property_value::PropertyValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    #[serde(rename = "PropertyValue")]
    pub record: Option<Vec<PropertyValue>>,
}
