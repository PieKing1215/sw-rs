//! A WIP library for working with Stormworks microcontrollers.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::expect_fun_call)]
#![warn(missing_docs)]

pub mod components;
pub mod mc_serde;
pub mod types;
pub mod util;

use std::collections::HashSet;

use components::{BridgeComponent, Component};
use mc_serde::microcontroller::{IONodeType, MicrocontrollerSerDe};
use serde::{Deserialize, Serialize};
use thiserror::Error;
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
    /// The length of the microcontroller.
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

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum MCSerDeError {
    #[error(transparent)]
    SerDeError(#[from] quick_xml::DeError),
    #[error(transparent)]
    ValidationError(#[from] MCValidationError),
}

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum MCValidationError {
    #[error("Invalid size {w}x{h}, max is 6x6")]
    InvalidSize { w: u8, h: u8 },
    #[error("Duplicate IONode id {0}")]
    DuplicateIONodeId(u32),
    #[error("Node id was greater than id_counter_node {found_id}/{max}")]
    NodeIdTooHigh { found_id: u32, max: u32 },
    #[error("Missing IONode component order map entry: component_id={0}")]
    MissingIONodeComponentOrder(u32),
    #[error("Duplicate Component id {0}")]
    DuplicateComponentId(u32),
    #[error("Component id was greater than id_counter {found_id}/{max}")]
    ComponentIdTooHigh { found_id: u32, max: u32 },
}

impl Microcontroller {
    /// # Errors
    /// Returns an [`Err(MCSerDeError)`] if the serialization failed, or if the microcontroller was invalid.
    pub fn to_xml_string(&self) -> Result<String, MCSerDeError> {
        self.validate()?;
        let mut se = quick_xml::se::Serializer::new(String::new());
        se.indent('\t', 1);
        se.escape(quick_xml::se::QuoteLevel::Partial);
        let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
        Ok(self.serialize(se).map(|s| format!("{header}\n{s}\n\n"))?)
    }

    /// # Errors
    /// Returns an [`Err(MCSerDeError)`] if the deserialization failed, or if the microcontroller was invalid.
    pub fn from_xml_str(xml: &str) -> Result<Self, MCSerDeError> {
        let mc: Self = quick_xml::de::from_str(xml)?;
        mc.validate()?;
        Ok(mc)
    }

    /// Creates a new blank Microcontroller with the given name, description, and size.
    ///
    /// # Errors
    /// Returns an [`Err(MCValidationError)`] if the microcontroller was invalid.
    pub fn new(
        name: String,
        description: String,
        width: u8,
        length: u8,
    ) -> Result<Self, MCValidationError> {
        let mc = Self {
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
        };
        mc.validate()?;
        Ok(mc)
    }

    /// Checks the [`Microcontroller`] for validity.
    ///
    /// Ideally, there should be no (safe) action that you can make to turn a valid [`Microcontroller`] invalid.
    ///
    /// # Errors
    /// Returns an [`Err(MCValidationError)`] if the microcontroller was invalid.
    pub fn validate(&self) -> Result<(), MCValidationError> {
        // check size in range
        if !(1..=6).contains(&self.width) || !(1..=6).contains(&self.length) {
            return Err(MCValidationError::InvalidSize { w: self.width, h: self.length });
        }

        // check io nodes
        let mut unique = HashSet::new();
        for ion in &self.io {
            // check all io node ids are unique
            if !unique.insert(ion.design.node_id) {
                return Err(MCValidationError::DuplicateIONodeId(ion.design.node_id));
            }

            // check components_bridge_order contains all io logic ids
            if !self
                .components_bridge_order
                .iter()
                .any(|c| *c == ion.logic.id())
            {
                return Err(MCValidationError::MissingIONodeComponentOrder(
                    ion.logic.id(),
                ));
            }

            // check node ids aren't higher than max
            if ion.design.node_id > self.id_counter_node.unwrap_or(0) {
                return Err(MCValidationError::NodeIdTooHigh {
                    found_id: ion.design.node_id,
                    max: self.id_counter_node.unwrap_or(0),
                });
            }
        }

        // check components
        let mut unique = HashSet::new();
        for c in &self.components {
            // check all io component ids are unique
            if !unique.insert(c.id()) {
                return Err(MCValidationError::DuplicateComponentId(c.id()));
            }

            // check component ids aren't higher than max
            if c.id() > self.id_counter {
                return Err(MCValidationError::NodeIdTooHigh {
                    found_id: c.id(),
                    max: self.id_counter,
                });
            }
        }

        Ok(())
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
        .expect("Default Microcontroller was invalid (?)")
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
