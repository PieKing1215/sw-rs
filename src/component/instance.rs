use crate::microcontroller::mc_serde::is_default;
use serde::{Deserialize, Serialize};

use crate::util::serde_utils::{RecursiveStringMap, Vector3I};

fn default_definition() -> String {
    "01_block".into()
}

fn is_default_definition(v: &String) -> bool {
    v == &default_definition()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentInstance {
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
    object: Object,
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
    pub base_color_2: Color,

    #[serde(rename = "@bc3", default, skip_serializing_if = "is_default")]
    pub base_color_3: Color,

    #[serde(rename = "@ac", default, skip_serializing_if = "is_default")]
    pub additive_color: Color,

    #[serde(rename = "@sc")]
    pub sc: String,
    // logic_slots contains as many <slot>s as there are voxels in the def (?)
    // logic_slots: Vec<>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct Color(u32);

impl Default for Color {
    fn default() -> Self {
        Self(0xFFFFFFFF)
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
