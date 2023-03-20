//! Module containing things related to the game's types

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// An enum representing the types of data available in the game.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
pub enum Type {
    /// On/Off (bool) value.
    #[default]
    OnOff = 0,
    /// Number (float) value.
    Number = 1,
    /// This type is not normally available ingame and may cause issues.
    _Power = 2,
    /// This type is not normally available ingame and may cause issues.
    _Fluid = 3,
    /// This type is not normally available ingame and may cause issues.
    _Electric = 4,
    /// Composite value.
    Composite = 5,
    /// Video value.
    Video = 6,
    /// Audio value.
    Audio = 7,
    /// This type is not normally available ingame and may cause issues.
    _Rope = 8,
}

/// Trait that represents a [`Type`] at compile-time.
///
/// Used in ser/de code to do different things depending on IO Type.
///
/// See also [`TypedInputConnection`][super::components::TypedInputConnection]
pub trait CompileType {
    /// Gets the underlying [`Type`].
    fn get_type() -> Type;
}

/// [`CompileType`] for [`Type::Number`].
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct TNumber;

impl CompileType for TNumber {
    fn get_type() -> Type {
        Type::Number
    }
}

/// [`CompileType`] for [`Type::OnOff`].
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct TOnOff;

impl CompileType for TOnOff {
    fn get_type() -> Type {
        Type::OnOff
    }
}

/// [`CompileType`] for [`Type::Composite`].
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct TComposite;

impl CompileType for TComposite {
    fn get_type() -> Type {
        Type::Composite
    }
}

/// [`CompileType`] for [`Type::Video`].
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct TVideo;

impl CompileType for TVideo {
    fn get_type() -> Type {
        Type::Video
    }
}

/// [`CompileType`] for [`Type::Audio`].
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct TAudio;

impl CompileType for TAudio {
    fn get_type() -> Type {
        Type::Audio
    }
}
