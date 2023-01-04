use serde::Serialize;
use sw_mc::Microcontroller;

fn main() {
    let mc: Microcontroller = Microcontroller::default();
    let mut se = quick_xml::se::Serializer::new(String::new());
    se.indent('\t', 1);
    let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
    let out = format!("{header}\n{}", mc.serialize(se).unwrap());

    println!("{out}");
}
