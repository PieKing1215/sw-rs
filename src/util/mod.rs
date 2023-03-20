//! Module containing some utility functions/types

use std::path::PathBuf;

use crate::microcontroller::components::{BridgeComponent, Component, ComponentConnection};

use self::serde_utils::PositionXY;

pub(crate) mod fakemap_hack;
pub mod serde_utils;

/// Finds the path of the user's microcontroller data folder.
///
/// It should be at `%appdata%/Stormworks/data/microprocessors/`.
///
/// # Errors
///
/// Will return an [`Err`] if the path cannot be found.
pub fn find_microcontroller_folder() -> Result<PathBuf, &'static str> {
    if let Some(data_dir) = dirs::data_dir() {
        let mcs = data_dir.join(
            "Stormworks/data/microprocessors/".replace('/', &std::path::MAIN_SEPARATOR.to_string()),
        );
        if mcs.exists() {
            Ok(mcs)
        } else {
            Err("Could not find folder at %appdata%/Stormworks/data/microprocessors/, please specify full path to microprocessors folder.")
        }
    } else {
        Err("Could not find %appdata%, please specify full path to microprocessors folder.")
    }
}

/// Wrapper around a [`Component`] or [`BridgeComponent`] reference.
#[allow(missing_docs)]
pub enum AnyComponentRef<'a> {
    Component(&'a Component),
    BridgeComponent(&'a BridgeComponent),
}

#[allow(missing_docs)]
impl AnyComponentRef<'_> {
    #[allow(clippy::must_use_candidate)]
    pub fn id(&self) -> u32 {
        match self {
            AnyComponentRef::Component(c) => c.id,
            AnyComponentRef::BridgeComponent(bc) => bc.id,
        }
    }

    #[allow(clippy::must_use_candidate)]
    pub fn pos(&self) -> &PositionXY {
        match self {
            AnyComponentRef::Component(c) => &c.pos,
            AnyComponentRef::BridgeComponent(bc) => &bc.pos,
        }
    }

    #[must_use]
    pub fn inputs(&self) -> Vec<&Option<ComponentConnection>> {
        match self {
            AnyComponentRef::Component(c) => c.component.inputs(),
            AnyComponentRef::BridgeComponent(bc) => bc.component.inputs(),
        }
    }
}

/// Wrapper around a [`Component`] or [`BridgeComponent`] mutable reference.
#[allow(missing_docs)]
pub enum AnyComponentMut<'a> {
    Component(&'a mut Component),
    BridgeComponent(&'a mut BridgeComponent),
}

#[allow(missing_docs)]
impl<'a> AnyComponentMut<'a> {
    #[allow(clippy::must_use_candidate)]
    pub fn id(&self) -> u32 {
        match self {
            AnyComponentMut::Component(c) => c.id,
            AnyComponentMut::BridgeComponent(bc) => bc.id,
        }
    }

    #[allow(clippy::must_use_candidate)]
    pub fn pos(&self) -> &PositionXY {
        match self {
            AnyComponentMut::Component(c) => &c.pos,
            AnyComponentMut::BridgeComponent(bc) => &bc.pos,
        }
    }

    #[allow(clippy::must_use_candidate)]
    pub fn pos_mut(&mut self) -> &PositionXY {
        match self {
            AnyComponentMut::Component(c) => &mut c.pos,
            AnyComponentMut::BridgeComponent(bc) => &mut bc.pos,
        }
    }

    #[must_use]
    pub fn inputs(&self) -> Vec<&Option<ComponentConnection>> {
        match self {
            AnyComponentMut::Component(c) => c.component.inputs(),
            AnyComponentMut::BridgeComponent(bc) => bc.component.inputs(),
        }
    }

    #[must_use]
    pub fn into_inputs_mut(self) -> Vec<&'a mut Option<ComponentConnection>> {
        match self {
            AnyComponentMut::Component(c) => c.component.inputs_mut(),
            AnyComponentMut::BridgeComponent(bc) => bc.component.inputs_mut(),
        }
    }
}
