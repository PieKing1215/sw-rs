use std::num::ParseIntError;

use crate::microcontroller::mc_serde::is_default;
use serde::{Deserialize, Serialize};

use crate::util::serde_utils::Vector3I;

fn default_definition() -> String {
    "01_block".into()
}

fn is_default_definition(v: &String) -> bool {
    v == &default_definition()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentInstance<C: Default + PartialEq = ()> {
    #[serde(
        rename = "@d",
        default = "default_definition",
        skip_serializing_if = "is_default_definition"
    )]
    pub definition: String,
    #[serde(
        rename = "@t",
        default,
        skip_serializing_if = "is_default",
        serialize_with = "ser_flip",
        deserialize_with = "de_flip"
    )]
    pub flip: Flip,
    #[serde(rename = "o")]
    pub object: Object,

    #[serde(
        default,
        skip_serializing_if = "is_default",
    )]
    pub custom_data: C,
    // pub rotation_matrix: [i8; 9],
    // pub position: PositionIntXYZ,
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[serde(transparent)]
    pub struct Flip: u8 {
        const X = 0b00000001;
        const Y = 0b00000010;
        const Z = 0b00000100;
    }
}

fn ser_flip<S>(flip: &Flip, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    flip.bits().serialize(ser)
}

fn de_flip<'de, D>(de: D) -> Result<Flip, D::Error>
where
    D: serde::Deserializer<'de>,
{
    u8::deserialize(de).map(|n| Flip::from_bits(n).unwrap())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Object {
    #[serde(default, skip_serializing_if = "is_default")]
    pub vp: Vector3I,
    #[serde(
        rename = "@r",
        serialize_with = "ser_rotation_matrix",
        deserialize_with = "de_rotation_matrix",
        default = "default_rotation_matrix",
        skip_serializing_if = "is_default_rotation_matrix"
    )]
    pub rotation_matrix: [i8; 9],

    #[serde(rename = "@bc", default, skip_serializing_if = "is_default")]
    pub base_color: Color,

    #[serde(rename = "@bc2", default, skip_serializing_if = "is_default")]
    pub base_color_2: Option<Color>,

    #[serde(rename = "@bc3", default, skip_serializing_if = "is_default")]
    pub base_color_3: Option<Color>,

    #[serde(rename = "@ac", default, skip_serializing_if = "is_default")]
    pub additive_color: Color,

    #[serde(rename = "@sc")]
    pub sc: String,
    // logic_slots contains as many <slot>s as there are voxels in the def (?)
    // logic_slots: Vec<>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for String {
    fn from(val: Color) -> Self {
        format!("{:x}{:x}{:x}", val.r, val.g, val.b)
    }
}

impl TryFrom<String> for Color {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        u32::from_str_radix(&value, 16).map(|v| Color {
            r: ((v >> 16) & 0xff) as u8,
            g: ((v >> 8) & 0xff) as u8,
            b: (v & 0xff) as u8,
        })
    }
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 0xff, g: 0xff, b: 0xff }
    }
}

fn default_rotation_matrix() -> [i8; 9] {
    [0, 0, 1, -1, 0, 0, 0, -1, 0]
}

fn is_default_rotation_matrix(v: &[i8; 9]) -> bool {
    v == &default_rotation_matrix()
}

fn ser_rotation_matrix<S>(rotation_matrix: &[i8; 9], ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ser.serialize_str(rotation_matrix.map(|i| i.to_string()).join(",").as_str())
}

fn de_rotation_matrix<'de, D>(de: D) -> Result<[i8; 9], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let str = String::deserialize(de)?;
    let s = str
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    Ok(s.try_into().unwrap())
}
