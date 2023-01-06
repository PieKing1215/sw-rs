//! Module containing things related to the game's types

use serde::{Deserialize, Serialize};

/// An enum representing the types of data available in the game.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Type {
    /// On/Off (bool) value.
    OnOff,
    /// Number (float) value.
    Number,
    /// Composite value.
    Composite,
    /// Video value.
    Video,
    /// Audio value.
    Audio,
}

/// Trait that represents a [`Type`] at compile-time.
///
/// Used in ser/de code to do different things depending on IO Type.
///
/// See also [`TypedInputConnection`][crate::components::TypedInputConnection]
pub trait CompileType {
    /// Gets the underlying [`Type`].
    fn get_type() -> Type;
}

/// [`CompileType`] for [`Type::Number`].
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TNumber;

impl CompileType for TNumber {
    fn get_type() -> Type {
        Type::Number
    }
}

/// [`CompileType`] for [`Type::OnOff`].
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TOnOff;

impl CompileType for TOnOff {
    fn get_type() -> Type {
        Type::OnOff
    }
}

/// [`CompileType`] for [`Type::Composite`].
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TComposite;

impl CompileType for TComposite {
    fn get_type() -> Type {
        Type::Composite
    }
}

/// [`CompileType`] for [`Type::Video`].
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TVideo;

impl CompileType for TVideo {
    fn get_type() -> Type {
        Type::Video
    }
}

/// [`CompileType`] for [`Type::Audio`].
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TAudio;

impl CompileType for TAudio {
    fn get_type() -> Type {
        Type::Audio
    }
}
