use pretty_assertions::assert_str_eq;
use sw_mc::Microcontroller;

#[test]
fn test_samples_serde_matches() {
    let samples = std::fs::read_dir("samples").unwrap();
    for f in samples {
        let entry = f.unwrap();
        let fname = entry.file_name().into_string().unwrap();
        if fname.ends_with(".xml") {
            println!("CHECKING {fname}...");
            let src = std::fs::read_to_string(entry.path()).unwrap();

            let mc: Microcontroller = quick_xml::de::from_str(&src)
                .unwrap_or_else(|_| panic!("Failed to deserialize {fname}"));

            let out = mc
                .to_microcontroller_xml()
                .unwrap_or_else(|_| panic!("Failed to serialize {fname}"));

            assert_str_eq!(src.trim(), out.trim(), "{fname}:\n{mc:#?}");
        }
    }
}

#[test]
fn test_sw_dir_serde_matches() {
    let mc_dir = sw_mc::util::find_microcontroller_folder()
        .ok()
        .and_then(|p| std::fs::read_dir(p).ok());
    if let Some(mc_dir) = mc_dir {
        for f in mc_dir {
            let entry = f.unwrap();
            let fname = entry.file_name().into_string().unwrap();
            if fname.ends_with(".xml") {
                println!("CHECKING {fname}...");
                let src = std::fs::read_to_string(entry.path()).unwrap();

                let mc: Microcontroller = quick_xml::de::from_str(&src)
                    .unwrap_or_else(|_| panic!("Failed to deserialize {fname}"));

                let out = mc
                    .to_microcontroller_xml()
                    .unwrap_or_else(|_| panic!("Failed to serialize {fname}"));

                assert_str_eq!(src.trim(), out.trim(), "{fname}:\n{mc:#?}");
            }
        }
    }
}
