use serde::{Deserialize, Serialize};

use crate::component::instance::ComponentInstance;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Body<C: Default + PartialEq = ()> {
    #[serde(rename = "@unique_id")]
    pub unique_id: u32,
    pub components: Components<C>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename = "components")]
pub struct Components<C: Default + PartialEq = ()> {
    #[serde(rename = "c", default)]
    pub nodes: Vec<ComponentInstance<C>>,
}
