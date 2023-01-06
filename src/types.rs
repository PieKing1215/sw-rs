use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Type {
    OnOff,
    Number,
    Composite,
    Video,
    Audio,
}

pub trait CompileType {
    fn get_type() -> Type;
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TNumber;

impl CompileType for TNumber {
    fn get_type() -> Type {
        Type::Number
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TOnOff;

impl CompileType for TOnOff {
    fn get_type() -> Type {
        Type::OnOff
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TComposite;

impl CompileType for TComposite {
    fn get_type() -> Type {
        Type::Composite
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TVideo;

impl CompileType for TVideo {
    fn get_type() -> Type {
        Type::Video
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TAudio;

impl CompileType for TAudio {
    fn get_type() -> Type {
        Type::Audio
    }
}

