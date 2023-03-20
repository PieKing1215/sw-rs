//! Most of the ser/de code

use std::collections::BTreeMap;

use fakemap::FakeMap;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::microcontroller::{
    components::{BridgeComponent, Component},
    types::Type,
    util::serde_utils::{PositionXZ, RecursiveStringMap},
};

use super::is_default;

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "microprocessor")]
pub(crate) struct MicrocontrollerSerDe {
    #[serde(rename = "@name", default, skip_serializing_if = "is_default")]
    pub name: String,
    #[serde(rename = "@description", default, skip_serializing_if = "is_default")]
    pub description: String,
    #[serde(rename = "@width")]
    pub width: u8,
    #[serde(rename = "@length")]
    pub length: u8,
    #[serde(rename = "@id_counter", default, skip_serializing_if = "is_default")]
    pub id_counter: u32,
    #[serde(
        rename = "@id_counter_node",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id_counter_node: Option<u32>,

    #[serde(rename = "@sym0", default, skip_serializing_if = "is_default")]
    pub sym0: u16,
    #[serde(rename = "@sym1", default, skip_serializing_if = "is_default")]
    pub sym1: u16,
    #[serde(rename = "@sym2", default, skip_serializing_if = "is_default")]
    pub sym2: u16,
    #[serde(rename = "@sym3", default, skip_serializing_if = "is_default")]
    pub sym3: u16,
    #[serde(rename = "@sym4", default, skip_serializing_if = "is_default")]
    pub sym4: u16,
    #[serde(rename = "@sym5", default, skip_serializing_if = "is_default")]
    pub sym5: u16,
    #[serde(rename = "@sym6", default, skip_serializing_if = "is_default")]
    pub sym6: u16,
    #[serde(rename = "@sym7", default, skip_serializing_if = "is_default")]
    pub sym7: u16,
    #[serde(rename = "@sym8", default, skip_serializing_if = "is_default")]
    pub sym8: u16,
    #[serde(rename = "@sym9", default, skip_serializing_if = "is_default")]
    pub sym9: u16,
    #[serde(rename = "@sym10", default, skip_serializing_if = "is_default")]
    pub sym10: u16,
    #[serde(rename = "@sym11", default, skip_serializing_if = "is_default")]
    pub sym11: u16,
    #[serde(rename = "@sym12", default, skip_serializing_if = "is_default")]
    pub sym12: u16,
    #[serde(rename = "@sym13", default, skip_serializing_if = "is_default")]
    pub sym13: u16,
    #[serde(rename = "@sym14", default, skip_serializing_if = "is_default")]
    pub sym14: u16,
    #[serde(rename = "@sym15", default, skip_serializing_if = "is_default")]
    pub sym15: u16,

    pub nodes: Nodes,
    pub group: Group,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename = "nodes")]
pub(crate) struct Nodes {
    #[serde(rename = "n", default)]
    pub nodes: Vec<IONodeSerDe>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename = "n")]
pub(crate) struct IONodeSerDe {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@component_id")]
    pub component_id: u32,

    pub node: IONodeInner,
}

/// [`Input`][`IONodeType::Input`] or [`Output`][`IONodeType::Output`]
#[derive(Serialize_repr, Deserialize_repr, Default, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum IONodeType {
    #[default]
    Output,
    Input,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename = "node")]
pub(crate) struct IONodeInner {
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@mode", default, skip_serializing_if = "is_default")]
    pub mode: IONodeType, // 1 = input, 0 = output
    #[serde(rename = "@type", default, skip_serializing_if = "is_default")]
    pub typ: Type, // on/off, number, composite, video, audio
    #[serde(rename = "@description")]
    pub description: String,

    #[serde(default, skip_serializing_if = "is_default")]
    pub position: PositionXZ,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "group")]
pub(crate) struct Group {
    pub data: Data,
    pub components: Components,
    pub components_bridge: ComponentsBridge,
    pub groups: (), // unused?
    #[serde(
        serialize_with = "ser_component_states",
        deserialize_with = "de_component_states"
    )]
    pub component_states: Vec<ComponentsBridgeInnerObject>,
    #[serde(
        serialize_with = "ser_component_states",
        deserialize_with = "de_component_states"
    )]
    pub component_bridge_states: Vec<ComponentsBridgeInnerObject>,
    pub group_states: (), // unused?
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "data")]
pub(crate) struct Data {
    #[serde(rename = "@type", default, skip_serializing_if = "is_default")]
    pub typ: Option<String>, // ??

    pub inputs: (),  // unused?
    pub outputs: (), // unused?
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "components")]
pub(crate) struct Components {
    #[serde(
        rename = "c",
        default,
        deserialize_with = "crate::microcontroller::components::components_deserialize",
        serialize_with = "crate::microcontroller::components::components_serialize"
    )]
    pub components: Vec<Component>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "components_bridge")]
pub(crate) struct ComponentsBridge {
    #[serde(
        rename = "c",
        default,
        deserialize_with = "crate::microcontroller::components::bridge_components_deserialize",
        serialize_with = "crate::microcontroller::components::bridge_components_serialize"
    )]
    pub components_bridge: Vec<BridgeComponent>,
}

#[derive(Serialize_repr, Deserialize_repr, Default, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub(crate) enum ComponentsBridgeType {
    #[default]
    OnOffIn,
    OnOffOut,
    NumberIn,
    NumberOut,
    CompositeIn,
    CompositeOut,
    VideoIn,
    VideoOut,
    AudioIn,
    AudioOut,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub(crate) struct ComponentsBridgeInnerObject {
    #[serde(rename = "@id", default, skip_serializing_if = "is_default")]
    pub id: u32,

    #[serde(flatten)]
    pub(crate) other: FakeMap<String, RecursiveStringMap>,
}

/// Serializes Vec into tags with names c0, c1, c2, etc.
fn ser_component_states<S, T: Serialize>(states: &[T], ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ser.collect_map(states.iter().enumerate().map(|(i, v)| (format!("c{i}"), v)))
}

fn de_component_states<'de, D, T: Deserialize<'de> + std::fmt::Debug>(
    de: D,
) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    BTreeMap::<String, T>::deserialize(de).map(|m| m.into_values().collect())
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "components_bridge")]
pub(crate) struct ComponentBridgeStates {
    pub components_bridge: Vec<ComponentsBridgeInnerObject>,
}
