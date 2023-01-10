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

use components::{
    BridgeComponent, BridgeComponentType, Component, ComponentConnection, ComponentType,
    TypedInputConnection, TypedOutputConnection,
};
use mc_serde::microcontroller::{IONodeType, MicrocontrollerSerDe};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use types::Type;
use util::{serde_utils::PositionXY, AnyComponentMut, AnyComponentRef};

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
    /// Needs to be private so we can manage ids
    io: Vec<IONode>,
    /// Vec of component_id.
    ///
    /// Needed because the order of components isn't necessarily the same as the order of IO nodes.
    components_bridge_order: Vec<u32>,

    /// The main components (IO nodes are in [`io`][`Self::io`]).
    ///
    /// Needs to be private so we can manage ids
    components: Vec<Component>,
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
                .any(|c| *c == ion.logic.id)
            {
                return Err(MCValidationError::MissingIONodeComponentOrder(ion.logic.id));
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
            if !unique.insert(c.id) {
                return Err(MCValidationError::DuplicateComponentId(c.id));
            }

            // check component ids aren't higher than max
            if c.id > self.id_counter {
                return Err(MCValidationError::NodeIdTooHigh {
                    found_id: c.id,
                    max: self.id_counter,
                });
            }
        }

        Ok(())
    }

    /// Access the list of [`IONode`]s.
    ///
    /// The actual list is kept private so that the [`Microcontroller`] has full control over ids.
    #[allow(clippy::must_use_candidate)]
    pub fn io_nodes(&self) -> &[IONode] {
        &self.io
    }

    /// Mutably access the list of [`IONode`]s.
    ///
    /// The actual list is kept private so that the [`Microcontroller`] has full control over ids.
    pub fn io_nodes_mut(&mut self) -> &mut [IONode] {
        &mut self.io
    }

    /// Adds a new [`IONode`] with the given properties and returns a mutable reference to it.
    pub fn add_io(
        &mut self,
        label: Option<String>,
        description: Option<String>,
        typ: Type,
        mode: IONodeType,
    ) -> &mut IONode {
        let id_counter_node = self.id_counter_node.get_or_insert(0);
        *id_counter_node += 1;
        let node_id = *id_counter_node;

        self.id_counter += 1;
        let component_id = self.id_counter;

        self.io.push(IONode {
            design: IONodeDesign {
                node_id,
                label: label.unwrap_or_else(|| "Input".into()),
                description: description
                    .unwrap_or_else(|| "The input signal to be processed.".into()),
                typ,
                mode,
                position: PositionXY { x: 0.0, y: 0.0 },
            },
            logic: {
                #[allow(clippy::wildcard_in_or_patterns)]
                BridgeComponent {
                    id: component_id,
                    pos: PositionXY::default(),
                    component: match (typ, mode) {
                        (Type::OnOff, IONodeType::Input) => BridgeComponentType::OnOffIn {
                            unused_input: TypedInputConnection::default(),
                            output: TypedOutputConnection::default(),
                        },
                        (Type::Composite, IONodeType::Input) => BridgeComponentType::CompositeIn {
                            unused_input: TypedInputConnection::default(),
                            output: TypedOutputConnection::default(),
                        },
                        (Type::Video, IONodeType::Input) => BridgeComponentType::VideoIn {
                            unused_input: TypedInputConnection::default(),
                            output: TypedOutputConnection::default(),
                        },
                        (Type::Audio, IONodeType::Input) => BridgeComponentType::AudioIn {
                            unused_input: TypedInputConnection::default(),
                            output: TypedOutputConnection::default(),
                        },
                        (Type::Number | _, IONodeType::Input) => BridgeComponentType::NumberIn {
                            unused_input: TypedInputConnection::default(),
                            output: TypedOutputConnection::default(),
                        },
                        (Type::OnOff, IONodeType::Output) => BridgeComponentType::OnOffOut {
                            input: TypedInputConnection::default(),
                            unused_output: TypedOutputConnection::default(),
                        },
                        (Type::Composite, IONodeType::Output) => {
                            BridgeComponentType::CompositeOut {
                                input: TypedInputConnection::default(),
                                unused_output: TypedOutputConnection::default(),
                            }
                        },
                        (Type::Video, IONodeType::Output) => BridgeComponentType::VideoOut {
                            input: TypedInputConnection::default(),
                            unused_output: TypedOutputConnection::default(),
                        },
                        (Type::Audio, IONodeType::Output) => BridgeComponentType::AudioOut {
                            input: TypedInputConnection::default(),
                            unused_output: TypedOutputConnection::default(),
                        },
                        (Type::Number | _, IONodeType::Output) => BridgeComponentType::NumberOut {
                            input: TypedInputConnection::default(),
                            unused_output: TypedOutputConnection::default(),
                        },
                    },
                }
            },
        });

        self.components_bridge_order.push(component_id);

        // cannot panic since we just added an element
        let l = self.io.len();
        &mut self.io[l - 1]
    }

    /// Removes the [`IONode`] at the given index.
    pub fn remove_io(&mut self, index: usize) {
        if let Some(node_id) = self.io.get(index).map(|ion| ion.design.node_id) {
            self.remove_io_id(node_id);
        }
    }

    /// Removes the [`IONode`] with the given id.
    pub fn remove_io_id(&mut self, id: u32) {
        let ion = self.io.iter().position(|ion| ion.design.node_id == id);
        if let Some(ion) = ion {
            let ion = self.io.remove(ion);
            if let Some(id_counter_node) = self.id_counter_node.as_mut() {
                if *id_counter_node == ion.design.node_id {
                    *id_counter_node -= 1;
                }
            }

            self.remove_component_id(ion.logic.id);
        }
    }

    /// Access the list of [`Component`]s.
    ///
    /// The actual list is kept private so that the [`Microcontroller`] has full control over ids.
    #[must_use]
    pub fn components(&self) -> Box<dyn Iterator<Item = AnyComponentRef> + '_> {
        Box::new(
            self.components
                .iter()
                .map(AnyComponentRef::Component)
                .chain(
                    self.io
                        .iter()
                        .map(|ion| AnyComponentRef::BridgeComponent(&ion.logic)),
                ),
        )
    }

    /// Mutably access the list of [`Component`]s.
    ///
    /// The actual list is kept private so that the [`Microcontroller`] has full control over ids.
    #[must_use]
    pub fn components_mut(&mut self) -> Box<dyn Iterator<Item = AnyComponentMut> + '_> {
        Box::new(
            self.components
                .iter_mut()
                .map(AnyComponentMut::Component)
                .chain(
                    self.io
                        .iter_mut()
                        .map(|ion| AnyComponentMut::BridgeComponent(&mut ion.logic)),
                ),
        )
    }

    /// Find a [`Component`] by its id.
    #[allow(clippy::must_use_candidate)]
    pub fn get_component(&self, id: u32) -> Option<AnyComponentRef> {
        self.components().find(|c| c.id() == id)
    }

    /// Find a [`Component`] by its id.
    #[allow(clippy::must_use_candidate)]
    pub fn get_component_mut(&mut self, id: u32) -> Option<AnyComponentMut> {
        self.components_mut().find(|c| c.id() == id)
    }

    /// Find a [`Component`] by its id.
    #[allow(clippy::must_use_candidate)]
    pub fn get_connection_mut<'a>(
        &'a mut self,
        src: &ComponentConnection,
    ) -> Option<&mut Option<ComponentConnection>> {
        let c = self
            .get_component_mut(src.component_id)
            .map(AnyComponentMut::into_inputs_mut);

        if let Some(mut c) = c {
            if (src.node_index as usize) < c.len() {
                let cc = c.remove(src.node_index as usize);
                Some(cc)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Adds a new [`Component`] with the given properties and returns a mutable reference to it.
    pub fn add_component(&mut self, component: ComponentType) -> &mut Component {
        self.id_counter += 1;
        let component_id = self.id_counter;

        self.components.push(Component {
            id: component_id,
            pos: PositionXY::default(),
            component,
        });

        // cannot panic since we just added an element
        let l = self.components.len();
        &mut self.components[l - 1]
    }

    /// Removes the [`Component`] at the given index.
    pub fn remove_component(&mut self, index: usize) -> Option<ComponentType> {
        if let Some(component_id) = self.components.get(index).map(|c| c.id) {
            self.remove_component_id(component_id)
        } else {
            None
        }
    }

    /// Removes the [`Component`] with the given id.
    pub fn remove_component_id(&mut self, id: u32) -> Option<ComponentType> {
        let c = self.components.iter().position(|c| c.id == id);
        if let Some(cidx) = c {
            let c = self.components.remove(cidx);
            if self.id_counter == c.id {
                self.id_counter -= 1;
            }
            Some(c.component)
        } else {
            None
        }
    }

    /// Connects two [`ComponentConnection`]s together, if possible.
    ///
    /// # Errors
    /// Returns an [`Err`] if the connection could not be made.
    // TODO: better return type
    #[allow(clippy::result_unit_err)]
    pub fn connect(
        &mut self,
        src: &ComponentConnection,
        dst: &ComponentConnection,
    ) -> Result<(), ()> {
        // TODO: valiate modes/types
        if let Some(dst) = self.get_connection_mut(dst) {
            *dst = Some(src.clone());
            Ok(())
        } else {
            Err(())
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

impl IONode {
    /// Gets the node id of this [`IONode`].
    #[allow(clippy::must_use_candidate)]
    pub fn get_id(&self) -> u32 {
        self.design.node_id
    }
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
