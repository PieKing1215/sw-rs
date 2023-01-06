use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sw_mc::{
    components::{Component, TypedInputConnection},
    mc_serde::microcontroller::{MicrocontrollerSerDe, PositionXY, RecursiveStringMap},
    Microcontroller,
};

fn main() {
    let mc: Microcontroller = MicrocontrollerSerDe::default().into();
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

    //

    #[derive(Serialize, Deserialize, Debug)]
    struct R2 {
        c0: CC,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct CC {
        #[serde(rename = "@id", default)]
        id: u32,
        #[serde(flatten)]
        other: HashMap<String, RecursiveStringMap>,
    }

    let mut rc = HashMap::new();
    rc.insert("a".into(), RecursiveStringMap::String("b".into()));

    let v = R2 { c0: CC { id: 0, other: rc } };

    let mut se = quick_xml::se::Serializer::new(String::new());
    se.indent('\t', 1);
    se.escape(quick_xml::se::QuoteLevel::Partial);
    println!("{}", v.serialize(se).unwrap());

    let des = r#"
<R2>
    <c0 id="2">
        <pos x="-1.25"/>
        <in1/>
        <out1/>
    </c0>
</R2>"#;

    let r2 = quick_xml::de::from_str::<R2>(&des).unwrap();

    println!("{:?}", r2);
}
