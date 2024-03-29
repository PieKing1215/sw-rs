//! Module containing some utility types for ser/de

use fakemap::FakeMap;
use serde::{Deserialize, Serialize};

use crate::microcontroller::{components::de_from_str, mc_serde::is_default};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub(crate) enum RecursiveStringMap {
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

/// A 2D f32 position that (de)serializes to/from "x" and "y".
#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct PositionXY {
    /// X position.
    #[serde(
        rename = "@x",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str"
    )]
    pub x: f32,
    /// Y position.
    #[serde(
        rename = "@y",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str"
    )]
    pub y: f32,
}

impl From<PositionXZ> for PositionXY {
    fn from(xz: PositionXZ) -> Self {
        Self { x: xz.x, y: xz.z }
    }
}

/// A 2D f32 position that (de)serializes to/from "x" and "z".
#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct PositionXZ {
    /// X position.
    #[serde(rename = "@x", default, skip_serializing_if = "is_default")]
    pub x: f32,
    /// Z position.
    #[serde(rename = "@z", default, skip_serializing_if = "is_default")]
    pub z: f32,
}

impl From<PositionXY> for PositionXZ {
    fn from(xy: PositionXY) -> Self {
        Self { x: xy.x, z: xy.y }
    }
}

impl From<PositionXY> for RecursiveStringMap {
    fn from(val: PositionXY) -> Self {
        let mut m = FakeMap::new();
        if val.x != 0.0 {
            m.insert("@x".into(), RecursiveStringMap::String(val.x.to_string()));
        }
        if val.y != 0.0 {
            m.insert("@y".into(), RecursiveStringMap::String(val.y.to_string()));
        }
        RecursiveStringMap::Map(m)
    }
}

/// A 2D i32 position that (de)serializes to/from "x", "y", and "z".
#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct Vector3I {
    /// X position.
    #[serde(
        rename = "@x",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str"
    )]
    pub x: i32,
    /// Y position.
    #[serde(
        rename = "@y",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str"
    )]
    pub y: i32,
    /// Z position.
    #[serde(
        rename = "@z",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str"
    )]
    pub z: i32,
}

/// A 2D f32 position that (de)serializes to/from "x", "y", and "z".
#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct Vector3F {
    /// X position.
    #[serde(
        rename = "@x",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str"
    )]
    pub x: f32,
    /// Y position.
    #[serde(
        rename = "@y",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str"
    )]
    pub y: f32,
    /// Z position.
    #[serde(
        rename = "@z",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str"
    )]
    pub z: f32,
}

/// A 2D f32 position that (de)serializes to/from "x", "y", and "z".
#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct Vector3FPadded {
    /// X position.
    #[serde(
        rename = "@x",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str",
        serialize_with = "ser_f32"
    )]
    pub x: f32,
    /// Y position.
    #[serde(
        rename = "@y",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str",
        serialize_with = "ser_f32"
    )]
    pub y: f32,
    /// Z position.
    #[serde(
        rename = "@z",
        default,
        skip_serializing_if = "is_default",
        deserialize_with = "de_from_str",
        serialize_with = "ser_f32"
    )]
    pub z: f32,
}

/// A 2D i32 position that (de)serializes to/from "x", "y", and "z".
#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct Vector3INoDefault {
    /// X position.
    #[serde(rename = "@x", deserialize_with = "de_from_str")]
    pub x: i32,
    /// Y position.
    #[serde(rename = "@y", deserialize_with = "de_from_str")]
    pub y: i32,
    /// Z position.
    #[serde(rename = "@z", deserialize_with = "de_from_str")]
    pub z: i32,
}

/// A 2D f32 position that (de)serializes to/from "x", "y", and "z".
#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct Vector3FNoDefault {
    /// X position.
    #[serde(rename = "@x", deserialize_with = "de_from_str")]
    pub x: f32,
    /// Y position.
    #[serde(rename = "@y", deserialize_with = "de_from_str")]
    pub y: f32,
    /// Z position.
    #[serde(rename = "@z", deserialize_with = "de_from_str")]
    pub z: f32,
}

/// A 2D f32 position that (de)serializes to/from "x", "y", and "z".
#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
#[serde(rename = "node")]
pub struct Vector3FPaddedNoDefault {
    /// X position.
    #[serde(
        rename = "@x",
        deserialize_with = "de_from_str",
        serialize_with = "ser_f32"
    )]
    pub x: f32,
    /// Y position.
    #[serde(
        rename = "@y",
        deserialize_with = "de_from_str",
        serialize_with = "ser_f32"
    )]
    pub y: f32,
    /// Z position.
    #[serde(
        rename = "@z",
        deserialize_with = "de_from_str",
        serialize_with = "ser_f32"
    )]
    pub z: f32,
}

fn ser_f32<S>(n: &f32, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    format!("{n:.6}").serialize(ser)
}
