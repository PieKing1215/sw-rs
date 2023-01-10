use serde::{Deserialize, Serialize};
use sw_mc::{
    components::{Component, TypedInputConnection},
    util::serde_utils::PositionXY,
    Microcontroller,
};

fn main() {
    #[derive(Serialize, Deserialize, Debug)]
    struct W {
        c: Component,
    }

    let p: W = W {
        c: Component::CompositeReadOnOff {
            pos: PositionXY { x: 1.0, y: 2.0 },
            composite: Some(TypedInputConnection::new(2, 0)),
            variable_channel: Some(TypedInputConnection::empty()),
            channel: 0,
            out: Default::default(),
        },
    };
    let mut se = quick_xml::se::Serializer::new(String::new());
    se.indent('\t', 1);
    se.escape(quick_xml::se::QuoteLevel::Partial);
    let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
    let out = format!("{header}\n{}", p.serialize(se).unwrap());

    println!("se = {out}");
    println!("de = {:?}", quick_xml::de::from_str::<W>(&out));

    let mc: Microcontroller = Microcontroller::default();
    let mut se = quick_xml::se::Serializer::new(String::new());
    se.indent('\t', 1);
    se.escape(quick_xml::se::QuoteLevel::Partial);
    let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
    let out = format!("{header}\n{}", mc.serialize(se).unwrap());

    println!("{out}");

    // let component = ComponentWithId {
    //     id: 1,
    //     component: Component::CompositeReadOnOff {
    //         pos: PositionXY { x: 1.1, y: 2.0 },
    //         composite: Some(TypedInputConnection::new(2, 0)),
    //         variable_channel: Some(TypedInputConnection::empty()),
    //         channel: 0,
    //         out: Default::default(),
    //     },
    // };

    // #[derive(Serialize, Deserialize, Debug)]
    // struct R {
    //     #[serde(
    //         rename = "c",
    //         deserialize_with = "sw_mc::components::component_deserialize",
    //         serialize_with = "sw_mc::components::component_serialize"
    //     )]
    //     c: ComponentWithId,
    // }

    // let v = R { c: component };

    // let mut se = quick_xml::se::Serializer::new(String::new());
    // se.indent('\t', 1);
    // se.escape(quick_xml::se::QuoteLevel::Partial);
    // let ser = v.serialize(se).unwrap();
    // println!("{ser}");
    // println!("{:?}", quick_xml::de::from_str::<R>(&ser).unwrap());
}
