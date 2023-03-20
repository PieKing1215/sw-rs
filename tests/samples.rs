use pretty_assertions::assert_str_eq;
use sw_rs::microcontroller::Microcontroller;

#[test]
fn test_samples_serde_matches() {
    let samples = std::fs::read_dir("samples").unwrap();
    for f in samples {
        let entry = f.unwrap();
        let fname = entry.file_name().into_string().unwrap();
        if fname.ends_with(".xml") {
            println!("CHECKING {fname}...");
            let src = std::fs::read_to_string(entry.path()).unwrap();

            #[allow(clippy::expect_fun_call)]
            let mc: Microcontroller = Microcontroller::from_xml_str(&src)
                .expect(&format!("Failed to deserialize {fname}"));

            #[allow(clippy::expect_fun_call)]
            let out = mc
                .to_xml_string()
                .expect(&format!("Failed to serialize {fname}"));

            assert_str_eq!(src, out, "{fname}:\n{mc:#?}");
        }
    }
}

#[test]
fn test_sw_dir_serde_matches() {
    let mc_dir = sw_rs::util::find_microcontroller_folder()
        .ok()
        .and_then(|p| std::fs::read_dir(p).ok());
    if let Some(mc_dir) = mc_dir {
        for f in mc_dir {
            let entry = f.unwrap();
            let fname = entry.file_name().into_string().unwrap();
            if fname.ends_with(".xml") {
                println!("CHECKING {fname}...");
                let src = std::fs::read_to_string(entry.path()).unwrap();

                #[allow(clippy::expect_fun_call)]
                let mc: Microcontroller = Microcontroller::from_xml_str(&src)
                    .expect(&format!("Failed to deserialize {fname}"));

                #[allow(clippy::expect_fun_call)]
                let out = mc
                    .to_xml_string()
                    .expect(&format!("Failed to serialize {fname}"));

                assert_str_eq!(src, out, "{fname}:\n{mc:#?}");
            }
        }
    }
}
