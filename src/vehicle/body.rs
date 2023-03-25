use serde::{Deserialize, Serialize};

use crate::component::instance::ComponentInstance;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Body<C: Default + PartialEq = ()> {
    #[serde(rename = "@unique_id")]
    pub unique_id: u32,
    pub components: Components<C>,
}

impl<C: Default + PartialEq> Body<C> {
    pub fn clone_as_vanilla(&self) -> Body<()> {
        Body {
            unique_id: self.unique_id,
            components: self.components.clone_as_vanilla(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename = "components")]
pub struct Components<C: Default + PartialEq = ()> {
    #[serde(rename = "c", default)]
    pub nodes: Vec<ComponentInstance<C>>,
}

impl<C: Default + PartialEq> Components<C> {
    pub fn clone_as_vanilla(&self) -> Components<()> {
        Components {
            nodes: self.nodes.iter().map(|n| n.clone_as_vanilla()).collect(),
        }
    }
}
