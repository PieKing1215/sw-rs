pub mod components;
pub mod mc_serde;
pub mod types;
pub mod util;

use components::Component;
use mc_serde::microcontroller::{ComponentsBridgeInner, MicrocontrollerSerDe, Nodes};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "MicrocontrollerSerDe", into = "MicrocontrollerSerDe")]
pub struct Microcontroller {
    pub name: String,
    pub description: String,
    pub width: u8,
    pub length: u8,
    id_counter: u32,
    id_counter_node: Option<u32>,
    pub icon: [u16; 16],

    _data_type: Option<String>,

    pub nodes: Nodes, // TODO: make non-serde type

    pub components: Vec<Component>,
    pub components_bridge: Vec<ComponentsBridgeInner>, // TODO: make non-serde type
}

impl Microcontroller {
    pub fn to_microcontroller_xml(&self) -> Result<String, quick_xml::DeError> {
        let mut se = quick_xml::se::Serializer::new(String::new());
        se.indent('\t', 1);
        se.escape(quick_xml::se::QuoteLevel::Partial);
        let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
        self.serialize(se).map(|s| format!("{header}\n{s}"))
    }
}
