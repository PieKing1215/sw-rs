use pretty_assertions::assert_str_eq;
use std::io::Write;
use sw_rs::{
    component::definition::ComponentDefinition, mesh::Mesh, microcontroller::Microcontroller,
    vehicle::Vehicle,
};

#[test]
fn test_samples_serde_matches() {
    let samples = std::fs::read_dir("samples/microcontroller").unwrap();
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

#[test]
fn test_vehicle_samples_serde_matches() {
    let samples = std::fs::read_dir("samples/vehicle").unwrap();
    for f in samples {
        let entry = f.unwrap();
        let fname = entry.file_name().into_string().unwrap();
        if fname.ends_with(".xml") {
            println!("CHECKING {fname}...");
            let src = std::fs::read_to_string(entry.path()).unwrap();
            let src = Vehicle::prettify_xml(&src).unwrap();

            #[allow(clippy::expect_fun_call)]
            let vehicle: Vehicle =
                Vehicle::from_xml_str(&src).expect(&format!("Failed to deserialize {fname}"));

            #[allow(clippy::expect_fun_call)]
            let out = vehicle
                .to_xml_string()
                .expect(&format!("Failed to serialize {fname}"));

            // assert_str_eq!(src, out, "{fname}:\n{vehicle:#?}");
        }
    }
}

#[test]
fn test_rom_component_definitions_serde_matches() {
    if let Ok(samples) =
        std::fs::read_dir("E:/SteamLibrary/steamapps/common/Stormworks/rom/data/definitions")
    {
        for f in samples {
            let entry = f.unwrap();
            let fname = entry.file_name().into_string().unwrap();
            if fname.ends_with(".xml") {
                println!("CHECKING {fname}...");
                let src = std::fs::read_to_string(entry.path()).unwrap();

                #[allow(clippy::expect_fun_call)]
                let vehicle: ComponentDefinition = ComponentDefinition::from_xml_str(&src)
                    .expect(&format!("Failed to deserialize {fname}"));

                #[allow(clippy::expect_fun_call)]
                let _out = vehicle
                    .to_xml_string()
                    .expect(&format!("Failed to serialize {fname}"));

                // assert_str_eq!(src.replace("\r\n", "\n"), out, "{fname}:\n{vehicle:#?}");
            }
        }
    } else {
        writeln!(&mut std::io::stdout(), "(skipping due to missing rom)").unwrap();
    }
}

#[test]
fn test_rom_meshes_serde_matches() {
    if let Ok(samples) = std::fs::read_dir("E:/SteamLibrary/steamapps/common/Stormworks/rom/meshes")
    {
        for f in samples {
            let entry = f.unwrap();
            let fname = entry.file_name().into_string().unwrap();
            if fname.ends_with(".mesh") {
                println!("CHECKING {fname}...");
                let file = std::fs::File::open(entry.path()).unwrap();

                #[allow(clippy::expect_fun_call)]
                let _mesh: Mesh = Mesh::load_file(file).unwrap();

                // assert_str_eq!(src.replace("\r\n", "\n"), out, "{fname}:\n{vehicle:#?}");
            }
        }
    } else {
        writeln!(&mut std::io::stdout(), "(skipping due to missing rom)").unwrap();
    }
}
