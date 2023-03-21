use sw_rs::vehicle::Vehicle;

pub fn main() {
    let fname = "samples/vehicle/sweditor3.xml";

    // read file
    let src: String = std::fs::read_to_string(fname).unwrap();

    // deserialize
    let vehicle: Vehicle = Vehicle::from_xml_str(&src).unwrap();

    // could modify `vehicle` here
    println!("{vehicle:?}");

    // serialize to String
    let out: String = vehicle.to_xml_string().unwrap();

    // check that output was identical
    if src == out {
        println!("Matches!");
    }
}
