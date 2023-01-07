//! A WIP library for working with Stormworks microcontrollers.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::expect_fun_call)]
#![warn(missing_docs)]

pub mod components;
pub mod mc_serde;
pub mod types;
pub mod util;

use components::{BridgeComponent, Component};
use mc_serde::microcontroller::{IONodeType, MicrocontrollerSerDe};
use serde::{Deserialize, Serialize};
use types::Type;
use util::serde_utils::PositionXY;

/// High level representation of a microcontroller.
///
/// Can be (de)serialized from XML using [`Microcontroller::from_xml_string()`] and [`Microcontroller::to_xml_string()`].
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "MicrocontrollerSerDe", into = "MicrocontrollerSerDe")]
pub struct Microcontroller {
    /// The name of the microcontroller.
    pub name: String,
    /// The description of the microcontroller.
    ///
    /// Default is `"No description set."`
    pub description: String,
    /// The width of the microcontroller.
    ///
    /// Can be `1..=6` Default is `2`.
    pub width: u8,
    /// The width of the microcontroller.
    ///
    /// Can be `1..=6` Default is `2`.
    pub length: u8,

    /// The highest id currently used.
    id_counter: u32,
    /// The highest node(IO) id currently used.
    id_counter_node: Option<u32>,

    /// 16x16 binary microcontroller icon.
    pub icon: [u16; 16],

    data_type: Option<String>,

    /// Definition of IO nodes.
    ///
    /// Subject to change.
    pub io: Vec<IONode>,
    /// Vec of component_id.
    ///
    /// Needed because the order of components isn't necessarily the same as the order of IO nodes.
    components_bridge_order: Vec<u32>,

    /// The main components (IO nodes are in [`io`][`Self::io`]).
    pub components: Vec<Component>,
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
            io: Vec::new(),
            id_counter: 0,
            id_counter_node: None,
            icon: [0; 16],
            data_type: None,
            components: Vec::new(),
            components_bridge_order: Vec::new(),
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

/// Represents an input or output for this microcontroller.
#[derive(Clone, Debug)]
pub struct IONode {
    /// Design/schematic part of the node
    pub design: IONodeDesign,
    /// Logic part of the node
    pub logic: BridgeComponent,
}

/// Design/schematic part of an [`IONode`]
#[derive(Clone, Debug)]
pub struct IONodeDesign {
    /// Unique id number for this node.
    node_id: u32,

    /// The name of the node.
    ///
    /// Default is `"Input"`
    pub label: String,
    /// The description of the node.
    ///
    /// Default is `"The input signal to be processed."`.
    pub description: String,
    /// The data type for this node
    pub typ: Type,
    /// The mode for this node ([`Input`][`IONodeType::Input`] or [`Output`][`IONodeType::Output`]).
    pub mode: IONodeType,
    /// Position in the design/schematic section.
    ///
    /// 0,0 is bottom left.
    pub position: PositionXY,
}
