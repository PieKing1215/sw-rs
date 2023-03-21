use serde::{Deserialize, Serialize};

use crate::component::instance::ComponentInstance;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Body {
    #[serde(rename = "@unique_id")]
    pub unique_id: u32,
    pub components: Components,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename = "components")]
pub struct Components {
    #[serde(rename = "c", default)]
    pub nodes: Vec<ComponentInstance>,
}
