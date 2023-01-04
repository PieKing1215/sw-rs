use pretty_assertions::assert_str_eq;
use sw_mc::Microcontroller;

#[test]
fn test_samples_serde_matches() {
    for f in std::fs::read_dir("samples").unwrap() {
        let entry = f.unwrap();
        let fname = entry.file_name().into_string().unwrap();
        let src = std::fs::read_to_string(entry.path()).unwrap();
        
        let mc: Microcontroller = quick_xml::de::from_str(&src).expect(&format!("Failed to deserialize {fname}"));
        let out = mc.to_microcontroller_xml().expect(&format!("Failed to serialize {fname}"));

        assert_str_eq!(src.trim(), out.trim(), "{fname}");
    }
}
