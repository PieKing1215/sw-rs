//! A WIP library for working with Stormworks microcontrollers.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

pub mod components;
pub mod mc_serde;
pub mod types;
pub mod util;

use components::Component;
use mc_serde::microcontroller::{ComponentsBridgeInner, MicrocontrollerSerDe, Nodes};
use serde::{Deserialize, Serialize};

/// High level representation of a microcontroller.
///
/// Can be (de)serialized from XML using [`Microcontroller::from_xml_string()`] and [`Microcontroller::to_xml_string()`].
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "MicrocontrollerSerDe", into = "MicrocontrollerSerDe")]
pub struct Microcontroller {
    /// The name of the microcontroller
    pub name: String,
    /// The description of the microcontroller
    ///
    /// Default is `"No description set."`
    pub description: String,
    /// The width of the microcontroller
    ///
    /// Can be `1..=6` Default is `2`
    pub width: u8,
    /// The width of the microcontroller
    ///
    /// Can be `1..=6` Default is `2`
    pub length: u8,

    id_counter: u32,
    id_counter_node: Option<u32>,

    /// 16x16 binary microcontroller icon.
    pub icon: [u16; 16],

    data_type: Option<String>,

    /// Definition of IO nodes
    ///
    /// Subject to change
    pub nodes: Nodes, // TODO: make non-serde type

    /// The main components (IO nodes are in [`components_bridge`][`Self::components_bridge`])
    pub components: Vec<Component>,
    /// The IO node components
    ///
    /// Subject to change
    pub components_bridge: Vec<ComponentsBridgeInner>, // TODO: make non-serde type
}

impl Microcontroller {
    /// # Errors
    /// Returns an `Err(quick_xml::DeError)` if the serialization failed.<br>
    /// If this happens, I consider it a bug in the library, please report it.
    pub fn to_xml_string(&self) -> Result<String, quick_xml::DeError> {
        let mut se = quick_xml::se::Serializer::new(String::new());
        se.indent('\t', 1);
        se.escape(quick_xml::se::QuoteLevel::Partial);
        let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
        self.serialize(se).map(|s| format!("{header}\n{s}\n\n"))
    }

    /// # Errors
    /// Returns an `Err(quick_xml::DeError)` if the deserialization failed.<br>
    /// If this happens, I consider it a bug in the library, please report it.
    pub fn from_xml_str(xml: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(xml)
    }

    /// Creates a new blank Microcontroller with the given name, description, and size.
    #[must_use]
    pub fn new(name: String, description: String, width: u8, length: u8) -> Self {
        Self {
            name,
            description,
            width,
            length,
            nodes: Nodes::default(),
            id_counter: 0,
            id_counter_node: None,
            icon: [0; 16],
            data_type: None,
            components: Vec::new(),
            components_bridge: Vec::new(),
        }
    }
}

impl Default for Microcontroller {
    fn default() -> Self {
        Self::new(
            "New microcontroller".into(),
            "No description set.".into(),
            2,
            2,
        )
    }
}
