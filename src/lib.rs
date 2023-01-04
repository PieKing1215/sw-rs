pub mod components;

use std::{collections::{BTreeMap, HashMap}, str::FromStr};

use serde::{Deserialize, Serialize};

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "microprocessor")]
pub struct Microcontroller {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    description: String,
    #[serde(rename = "@width")]
    width: u8,
    #[serde(rename = "@length")]
    length: u8,
    #[serde(rename = "@id_counter", default, skip_serializing_if = "is_default")]
    id_counter: u32,
    #[serde(rename = "@id_counter_node", default, skip_serializing_if = "Option::is_none")]
    id_counter_node: Option<u32>,

    nodes: Nodes,
    group: Group,
}

impl Microcontroller {
    pub fn new(name: String, description: String, width: u8, length: u8) -> Self {
        let mut s = Self {
            name,
            description,
            width,
            length,
            nodes: Default::default(),
            group: Default::default(),
            id_counter: 0,
            id_counter_node: None,
        };

        return s;
    }

    pub fn to_microcontroller_xml(&self) -> Result<String, quick_xml::DeError> {
        let mut se = quick_xml::se::Serializer::new(String::new());
        se.indent('\t', 1);
        let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
        self.serialize(se).map(|s| format!("{header}\n{s}"))
    }
}

impl Default for Microcontroller {
    fn default() -> Self {
        Self::new("New microcontroller".into(), "No description set.".into(), 2, 2)
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "nodes")]
pub struct Nodes {
    #[serde(rename = "n", default)]
    nodes: Vec<IONode>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "n")]
pub struct IONode {
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@component_id")]
    component_id: u32,

    node: IONodeInner,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "node")]
pub struct IONodeInner {
    #[serde(rename = "@label")]
    label: String,
    #[serde(rename = "@mode", default, skip_serializing_if = "is_default")]
    mode: u8, // 1 = input, 0 = output
    #[serde(rename = "@type", default, skip_serializing_if = "is_default")]
    typ: u8, // on/off, number, composite, video, audio
    #[serde(rename = "@description")]
    description: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "node")]
pub struct Position {
    #[serde(rename = "@x", default, skip_serializing_if = "is_default")]
    x: f32,
    #[serde(rename = "@y", default, skip_serializing_if = "is_default")]
    y: f32,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "group")]
pub struct Group {
    data: Data,
    components: Components,
    components_bridge: ComponentsBridge,
    groups: (), // unused?
    #[serde(
        serialize_with = "ser_component_states",
        deserialize_with = "de_component_states"
    )]
    component_states: Vec<ComponentsBridgeInnerObject>,
    #[serde(
        serialize_with = "ser_component_states",
        deserialize_with = "de_component_states"
    )]
    component_bridge_states: Vec<ComponentsBridgeInnerObject>,
    group_states: (), // unused?
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "data")]
pub struct Data {
    inputs: (),  // unused?
    outputs: (), // unused?
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "components")]
pub struct Components {
    #[serde(rename = "c", default)]
    components: Vec<ComponentsBridgeInner>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "components_bridge")]
pub struct ComponentsBridge {
    #[serde(rename = "c", default)]
    components_bridge: Vec<ComponentsBridgeInner>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ComponentsBridgeInner {
    #[serde(rename = "@type", default, skip_serializing_if = "is_default")]
    typ: u8,

    object: ComponentsBridgeInnerObject,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ComponentsBridgeInnerObject {
    #[serde(rename = "@id", default, skip_serializing_if = "is_default")]
    id: u32,
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pos: Option<Position>,

    #[serde(flatten, serialize_with = "ser_bridge_in", deserialize_with = "de_bridge_in")]
    ins: Vec<ComponentsBridgeInnerObjectIO>,

    #[serde(flatten, serialize_with = "ser_bridge_out", deserialize_with = "de_bridge_out")]
    outs: Vec<ComponentsBridgeInnerObjectIO>,
}

/// Serializes Vec into tags with names in1, in2, in3, etc.
fn ser_bridge_in<S, T: Serialize>(states: &Vec<T>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ser.collect_map(states.iter().enumerate().map(|(i, v)| (format!("in{}", i + 1), v)))
}

/// Serializes Vec into tags with names out1, out2, out3, etc.
fn ser_bridge_out<S, T: Serialize>(states: &Vec<T>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ser.collect_map(states.iter().enumerate().map(|(i, v)| (format!("out{}", i + 1), v)))
}

fn de_bridge_in<'de, D, T: Deserialize<'de>>(
    de: D,
) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    BTreeMap::<String, T>::deserialize(de).map(|m| {
        m.into_iter().filter_map(|(k, v)| if k.starts_with("in") { Some(v) } else { None }).collect()
    })
}

fn de_bridge_out<'de, D, T: Deserialize<'de>>(
    de: D,
) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    BTreeMap::<String, T>::deserialize(de).map(|m| {
        m.into_iter().filter_map(|(k, v)| if k.starts_with("out") { Some(v) } else { None }).collect()
    })
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ComponentsBridgeInnerObjectIO {
    #[serde(rename = "@component_id", default, skip_serializing_if = "Option::is_none", deserialize_with = "de_str_to_opt_parse")]
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
fn ser_component_states<S, T: Serialize>(states: &Vec<T>, ser: S) -> Result<S::Ok, S::Error>
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
    BTreeMap::<String, T>::deserialize(de).map(|m| {
        m.into_values().collect()
    })
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "components_bridge")]
pub struct ComponentBridgeStates {
    components_bridge: Vec<ComponentsBridgeInnerObject>,
}