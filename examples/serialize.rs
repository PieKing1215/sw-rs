use serde::{Deserialize, Serialize};
use sw_mc::{
    components::{Component, TypedInputConnection},
    mc_serde::microcontroller::PositionXY,
    Microcontroller,
};

fn main() {
    let mc: Microcontroller = Microcontroller::default();
    let mut se = quick_xml::se::Serializer::new(String::new());
    se.indent('\t', 1);
    se.escape(quick_xml::se::QuoteLevel::Partial);
    let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
    let out = format!("{header}\n{}", mc.serialize(se).unwrap());

    println!("{out}");

    let component = Component::CompositeReadOnOff {
        id: 1,
        pos: PositionXY { x: 1.0, y: 2.0 },
        composite: TypedInputConnection::new(2, 0),
        variable_channel: TypedInputConnection::empty(),
        channel: 0,
        out: Default::default(),
    };

    #[derive(Serialize, Deserialize, Debug)]
    struct R {
        c: Component,
    }

    let v = R { c: component };

    let mut se = quick_xml::se::Serializer::new(String::new());
    se.indent('\t', 1);
    se.escape(quick_xml::se::QuoteLevel::Partial);
    let ser = v.serialize(se).unwrap();
    println!("{ser}");
    println!("{:?}", quick_xml::de::from_str::<R>(&ser).unwrap());
}
