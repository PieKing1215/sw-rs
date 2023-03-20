use pretty_assertions::assert_str_eq;
use serde::Serialize;
use sw_mc::microcontroller::Microcontroller;

fn main() {
    let st = std::fs::read_to_string("samples/mc_blank.xml").unwrap();
    let mc: Microcontroller = quick_xml::de::from_str(&st).unwrap();
    let mut se = quick_xml::se::Serializer::new(String::new());
    se.indent('\t', 1);
    se.escape(quick_xml::se::QuoteLevel::Partial);
    let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
    let out = format!("{header}\n{}", mc.serialize(se).unwrap());

    assert_str_eq!(st.trim(), out.trim());
    println!("OK!");
    println!("{out}");
}
