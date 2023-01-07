//! Module containing things related to microcontroller components (nodes)

use std::marker::PhantomData;
use std::num::ParseFloatError;

use crate::types::CompileType;
use crate::util::fakemap_hack::FakeMapExt;
use crate::util::serde_utils::PositionXY;
use crate::util::serde_utils::RecursiveStringMap;
use fakemap::FakeMap;
use paste::paste;
use serde::{Deserialize, Serialize};

use crate::{mc_serde::is_default, types::Type};

/// List of IO types for a component.
pub struct ComponentIODef {
    /// List of input Types.
    pub inputs: Vec<Type>,
    /// List of output Types.
    pub outputs: Vec<Type>,
}

fn skip_connection<T: CompileType, const S: bool>(v: &Option<ConnectionV>) -> bool {
    match v {
        Some(v) => {
            (*v == ConnectionV::default())
                && (T::get_type() == Type::Number || T::get_type() == Type::OnOff)
        },
        None => true,
    }
}

/// Represents a connection to another component.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ComponentConnection {
    /// The id of the component to connect to.
    #[serde(
        rename = "@component_id",
        deserialize_with = "deserialize_string_to_u32"
    )]
    pub component_id: u32,
    /// The index on the other component to connect to.
    #[serde(
        rename = "@node_index",
        deserialize_with = "deserialize_string_to_u8",
        default,
        skip_serializing_if = "is_default"
    )]
    pub node_index: u8,
}

fn deserialize_string_to_u32<'de, D>(de: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    Ok(s.parse().unwrap())
}

fn deserialize_string_to_u8<'de, D>(de: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    Ok(s.parse().unwrap())
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub(crate) struct ConnectionV {
    #[serde(rename = "@bools", default, skip_serializing_if = "is_default")]
    __bools: Option<String>, // ??
    #[serde(rename = "@01", default, skip_serializing_if = "is_default")]
    __01: Option<String>, // ??
    #[serde(rename = "@02", default, skip_serializing_if = "is_default")]
    __02: Option<String>, // ??
    #[serde(rename = "@03", default, skip_serializing_if = "is_default")]
    __03: Option<String>, // ??
    #[serde(rename = "@04", default, skip_serializing_if = "is_default")]
    __04: Option<String>, // ??
    #[serde(rename = "@05", default, skip_serializing_if = "is_default")]
    __05: Option<String>, // ??
    #[serde(rename = "@06", default, skip_serializing_if = "is_default")]
    __06: Option<String>, // ??
    #[serde(rename = "@07", default, skip_serializing_if = "is_default")]
    __07: Option<String>, // ??
    #[serde(rename = "@08", default, skip_serializing_if = "is_default")]
    __08: Option<String>, // ??
    #[serde(rename = "@09", default, skip_serializing_if = "is_default")]
    __09: Option<String>, // ??
    #[serde(rename = "@10", default, skip_serializing_if = "is_default")]
    __10: Option<String>, // ??
    #[serde(rename = "@11", default, skip_serializing_if = "is_default")]
    __11: Option<String>, // ??
    #[serde(rename = "@12", default, skip_serializing_if = "is_default")]
    __12: Option<String>, // ??
    #[serde(rename = "@13", default, skip_serializing_if = "is_default")]
    __13: Option<String>, // ??
    #[serde(rename = "@14", default, skip_serializing_if = "is_default")]
    __14: Option<String>, // ??
    #[serde(rename = "@15", default, skip_serializing_if = "is_default")]
    __15: Option<String>, // ??
    #[serde(rename = "@16", default, skip_serializing_if = "is_default")]
    __16: Option<String>, // ??
    #[serde(rename = "@17", default, skip_serializing_if = "is_default")]
    __17: Option<String>, // ??
    #[serde(rename = "@18", default, skip_serializing_if = "is_default")]
    __18: Option<String>, // ??
    #[serde(rename = "@19", default, skip_serializing_if = "is_default")]
    __19: Option<String>, // ??
    #[serde(rename = "@20", default, skip_serializing_if = "is_default")]
    __20: Option<String>, // ??
}

impl core::fmt::Debug for ConnectionV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionV")
            .field("__bools", &self.__bools)
            .finish()
    }
}

/// Represents an input connection slot.
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct TypedInputConnection<T: CompileType, const S: bool> {
    /// The actual connection.
    #[serde(flatten)]
    pub connection: Option<ComponentConnection>,

    // some extra ser/de fields
    #[serde(rename = "@v", default, skip_serializing_if = "is_default")]
    v_attr: Option<String>,
    #[serde(default, skip_serializing_if = "skip_connection::<T, S>")]
    v: Option<ConnectionV>,
    #[serde(skip, default)]
    _phantom: PhantomData<T>,
}

impl<T: CompileType, const S: bool> core::fmt::Debug for TypedInputConnection<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedInputConnection")
            .field("connection", &self.connection)
            .finish()
    }
}

impl<T: CompileType, const S: bool> TypedInputConnection<T, S> {
    /// Creates a [`TypedInputConnection`] with the given connection.
    #[must_use]
    pub fn new(component_id: u32, node_index: u8) -> Self {
        Self {
            connection: Some(ComponentConnection { component_id, node_index }),
            v_attr: None,
            v: None,
            _phantom: PhantomData,
        }
    }

    /// Creates an unconnected [`TypedInputConnection`].
    #[must_use]
    pub fn empty() -> Self {
        Self {
            connection: None,
            v_attr: None,
            v: None,
            _phantom: PhantomData,
        }
    }
}

/// Represents an output connection slot.
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct TypedOutputConnection<T: CompileType> {
    #[serde(rename = "@v", default, skip_serializing_if = "is_default")]
    v_attr: Option<String>,
    #[serde(default, skip_serializing_if = "skip_connection::<T, true>")]
    v: Option<ConnectionV>,
    #[serde(skip, default)]
    _phantom: PhantomData<T>,
}

impl<T: CompileType> core::fmt::Debug for TypedOutputConnection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedInputConnection").finish()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct _ComponentDe {
    #[serde(flatten)]
    inner: FakeMap<String, RecursiveStringMap>,
}

impl From<_ComponentDe> for Component {
    fn from(de: _ComponentDe) -> Self {
        #[derive(Serialize, Deserialize, Debug)]
        struct W {
            object: _ComponentDe,
        }

        let db = format!("{de:?}");

        let mut se = quick_xml::se::Serializer::new(String::new());
        se.escape(quick_xml::se::QuoteLevel::Partial);
        let ser = W { object: de }.serialize(se).unwrap();
        let ser = ser.trim_start_matches("<W>").trim_end_matches("</W>");

        let de: Component = quick_xml::de::from_str(ser)
            .expect(&format!("Deserializing component:\n{db}\n{ser}\n"));

        de
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct _BridgeComponentDe {
    #[serde(flatten)]
    inner: FakeMap<String, RecursiveStringMap>,
}

impl From<_BridgeComponentDe> for BridgeComponent {
    fn from(de: _BridgeComponentDe) -> Self {
        #[derive(Serialize, Deserialize, Debug)]
        struct W {
            object: _BridgeComponentDe,
        }

        let db = format!("{de:?}");

        let mut se = quick_xml::se::Serializer::new(String::new());
        se.escape(quick_xml::se::QuoteLevel::Partial);
        let ser = W { object: de }.serialize(se).unwrap();
        let ser = ser.trim_start_matches("<W>").trim_end_matches("</W>");

        let de: BridgeComponent = quick_xml::de::from_str(ser)
            .expect(&format!("Deserializing bridge component:\n{db}\n{ser}\n"));

        de
    }
}

#[allow(dead_code)]
pub(crate) fn component_deserialize<'de, D>(de: D) -> Result<Component, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut cde = _ComponentDe::deserialize(de)?;

    if cde.inner.get("@type").is_none() {
        cde.inner
            .insert("@type".into(), RecursiveStringMap::String("0".into()));
    }
    if let Some(RecursiveStringMap::String(s)) = cde.inner.get("@type") {
        // see note on NumericalJunction
        if s == "21" {
            // FakeMap has no get_mut
            if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
                o.remove("out1");
                o.insert("out2".into(), RecursiveStringMap::default());
                cde.inner
                    .insert("object".into(), RecursiveStringMap::Map(o));
            }
        } else if s == "40" || s == "41" {
            if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
                for (k, _) in o.iter_mut() {
                    if *k == "inc" {
                        *k = "in1".into();
                    } else if *k == "inoff" {
                        *k = "in34".into();
                    } else if k.starts_with("in") {
                        if let Ok(n) = k.trim_start_matches("in").parse::<u8>() {
                            *k = format!("in{}", n + 1);
                        }
                    }
                }
                cde.inner
                    .insert("object".into(), RecursiveStringMap::Map(o));
            }
        }
    }

    Ok(cde.into())
}

pub(crate) fn components_deserialize<'de, D>(de: D) -> Result<Vec<Component>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let de = Vec::<_ComponentDe>::deserialize(de)?;
    // println!("{de:?}");
    let cs = de
        .into_iter()
        .map(|mut cde| {
            if cde.inner.get("@type").is_none() {
                cde.inner
                    .insert("@type".into(), RecursiveStringMap::String("0".into()));
            }
            if let Some(RecursiveStringMap::String(s)) = cde.inner.get("@type") {
                // see note on NumericalJunction
                if s == "21" {
                    // FakeMap has no get_mut
                    if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
                        o.remove("out1");
                        o.insert("out2".into(), RecursiveStringMap::default());
                        cde.inner
                            .insert("object".into(), RecursiveStringMap::Map(o));
                    }
                } else if s == "40" || s == "41" {
                    if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
                        for (k, _) in o.iter_mut() {
                            if *k == "inc" {
                                *k = "in1".into();
                            } else if *k == "inoff" {
                                *k = "in34".into();
                            } else if k.starts_with("in") {
                                if let Ok(n) = k.trim_start_matches("in").parse::<u8>() {
                                    *k = format!("in{}", n + 1);
                                }
                            }
                        }
                        cde.inner
                            .insert("object".into(), RecursiveStringMap::Map(o));
                    }
                }
            }

            cde.into()
        })
        .collect();

    Ok(cs)
}

#[allow(dead_code)]
pub(crate) fn component_serialize<S>(
    component: &Component,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut se = quick_xml::se::Serializer::new(String::new());
    se.escape(quick_xml::se::QuoteLevel::Partial);
    let ser = component.serialize(se).unwrap();
    let ser = ser.trim_start_matches("<W>").trim_end_matches("</W>");

    let mut cde: _ComponentDe = quick_xml::de::from_str(ser).unwrap();
    if let Some(RecursiveStringMap::String(s)) = cde.inner.get("@type").cloned() {
        if s == "0" {
            cde.inner.remove("@type");
        }
    }

    // see note on NumericalJunction
    if matches!(component, Component::NumericalJunction { .. }) {
        // FakeMap has no get_mut
        if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
            o.remove("out2");
            o.duplicate_by_key("out1".into(), "__reserved".into());
            cde.inner
                .insert("object".into(), RecursiveStringMap::Map(o));
        }
    }

    // map in1,in2,in3,etc. to inc,in1,in2,etc.
    // see note on CompositeWriteNum/CompositeWriteOnOff
    if let Component::CompositeWriteNum { count, offset, .. }
    | Component::CompositeWriteOnOff { count, offset, .. } = component
    {
        if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
            for (k, _) in o.iter_mut() {
                if *k == "in1" {
                    *k = "inc".into();
                } else if *k == "in34" {
                    *k = "inoff".into();
                } else if k.starts_with("in") {
                    if let Ok(n) = k.trim_start_matches("in").parse::<u8>() {
                        *k = format!("in{}", n - 1);
                    }
                }
            }

            if *offset != -1 {
                o.remove("inoff");
            }

            for i in 1..=32 {
                if i > *count {
                    let l = format!("in{}", i);
                    o.remove(&l);
                }
            }

            cde.inner
                .insert("object".into(), RecursiveStringMap::Map(o));
        }
    }

    // remove in2 if channel is constant
    if let Component::CompositeReadNum { channel, .. }
    | Component::CompositeReadOnOff { channel, .. } = component
    {
        if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
            if *channel == -1 {
                // for some reason, in these nodes in2 is supposed to go after out1
                let in2 = o.remove("in2").unwrap();
                o.insert("in2".into(), in2);
            } else {
                o.remove("in2");
            }

            cde.inner
                .insert("object".into(), RecursiveStringMap::Map(o));
        }
    }

    cde.serialize(serializer)
}

pub(crate) fn components_serialize<S>(components: &[Component], ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let cdes = components
        .iter()
        .map(|c| {
            let mut se = quick_xml::se::Serializer::new(String::new());
            se.escape(quick_xml::se::QuoteLevel::Partial);
            let ser = c.serialize(se).unwrap();
            let ser = ser.trim_start_matches("<W>").trim_end_matches("</W>");

            let mut cde: _ComponentDe = quick_xml::de::from_str(ser).unwrap();
            if let Some(RecursiveStringMap::String(s)) = cde.inner.get("@type").cloned() {
                if s == "0" {
                    cde.inner.remove("@type");
                }
            }

            // rename out2 to out1
            // see note on NumericalJunction
            if matches!(c, Component::NumericalJunction { .. }) {
                // FakeMap has no get_mut
                if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
                    o.remove("out2");
                    o.duplicate_by_key("out1".into(), "__reserved".into());
                    cde.inner
                        .insert("object".into(), RecursiveStringMap::Map(o));
                }
            }

            // map in1,in2,in3,etc. to inc,in1,in2,etc.
            // see note on CompositeWriteNum/CompositeWriteOnOff
            if let Component::CompositeWriteNum { count, offset, .. }
            | Component::CompositeWriteOnOff { count, offset, .. } = c
            {
                if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
                    for (k, _) in o.iter_mut() {
                        if *k == "in1" {
                            *k = "inc".into();
                        } else if *k == "in34" {
                            *k = "inoff".into();
                        } else if k.starts_with("in") {
                            if let Ok(n) = k.trim_start_matches("in").parse::<u8>() {
                                *k = format!("in{}", n - 1);
                            }
                        }
                    }

                    if *offset != -1 {
                        o.remove("inoff");
                    }

                    for i in 1..=32 {
                        if i > *count {
                            let l = format!("in{}", i);
                            o.remove(&l);
                        }
                    }

                    cde.inner
                        .insert("object".into(), RecursiveStringMap::Map(o));
                }
            }

            // remove in2 if channel is constant
            if let Component::CompositeReadNum { channel, .. }
            | Component::CompositeReadOnOff { channel, .. } = c
            {
                if let Some(RecursiveStringMap::Map(mut o)) = cde.inner.remove("object") {
                    if *channel == -1 {
                        // for some reason, in these nodes in2 is supposed to go after out1
                        let in2 = o.remove("in2").unwrap();
                        o.insert("in2".into(), in2);
                    } else {
                        o.remove("in2");
                    }

                    cde.inner
                        .insert("object".into(), RecursiveStringMap::Map(o));
                }
            }

            cde
        })
        .collect::<Vec<_>>();

    ser.collect_seq(cdes.iter())
}

macro_rules! components {
    (   $type:ident,
        $(
            $id:literal = $x:ident [$($in_id:ident($idx_i:literal): $in:expr),*] [$($out_id:ident($idx_o:literal): $out:expr),*] {$($b:tt)*}
        ),*,
        {$($ser_to_map:tt)*}
    ) => {
        paste! {
            #[allow(missing_docs)]
            #[derive(Serialize, Deserialize, Clone, Debug)]
            #[serde(rename = "object", tag = "@type", content = "object")]
            pub enum $type {
                $(
                    #[serde(rename = "" $id "")]
                    $x {
                        #[serde(rename = "@id")]
                        id: u32,
                        /// The position of the component.
                        ///
                        /// Each grid square is 0.25 units.
                        #[serde(default, skip_serializing_if = "is_default")]
                        pos: PositionXY,
                        $(
                            #[serde(rename = "" [<in $idx_i>] "", default, skip_serializing_if = "is_default")]
                            $in_id: Option<TypedInputConnection<crate::types::[<T $in>], true>>,
                        )*
                        $(
                            #[serde(rename = "" [<out $idx_o>] "", default, skip_serializing_if = "is_default")]
                            $out_id: Option<TypedOutputConnection<crate::types::[<T $out>]>>,
                        )*
                        $($b)*
                    },
                )*
            }

            impl $type {
                /// Generates a [`ComponentIODef`] for this [`Component`].
                #[must_use]
                pub fn io_def(&self) -> ComponentIODef {
                    match self {
                        $(
                            Self::$x { .. } => ComponentIODef {
                                inputs: vec![$(Type::$in,)*],
                                outputs: vec![$(Type::$out,)*],
                            },
                        )*
                    }
                }

                /// Returns an immutable list of the input connections for this [`Component`].
                #[must_use]
                pub fn inputs(&self) -> Vec<Option<&ComponentConnection>> {
                    match self {
                        $(
                            Self::$x {
                                $( $in_id, )* .. } => vec![
                                $( $in_id.as_ref().and_then(|c| c.connection.as_ref()), )*
                            ],
                        )*
                    }
                }

                /// Returns a mutable list of the input connections for this [`Component`].
                #[must_use]
                pub fn inputs_mut(&mut self) -> Vec<Option<&mut ComponentConnection>> {
                    match self {
                        $(
                            Self::$x {
                                $( $in_id, )* .. } => vec![
                                $( $in_id.as_mut().and_then(|c| c.connection.as_mut()), )*
                            ],
                        )*
                    }
                }

                /// Returns the id of this [`Component`].
                #[must_use]
                pub fn id(&self) -> u32 {
                    match self {
                        $(
                            Self::$x { id, .. } => *id,
                        )*
                    }
                }

                /// Immutably borrows the position of this [`Component`].
                #[must_use]
                pub fn position(&self) -> &PositionXY {
                    match self {
                        $(
                            Self::$x { pos, .. } => pos,
                        )*
                    }
                }

                /// Mutably borrows the position of this [`Component`].
                #[must_use]
                pub fn position_mut(&mut self) -> &mut PositionXY {
                    match self {
                        $(
                            Self::$x { pos, .. } => pos,
                        )*
                    }
                }

                #[allow(dead_code)]
                #[must_use]
                pub(crate) fn ser_to_map(&self) -> FakeMap<String, RecursiveStringMap> {
                    let mut se = quick_xml::se::Serializer::new(String::new());
                    se.escape(quick_xml::se::QuoteLevel::Partial);
                    let ser = self.serialize(se).unwrap();

                    let mut de: FakeMap<String, RecursiveStringMap> = quick_xml::de::from_str(&ser).unwrap();

                    #[allow(clippy::redundant_closure_call)]
                    ($($ser_to_map)*)(&self, &mut de);

                    de
                }

            }
        }
    };
}

/// Represents a [`String`] and the [`f64`] value it parses to
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextValue {
    #[serde(rename = "@text")]
    text: String,
    #[serde(rename = "@value", default, skip_serializing_if = "is_default")]
    value: f64,
}

impl TextValue {
    /// Creates a [`TextValue`] from the given text.
    ///
    /// # Errors
    /// Will return an [`Err`] if the text fails to parse into a [`f64`].
    pub fn from_text(text: impl Into<String>) -> Result<Self, ParseFloatError> {
        let text: String = text.into();
        Ok(Self { value: text.parse()?, text })
    }

    /// Creates a [`TextValue`] from the given value.
    pub fn from_value(val: impl Into<f64>) -> Self {
        let val = val.into();
        Self { text: val.to_string(), value: val }
    }
}

/// Struct representing a dropdown item with a label and value.
///
/// Used in [`Component::PropertyDropdown`].
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "i")]
pub struct DropdownItem {
    /// The label for this item.
    #[serde(rename = "@l")]
    pub label: String,

    /// The value for this item.
    #[serde(rename = "v")]
    pub value: TextValue,
}

mod dropdown_items {
    use serde::{Deserialize, Serialize};

    use super::DropdownItem;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct DropdownItems {
        #[serde(rename = "i", default)]
        items: Vec<DropdownItem>,
    }

    pub fn deserialize<'de, D>(de: D) -> Result<Vec<DropdownItem>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let di = DropdownItems::deserialize(de)?;
        Ok(di.items)
    }

    pub fn serialize<S>(items: &[DropdownItem], ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        DropdownItems { items: items.to_vec() }.serialize(ser)
    }
}

fn one() -> f32 {
    1.0
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_one(v: &f32) -> bool {
    (*v - 1.0).abs() < f32::EPSILON
}

macro_rules! str_def_fns {
    ($val:literal) => {
        paste! {
            #[allow(non_snake_case)]
            fn [<str_ $val>]() -> String {
                $val .into()
            }

            #[allow(non_snake_case)]
            fn [<is_str_ $val>](s: &String) -> bool {
                s == $val
            }
        }
    };
    ($id:literal, $val:literal) => {
        paste! {
            #[allow(non_snake_case)]
            fn [<str_ $id>]() -> String {
                $val .into()
            }

            #[allow(non_snake_case)]
            fn [<is_str_ $id>](s: &String) -> bool {
                s == $val
            }
        }
    };
}

str_def_fns!("value");
str_def_fns!("toggle");
str_def_fns!("on");
str_def_fns!("off");
str_def_fns!("number");

components! { Component,
    0 = NOT[input(1): OnOff][out(1): OnOff]{},
    1 = AND[input_a(1): OnOff, input_b(2): OnOff][out(1): OnOff]{},
    2 = OR[input_a(1): OnOff, input_b(2): OnOff][out(1): OnOff]{},
    3 = XOR[input_a(1): OnOff, input_b(2): OnOff][out(1): OnOff]{},
    4 = NAND[input_a(1): OnOff, input_b(2): OnOff][out(1): OnOff]{},
    5 = NOR[input_a(1): OnOff, input_b(2): OnOff][out(1): OnOff]{},
    6 = Add[input_a(1): Number, input_b(2): Number][out(1): Number]{},
    7 = Subtract[input_a(1): Number, input_b(2): Number][out(1): Number]{},
    8 = Multiply[input_a(1): Number, input_b(2): Number][out(1): Number]{},
    9 = Divide[input_a(1): Number, input_b(2): Number][out(1): Number, div_by_zero(2): OnOff]{},
    10 = Func3n[x(1): Number, y(2): Number, z(3): Number][out(1): Number]{
        #[serde(rename = "@e", default, skip_serializing_if="is_default")]
        expr: String,

        #[serde(rename = "@p1", default, skip_serializing_if="is_default")]
        __p1: String, // always "340282346638528859811704183484516925440" (f32::MAX) ??
        #[serde(rename = "@p2", default, skip_serializing_if="is_default")]
        __p2: String, // always "340282346638528859811704183484516925440" (f32::MAX) ??
        #[serde(rename = "@p3", default, skip_serializing_if="is_default")]
        __p3: String, // always "340282346638528859811704183484516925440" (f32::MAX) ??
    },
    11 = Clamp[input(1): Number][out(1): Number]{
        min: TextValue,
        max: TextValue,
    },
    12 = Threshold[input(1): Number][out(1): OnOff]{
        min: TextValue,
        max: TextValue,
    },
    13 = MemoryRegister[set(1): OnOff, reset(2): OnOff, number(3): Number][out(1): Number]{
        #[serde(rename = "r")]
        reset_value: TextValue,

        #[serde(rename = "@memory", default, skip_serializing_if = "is_default")]
        __memory: Option<String>, // ??
    },
    14 = Abs[input(1): Number][out(1): Number]{},
    15 = ConstantNum[][out(1): Number]{
        n: TextValue,
    },
    16 = ConstantOn[][out(1): OnOff]{},
    17 = GreaterThan[input_a(1): Number, input_b(2): Number][out(1): OnOff]{},
    18 = LessThan[input_a(1): Number, input_b(2): Number][out(1): OnOff]{},
    19 = PropertySlider[][out(1): Number]{
        #[serde(rename = "@name", default = "str_value", skip_serializing_if = "is_str_value")]
        name: String,

        min: TextValue,
        max: TextValue,
        int: TextValue,
        v: TextValue,
    },
    20 = PropertyDropdown[][out(1): Number]{
        #[serde(rename = "@name", default = "str_value", skip_serializing_if = "is_str_value")]
        name: String,

        #[serde(with = "dropdown_items")]
        items: Vec<DropdownItem>,
    },
    // NOTE: due to a bug(?) in the game, both outputs for NumericalJunctions use the tag "out1" in the XML
    21 = NumericalJunction[pass(1): Number, switch(2): OnOff][on_path(1): Number, off_path(2): Number]{},
    22 = NumericalSwitchbox[on(1): Number, off(2): Number, switch(3): OnOff][out(1): Number]{},
    23 = PIDController[setpoint(1): Number, process_var(2): Number, active(3): OnOff][out(1): Number]{
        #[serde(rename = "@te", default, skip_serializing_if = "is_default")]
        __te: Option<String>, // ??
        #[serde(rename = "@p2", default, skip_serializing_if = "is_default")]
        __p2: Option<String>, // ??
        #[serde(rename = "@pe", default, skip_serializing_if = "is_default")]
        __pe: Option<String>, // ??
        #[serde(rename = "@pes", default, skip_serializing_if = "is_default")]
        __pes: Option<String>, // ??

        kp: TextValue,
        ki: TextValue,
        kd: TextValue,
    },
    24 = SRLatch[set(1): OnOff, reset(2): OnOff][out(1): OnOff, not_out(2): OnOff]{
        #[serde(rename = "@p1", default, skip_serializing_if = "is_default")]
        __p1: Option<String>, // ??
    },
    25 = JKFlipFlop[set(1): OnOff, reset(2): OnOff][out(1): OnOff, not_out(2): OnOff]{},
    26 = Capacitor[charge(1): OnOff][stored(1): OnOff]{
        #[serde(rename = "@ct", default = "one", skip_serializing_if="is_one")]
        ct: f32,
        #[serde(rename = "@dt", default = "one", skip_serializing_if="is_one")]
        dt: f32,

        #[serde(rename = "@c1", default, skip_serializing_if = "is_default")]
        __c1: Option<String>, // ??
        #[serde(rename = "@c2", default, skip_serializing_if = "is_default")]
        __c2: Option<String>, // ??
        #[serde(rename = "@p", default, skip_serializing_if = "is_default")]
        __p: Option<String>, // ??
    },
    27 = Blinker[control(1): OnOff][out(1): OnOff]{
        #[serde(rename = "@on", default = "one", skip_serializing_if="is_one")]
        on: f32,
        #[serde(rename = "@off", default = "one", skip_serializing_if="is_one")]
        off: f32,

        #[serde(rename = "@c", default, skip_serializing_if = "is_default")]
        __c: Option<String>, // ??
    },
    28 = PushToToggle[toggle(1): OnOff][state(1): OnOff]{},
    29 = CompositeReadOnOff[composite(1): Composite, variable_channel(2): Number][out(1): OnOff]{
        #[serde(rename = "@i", default, skip_serializing_if = "is_default")] // TODO
        channel: i8, // TODO: "variable (from node)" is -1, make this an enum
    },
    30 = _OldCompositeWriteOnOff[composite(1): Composite, val(2): OnOff][out(1): Composite]{
        #[serde(rename = "@i", default, skip_serializing_if = "is_default")] // TODO
        channel: u8, // no option for "variable (from node)"
    },
    31 = CompositeReadNum[composite(1): Composite, variable_channel(2): Number][out(1): Number]{
        #[serde(rename = "@i", default, skip_serializing_if = "is_default")] // TODO
        channel: i8, // TODO: "variable (from node)" is -1, make this an enum
    },
    32 = _OldCompositeWriteNum[composite(1): Composite, val(2): Number][out(1): Composite]{
        #[serde(rename = "@i", default, skip_serializing_if = "is_default")] // TODO
        channel: u8, // no option for "variable (from node)"
    },
    33 = PropertyToggle[][out(1): OnOff]{
        #[serde(rename = "@n", default = "str_toggle", skip_serializing_if = "is_str_toggle")]
        name: String,
        #[serde(rename = "@on", default = "str_on", skip_serializing_if = "is_str_on")]
        on: String,
        #[serde(rename = "@off", default = "str_off", skip_serializing_if = "is_str_off")]
        off: String,
        #[serde(rename = "@v", default, skip_serializing_if = "is_default")]
        value: bool,
    },
    34 = PropertyNumber[][out(1): Number]{
        #[serde(rename = "@n", default = "str_number", skip_serializing_if = "is_str_number")]
        name: String,

        #[serde(rename = "v")]
        value: TextValue,
    },
    35 = Delta[input(1): Number][out(1): Number]{
        #[serde(rename = "@vp", default, skip_serializing_if = "is_default")]
        __vp: Option<String>, // ??
        #[serde(rename = "@ip", default, skip_serializing_if = "is_default")]
        __ip: Option<String>, // ??
    },
    36 = Func8n[x(1): Number, y(2): Number, z(3): Number, w(4): Number, a(5): Number, b(6): Number, c(7): Number, d(8): Number][out(1): Number]{
        #[serde(rename = "@e", default, skip_serializing_if = "is_default")]
        expr: String,
    },
    37 = UpDownCounter[up(1): OnOff, down(2): OnOff, reset(3): OnOff][out(1): Number]{
        #[serde(rename = "@m", default, skip_serializing_if = "is_default")]
        mode: u8, // 1 for clamp, 0 for disabled

        #[serde(rename = "@is", default, skip_serializing_if = "is_default")]
        __is: Option<String>, // ??

        #[serde(rename = "r")]
        reset_val: TextValue,
        #[serde(rename = "i")]
        increment: TextValue,
        #[serde(rename = "min")]
        min: TextValue,
        #[serde(rename = "max")]
        max: TextValue,
    },
    38 = Modulo[input_a(1): Number, input_b(2): Number][out(1): Number]{},
    39 = PIDControllerAdvanced[setpoint(1): Number, process_var(2): Number, p(3): Number, i(4): Number, d(5): Number, active(6): OnOff][out(1): Number]{
        #[serde(rename = "@te", default, skip_serializing_if = "is_default")]
        __te: Option<String>, // ??
        #[serde(rename = "@p2", default, skip_serializing_if = "is_default")]
        __p2: Option<String>, // ??
        #[serde(rename = "@pe", default, skip_serializing_if = "is_default")]
        __pe: Option<String>, // ??
        #[serde(rename = "@pes", default, skip_serializing_if = "is_default")]
        __pes: Option<String>, // ??
    },
    // NOTE: CompositeWriteNum uses tags inc, in1, in2, etc.
    40 = CompositeWriteNum[composite(1): Composite, in1(2): Number, in2(3): Number, in3(4): Number, in4(5): Number, in5(6): Number, in6(7): Number, in7(8): Number, in8(9): Number, in9(10): Number, in10(11): Number, in11(12): Number, in12(13): Number, in13(14): Number, in14(15): Number, in15(16): Number, in16(17): Number, in17(18): Number, in18(19): Number, in19(20): Number, in20(21): Number, in21(22): Number, in22(23): Number, in23(24): Number, in24(25): Number, in25(26): Number, in26(27): Number, in27(28): Number, in28(29): Number, in29(30): Number, in30(31): Number, in31(32): Number, in32(33): Number, start(34): Number][out(1): Composite]{
        #[serde(rename = "@count")]
        count: u8,
        #[serde(rename = "@offset", default, skip_serializing_if = "is_default")]
        offset: i8, // TODO: "variable (from node)" is -1, make this an enum
    },
    // NOTE: CompositeWriteOnOff uses tags inc, in1, in2, etc.
    41 = CompositeWriteOnOff[composite(1): Composite, in1(2): OnOff, in2(3): OnOff, in3(4): OnOff, in4(5): OnOff, in5(6): OnOff, in6(7): OnOff, in7(8): OnOff, in8(9): OnOff, in9(10): OnOff, in10(11): OnOff, in11(12): OnOff, in12(13): OnOff, in13(14): OnOff, in14(15): OnOff, in15(16): OnOff, in16(17): OnOff, in17(18): OnOff, in18(19): OnOff, in19(20): OnOff, in20(21): OnOff, in21(22): OnOff, in22(23): OnOff, in23(24): OnOff, in24(25): OnOff, in25(26): OnOff, in26(27): OnOff, in27(28): OnOff, in28(29): OnOff, in29(30): OnOff, in30(31): OnOff, in31(32): OnOff, in32(33): OnOff, start(34): Number][out(1): Composite]{
        #[serde(rename = "@count")]
        count: u8,
        #[serde(rename = "@offset", default, skip_serializing_if = "is_default")]
        offset: i8, // TODO: "variable (from node)" is -1, make this an enum
    },
    42 = Equal[input_a(1): Number, input_b(2): Number][out(1): OnOff]{
        #[serde(rename = "e")]
        epsilon: TextValue,
    },
    43 = TooltipNum[num(1): Number, is_error(2): OnOff][]{
        #[serde(rename = "@l")]
        label: String,
        #[serde(rename = "@m", default, skip_serializing_if = "is_default")]
        mode: u8, // TODO: enum: 0 for Always, 1 for If Error, 2 for If No Error
    },
    44 = TooltipOnOff[display(1): OnOff][]{
        #[serde(rename = "@l")]
        label: String,
        #[serde(rename = "@on")]
        on: String,
        #[serde(rename = "@off")]
        off: String,
        #[serde(rename = "@m", default, skip_serializing_if = "is_default")]
        mode: u8, // TODO: enum: 0 for Always, 1 for If Error, 2 for If No Error
    },
    45 = Func1n[input(1): Number][out(1): Number]{
        #[serde(rename = "@e", default, skip_serializing_if="is_default")]
        expr: String,
    },
    46 = Func4b[x(1): OnOff, y(2): OnOff, z(3): OnOff, w(4): OnOff][out(1): OnOff]{
        #[serde(rename = "@e", default, skip_serializing_if="is_default")]
        expr: String,
    },
    47 = Func8b[x(1): OnOff, y(2): OnOff, z(3): OnOff, w(4): OnOff, a(5): OnOff, b(6): OnOff, c(7): OnOff, d(8): OnOff][out(1): OnOff]{
        #[serde(rename = "@e", default, skip_serializing_if="is_default")]
        expr: String,
    },
    48 = Pulse[input(1): OnOff][out(1): OnOff]{
        #[serde(rename = "@m", default, skip_serializing_if="is_default")]
        mode: Option<u8>, // None for Off->On, 0 for On->Off, 2 for Always

        #[serde(rename = "@p", default, skip_serializing_if = "is_default")]
        __p: Option<String>, // ??
    },
    49 = TimerTON[enable(1): OnOff, duration(2): Number][complete(1): OnOff]{
        #[serde(rename = "@u", default, skip_serializing_if="is_default")]
        units: u8, // TODO: enum: 0 for secs, 1 for ticks

        #[serde(rename = "@t", default, skip_serializing_if = "is_default")]
        __t: Option<String>, // ??
    },
    50 = TimerTOF[enable(1): OnOff, duration(2): Number][timing(1): OnOff]{
        #[serde(rename = "@u", default, skip_serializing_if="is_default")]
        units: u8, // TODO: enum: 0 for secs, 1 for ticks

        #[serde(rename = "@t", default, skip_serializing_if = "is_default")]
        __t: Option<String>, // ??
    },
    51 = TimerRTO[enable(1): OnOff, duration(2): Number, reset(3): OnOff][complete(1): OnOff]{
        #[serde(rename = "@u", default, skip_serializing_if="is_default")]
        units: u8, // TODO: enum: 0 for secs, 1 for ticks

        #[serde(rename = "@t", default, skip_serializing_if = "is_default")]
        __t: Option<String>, // ??
    },
    52 = TimerRTF[enable(1): OnOff, duration(2): Number, reset(3): OnOff][timing(1): OnOff]{
        #[serde(rename = "@u", default, skip_serializing_if="is_default")]
        units: u8, // TODO: enum: 0 for secs, 1 for ticks

        #[serde(rename = "@t", default, skip_serializing_if = "is_default")]
        __t: Option<String>, // ??
    },
    53 = CompositeSwitchbox[on(1): Composite, off(2): Composite, switch(3): OnOff][out(1): Composite]{},
    54 = NumToCompositeBin[input(1): Number][out(1): Composite]{},
    55 = CompositeBinToNum[input(1): Composite][out(1): Number]{},
    56 = Lua[data_in(1): Composite, video_in(2): Video][data_out(1): Composite, video_out(2): Video]{
        #[serde(rename = "@script", default, skip_serializing_if="is_default")]
        script: Option<String>,
    },
    57 = VideoSwitchbox[on(1): Video, off(2): Video, switch(3): OnOff][out(1): Video]{},
    58 = PropertyText[][]{
        #[serde(rename = "@n")]
        name: String,
        #[serde(rename = "@v", default, skip_serializing_if="is_default")]
        val: String,
    },
    59 = AudioSwitchbox[on(1): Audio, off(2): Audio, switch(3): OnOff][out(1): Audio]{},
    {
        |c: &Component, de: &mut FakeMap<String, RecursiveStringMap>| {
            // see note on NumericalJunction
            if matches!(c, Component::NumericalJunction { .. }) {
                // FakeMap has no get_mut
                if let Some(RecursiveStringMap::Map(mut o)) = de.remove("object") {
                    o.remove("out2");
                    o.duplicate_by_key("out1".into(), "__reserved".into());
                    de.insert("object".into(), RecursiveStringMap::Map(o));
                }
            }


            // map in1,in2,in3,etc. to inc,in1,in2,etc.
            // see note on CompositeWriteNum/CompositeWriteOnOff
            if let Component::CompositeWriteNum { count, offset, .. } | Component::CompositeWriteOnOff { count, offset, .. } = c {
                if let Some(RecursiveStringMap::Map(mut o)) = de.remove("object") {
                    for (k, _) in o.iter_mut() {
                        if *k == "in1" {
                            *k = "inc".into();
                        } else if *k == "in34" {
                            *k = "inoff".into();
                        } else if k.starts_with("in") {
                            if let Ok(n) = k.trim_start_matches("in").parse::<u8>() {
                                *k = format!("in{}", n - 1);
                            }
                        }
                    }

                    if *offset != -1 {
                        o.remove("inoff");
                    }

                    for i in 1..=32 {
                        if i > *count {
                            let l = format!("in{}", i);
                            o.remove(&l);
                        }
                    }

                    de.insert("object".into(), RecursiveStringMap::Map(o));
                }
            }

            // remove in2 if channel is constant
            if let Component::CompositeReadNum { channel, .. } | Component::CompositeReadOnOff { channel, .. } = c {
                if let Some(RecursiveStringMap::Map(mut o)) = de.remove("object") {
                    if *channel == -1 {
                        // for some reason, in these nodes in2 is supposed to go after out1
                        let in2 = o.remove("in2").unwrap();
                        o.insert("in2".into(), in2);
                    } else {
                        o.remove("in2");
                    }

                    de.insert("object".into(), RecursiveStringMap::Map(o));
                }
            }
        }
    }
}

components! { BridgeComponent,
    0 = OnOffIn[][]{
        #[serde(rename = "in1", default, skip_serializing_if = "is_default")]
        unused_input: Option<TypedInputConnection<crate::types::TOnOff, false>>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        output: Option<TypedOutputConnection<crate::types::TOnOff>>,
    },
    1 = OnOffOut[][]{
        #[serde(rename = "in1", default)]
        input: TypedInputConnection<crate::types::TOnOff, false>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        unused_output: Option<TypedOutputConnection<crate::types::TOnOff>>,
    },
    2 = NumberIn[][]{
        #[serde(rename = "in1", default, skip_serializing_if = "is_default")]
        unused_input: Option<TypedInputConnection<crate::types::TNumber, false>>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        output: Option<TypedOutputConnection<crate::types::TNumber>>,
    },
    3 = NumberOut[][]{
        #[serde(rename = "in1", default)]
        input: TypedInputConnection<crate::types::TNumber, false>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        unused_output: Option<TypedOutputConnection<crate::types::TNumber>>,
    },
    4 = CompositeIn[][]{
        #[serde(rename = "in1", default, skip_serializing_if = "is_default")]
        unused_input: Option<TypedInputConnection<crate::types::TComposite, true>>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        output: Option<TypedOutputConnection<crate::types::TComposite>>,
    },
    5 = CompositeOut[][]{
        #[serde(rename = "in1", default)]
        input: TypedInputConnection<crate::types::TComposite, true>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        unused_output: Option<TypedOutputConnection<crate::types::TComposite>>,
    },
    6 = VideoIn[][]{
        #[serde(rename = "in1", default, skip_serializing_if = "is_default")]
        unused_input: Option<TypedInputConnection<crate::types::TVideo, true>>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        output: Option<TypedOutputConnection<crate::types::TVideo>>,
    },
    7 = VideoOut[][]{
        #[serde(rename = "in1", default)]
        input: TypedInputConnection<crate::types::TVideo, true>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        unused_output: Option<TypedOutputConnection<crate::types::TVideo>>,
    },
    8 = AudioIn[][]{
        #[serde(rename = "in1", default, skip_serializing_if = "is_default")]
        unused_input: Option<TypedInputConnection<crate::types::TAudio, true>>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        output: Option<TypedOutputConnection<crate::types::TAudio>>,
    },
    9 = AudioOut[][]{
        #[serde(rename = "in1", default)]
        input: TypedInputConnection<crate::types::TAudio, true>,
        #[serde(rename = "out1", default, skip_serializing_if = "is_default")]
        unused_output: Option<TypedOutputConnection<crate::types::TAudio>>,
    },
    {
        |_c: &BridgeComponent, _de: &mut FakeMap<String, RecursiveStringMap>| {}
    }
}

pub(crate) fn bridge_components_deserialize<'de, D>(de: D) -> Result<Vec<BridgeComponent>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let de = Vec::<_BridgeComponentDe>::deserialize(de)?;
    // println!("{de:?}");
    let cs = de
        .into_iter()
        .map(|mut cde| {
            if cde.inner.get("@type").is_none() {
                cde.inner
                    .insert("@type".into(), RecursiveStringMap::String("0".into()));
            }

            cde.into()
        })
        .collect();

    Ok(cs)
}

pub(crate) fn bridge_components_serialize<S>(
    components: &[BridgeComponent],
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let cdes = components
        .iter()
        .map(|c| {
            let mut se = quick_xml::se::Serializer::new(String::new());
            se.escape(quick_xml::se::QuoteLevel::Partial);
            let ser = c.serialize(se).unwrap();
            let ser = ser.trim_start_matches("<W>").trim_end_matches("</W>");

            let mut cde: _ComponentDe = quick_xml::de::from_str(ser).unwrap();
            if let Some(RecursiveStringMap::String(s)) = cde.inner.get("@type").cloned() {
                if s == "0" {
                    cde.inner.remove("@type");
                }
            }

            cde
        })
        .collect::<Vec<_>>();

    ser.collect_seq(cdes.iter())
}
