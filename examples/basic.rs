use sw_mc::Microcontroller;

pub fn main() {
    let fname = "samples/blank.xml";

    // read file
    let src: String = std::fs::read_to_string(fname).unwrap();

    // deserialize
    let mc: Microcontroller = Microcontroller::from_xml_str(&src).unwrap();

    // modify `mc` here

    // serialize to String
    let out: String = mc.to_xml_string().unwrap();

    // check that output was identical
    if src == out {
        println!("Matches!");
    }
}
