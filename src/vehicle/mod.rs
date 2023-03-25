use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::util::serde_utils::RecursiveStringMap;

use self::body::Body;

pub mod body;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "vehicle")]
pub struct Vehicle<C: Default + PartialEq = ()> {
    #[serde(rename = "@data_version")]
    pub data_version: u32,
    #[serde(rename = "@bodies_id")]
    pub bodies_id: u32,
    pub authors: (),
    pub bodies: Bodies<C>,
    pub logic_node_links: (),
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename = "bodies")]
pub struct Bodies<C: Default + PartialEq = ()> {
    #[serde(rename = "body", default)]
    pub nodes: Vec<Body<C>>,
}

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum VehicleSerDeError {
    #[error(transparent)]
    SerDeError(#[from] quick_xml::DeError),
}

impl Vehicle {
    /// # Errors
    /// Returns an [`Err(VehicleSerDeError)`] if the serialization failed, or if the microcontroller was invalid.
    pub fn to_xml_string(&self) -> Result<String, VehicleSerDeError> {
        let mut se = quick_xml::se::Serializer::new(String::new());
        se.indent('\t', 1);
        se.escape(quick_xml::se::QuoteLevel::Partial);
        let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
        Ok(self.serialize(se).map(|s| format!("{header}\n{s}\n\n"))?)
    }

    /// # Errors
    /// Returns an [`Err(VehicleSerDeError)`] if the deserialization failed, or if the microcontroller was invalid.
    pub fn from_xml_str(xml: &str) -> Result<Self, VehicleSerDeError> {
        let mc: Self = quick_xml::de::from_str(xml)?;
        Ok(mc)
    }

    pub fn prettify_xml(xml: &str) -> Result<String, quick_xml::DeError> {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename = "wrapper")]
        struct V {
            #[serde(rename = "vehicle")]
            map: RecursiveStringMap,
        }
        let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

        let map: V = quick_xml::de::from_str(
            format!("<wrapper>{}</wrapper>", xml.trim_start_matches(header)).as_str(),
        )?;
        let mut se = quick_xml::se::Serializer::new(String::new());
        se.indent('\t', 1);
        map.serialize(se).map(|s| {
            format!(
                "{header}\n{}\n\n",
                s.trim_start_matches("<wrapper>\n\t")
                    .trim_end_matches("\n</wrapper>")
                    .replace("\n\t", "\n")
            )
        })
    }
}
