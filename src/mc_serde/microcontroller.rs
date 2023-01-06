use std::{collections::BTreeMap, str::FromStr};

use fakemap::FakeMap;
use serde::{Deserialize, Serialize};

use crate::components::Component;

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
pub struct Nodes {
    #[serde(rename = "n", default)]
    pub nodes: Vec<IONode>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename = "n")]
pub struct IONode {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@component_id")]
    pub component_id: u32,

    pub node: IONodeInner,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename = "node")]
pub struct IONodeInner {
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@mode", default, skip_serializing_if = "is_default")]
    pub mode: u8, // 1 = input, 0 = output
    #[serde(rename = "@type", default, skip_serializing_if = "is_default")]
    pub typ: u8, // on/off, number, composite, video, audio
    #[serde(rename = "@description")]
    pub description: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionXZ>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct PositionXY {
    #[serde(rename = "@x", default, skip_serializing_if = "is_default")]
    pub x: f32,
    #[serde(rename = "@y", default, skip_serializing_if = "is_default")]
    pub y: f32,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct PositionXZ {
    #[serde(rename = "@x", default, skip_serializing_if = "is_default")]
    pub x: f32,
    #[serde(rename = "@z", default, skip_serializing_if = "is_default")]
    pub z: f32,
}

impl From<PositionXY> for RecursiveStringMap {
    fn from(val: PositionXY) -> Self {
        let mut m = FakeMap::new();
        m.insert("@x".into(), RecursiveStringMap::String(val.x.to_string()));
        m.insert("@y".into(), RecursiveStringMap::String(val.y.to_string()));
        RecursiveStringMap::Map(m)
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "group")]
pub struct Group {
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
pub struct Data {
    #[serde(rename = "@type", default, skip_serializing_if = "is_default")]
    pub typ: Option<String>, // ??

    pub inputs: (),  // unused?
    pub outputs: (), // unused?
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "components")]
pub struct Components {
    #[serde(
        rename = "c",
        default,
        deserialize_with = "crate::components::components_deserialize",
        serialize_with = "crate::components::components_serialize"
    )]
    pub components: Vec<Component>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "components_bridge")]
pub struct ComponentsBridge {
    #[serde(rename = "c", default)]
    pub components_bridge: Vec<ComponentsBridgeInner>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ComponentsBridgeInner {
    #[serde(rename = "@type", default, skip_serializing_if = "is_default")]
    pub typ: u8,

    pub object: ComponentsBridgeInnerObject,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum RecursiveStringMap {
    String(String),
    Map(FakeMap<String, RecursiveStringMap>),
}

impl Default for RecursiveStringMap {
    fn default() -> Self {
        Self::Map(FakeMap::new())
    }
}

impl RecursiveStringMap {
    #[must_use]
    pub fn into_map(self) -> Option<FakeMap<String, RecursiveStringMap>> {
        match self {
            RecursiveStringMap::Map(m) => Some(m),
            RecursiveStringMap::String(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ComponentsBridgeInnerObject {
    #[serde(rename = "@id", default, skip_serializing_if = "is_default")]
    pub id: u32,

    #[serde(flatten)]
    pub other: FakeMap<String, RecursiveStringMap>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ComponentsBridgeInnerObjectIO {
    #[serde(
        rename = "@component_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_str_to_opt_parse"
    )]
    component_id: Option<u32>,
}

fn de_str_to_opt_parse<'de, D, T: Deserialize<'de> + FromStr + std::fmt::Debug>(
    de: D,
) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    <T as FromStr>::Err: std::fmt::Debug,
{
    Option::<String>::deserialize(de).map(|m| m.map(|s| s.parse().unwrap()))
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
pub struct ComponentBridgeStates {
    pub components_bridge: Vec<ComponentsBridgeInnerObject>,
}
