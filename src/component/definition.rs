use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    microcontroller::mc_serde::is_default,
    util::serde_utils::{RecursiveStringMap, Vector3F, Vector3I},
};

/// Note: Deserializing and re-serializing is not guaranteed to result in the exact same result, since the built-in definitions' formatting is wildly inconsistent
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "definition", deny_unknown_fields)]
pub struct ComponentDefinition {
    #[serde(rename = "@name")]
    pub name: String,

    /// Component's category in the component selector
    #[serde(rename = "@category", default, skip_serializing_if = "is_default")]
    pub category: Category,

    /// Component's type. Unknown usage
    #[serde(rename = "@type")]
    pub typ: Type,

    /// Mass of the component
    #[serde(rename = "@mass")]
    pub mass: f32,

    /// Price of the component
    #[serde(rename = "@value")]
    pub value: u32,

    #[serde(
        rename = "@flags",
        default,
        skip_serializing_if = "is_default",
        serialize_with = "ser_flags",
        deserialize_with = "de_flags"
    )]
    pub flags: Flags,

    /// List of the component's tags, as shown in the component selector
    #[serde(
        rename = "@tags",
        default,
        skip_serializing_if = "is_default",
        serialize_with = "ser_tags",
        deserialize_with = "de_tags"
    )]
    pub tags: Vec<String>,

    #[serde(rename = "@phys_collision_dampen")]
    pub phys_collision_dampen: Option<u32>,

    #[serde(rename = "@audio_filename_start")]
    pub audio_filename_start: Option<String>,
    #[serde(rename = "@audio_filename_loop")]
    pub audio_filename_loop: Option<String>,
    #[serde(rename = "@audio_filename_end")]
    pub audio_filename_end: Option<String>,
    #[serde(rename = "@audio_filename_start_b")]
    pub audio_filename_start_b: Option<String>,
    #[serde(rename = "@audio_filename_loop_b")]
    pub audio_filename_loop_b: Option<String>,
    #[serde(rename = "@audio_filename_end_b")]
    pub audio_filename_end_b: Option<String>,
    #[serde(rename = "@audio_gain")]
    pub audio_gain: Option<f32>,

    /// Path to a .mesh, relative to `rom` folder
    #[serde(rename = "@mesh_data_name")]
    pub mesh_data_name: Option<String>,
    /// Path to a .mesh, relative to `rom` folder
    #[serde(rename = "@mesh_0_name")]
    pub mesh_0_name: Option<String>,
    /// Path to a .mesh, relative to `rom` folder
    #[serde(rename = "@mesh_1_name")]
    pub mesh_1_name: Option<String>,
    /// Path to a .mesh, relative to `rom` folder
    #[serde(rename = "@mesh_2_name")]
    pub mesh_2_name: Option<String>,
    /// Path to a .mesh, relative to `rom` folder
    #[serde(rename = "@mesh_editor_only_name")]
    pub mesh_editor_only_name: Option<String>,

    #[serde(rename = "@block_type")]
    pub block_type: Option<u32>, // TODO: figure this out

    #[serde(rename = "@child_name")]
    pub child_name: Option<String>, // TODO: figure this out
    #[serde(rename = "@extender_name")]
    pub extender_name: Option<String>, // TODO: figure this out

    #[serde(rename = "@constraint_type")]
    pub constraint_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@constraint_axis")]
    pub constraint_axis: Option<u32>, // TODO: figure this out
    #[serde(rename = "@constraint_range_of_motion")]
    pub constraint_range_of_motion: Option<f32>,
    #[serde(rename = "@max_motor_force")]
    pub max_motor_force: Option<f32>,
    #[serde(rename = "@max_motor_speed")]
    pub max_motor_speed: Option<f32>,
    #[serde(rename = "@cable_radius")]
    pub cable_radius: Option<f32>,
    #[serde(rename = "@cable_length")]
    pub cable_length: Option<f32>,
    #[serde(rename = "@seat_pose")]
    pub seat_pose: Option<u32>, // TODO: figure this out
    #[serde(rename = "@seat_type")]
    pub seat_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@seat_health_per_sec")]
    pub seat_health_per_sec: Option<f32>,

    #[serde(rename = "@buoy_radius")]
    pub buoy_radius: Option<f32>,
    #[serde(rename = "@buoy_factor")]
    pub buoy_factor: Option<f32>,
    #[serde(rename = "@buoy_force")]
    pub buoy_force: Option<f32>,

    #[serde(rename = "@force_emitter_max_force")]
    pub force_emitter_max_force: Option<f32>,
    #[serde(rename = "@force_emitter_max_vector")]
    pub force_emitter_max_vector: Option<f32>,
    #[serde(rename = "@force_emitter_default_pitch")]
    pub force_emitter_default_pitch: Option<f32>,
    #[serde(rename = "@force_emitter_blade_height")]
    pub force_emitter_blade_height: Option<f32>,
    #[serde(rename = "@force_emitter_rotation_speed")]
    pub force_emitter_rotation_speed: Option<f32>,
    #[serde(rename = "@force_emitter_blade_physics_length")]
    pub force_emitter_blade_physics_length: Option<f32>,
    #[serde(rename = "@force_emitter_blade_efficiency")]
    pub force_emitter_blade_efficiency: Option<f32>,
    #[serde(rename = "@force_emitter_efficiency")]
    pub force_emitter_efficiency: Option<f32>,
    #[serde(rename = "@engine_max_force")]
    pub engine_max_force: Option<f32>,
    #[serde(rename = "@engine_frictionless_force")]
    pub engine_frictionless_force: Option<f32>,

    #[serde(rename = "@trans_conn_type")]
    pub trans_conn_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@trans_type")]
    pub trans_type: Option<u32>, // TODO: figure this out

    #[serde(rename = "@wheel_radius")]
    pub wheel_radius: Option<f32>,
    #[serde(rename = "@wheel_wishbone_length")]
    pub wheel_wishbone_length: Option<f32>,
    #[serde(rename = "@wheel_suspension_height")]
    pub wheel_suspension_height: Option<f32>,
    #[serde(rename = "@wheel_wishbone_margin")]
    pub wheel_wishbone_margin: Option<f32>,
    #[serde(rename = "@wheel_suspension_offset")]
    pub wheel_suspension_offset: Option<f32>,
    #[serde(rename = "@wheel_wishbone_offset")]
    pub wheel_wishbone_offset: Option<f32>,
    #[serde(rename = "@wheel_type")]
    pub wheel_type: Option<u32>, // TODO: figure this out

    #[serde(rename = "@button_type")]
    pub button_type: Option<u32>, // TODO: figure this out

    #[serde(rename = "@light_intensity")]
    pub light_intensity: Option<f32>,
    #[serde(rename = "@light_range")]
    pub light_range: Option<f32>,
    #[serde(rename = "@light_ies_map")]
    pub light_ies_map: Option<String>, // TODO: figure this out
    #[serde(rename = "@light_fov")]
    pub light_fov: Option<f32>,
    #[serde(rename = "@light_type")]
    pub light_type: Option<u32>, // TODO: figure this out

    #[serde(rename = "@door_lower_limit")]
    pub door_lower_limit: Option<f32>,
    #[serde(rename = "@door_upper_limit")]
    pub door_upper_limit: Option<f32>,
    #[serde(rename = "@door_flipped")]
    pub door_flipped: Option<bool>,
    #[serde(rename = "@custom_door_type")]
    pub custom_door_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@door_side_dist")]
    pub door_side_dist: Option<f32>,
    #[serde(rename = "@door_up_dist")]
    pub door_up_dist: Option<f32>,

    #[serde(rename = "@dynamic_min_rotation")]
    pub dynamic_min_rotation: Option<f32>,
    #[serde(rename = "@dynamic_max_rotation")]
    pub dynamic_max_rotation: Option<f32>,

    #[serde(rename = "@logic_gate_type")]
    pub logic_gate_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@logic_gate_subtype")]
    pub logic_gate_subtype: Option<u32>, // TODO: figure this out
    #[serde(rename = "@indicator_type")]
    pub indicator_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@connector_type")]
    pub connector_type: Option<u32>, // TODO: figure this out

    #[serde(rename = "@magnet_force")]
    pub magnet_force: Option<f32>,

    #[serde(rename = "@gyro_type")]
    pub gyro_type: Option<u32>, // TODO: figure this out

    #[serde(rename = "@reward_tier")]
    pub reward_tier: Option<u32>, // TODO: figure this out

    #[serde(rename = "@revision")]
    pub revision: Option<u32>, // TODO: figure this out

    #[serde(rename = "@rudder_surface_area")]
    pub rudder_surface_area: Option<f32>,

    #[serde(rename = "@pump_pressure")]
    pub pump_pressure: Option<f32>,
    #[serde(rename = "@m_pump_pressure")]
    pub m_pump_pressure: Option<f32>,

    #[serde(rename = "@water_component_type")]
    pub water_component_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@torque_component_type")]
    pub torque_component_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@jet_engine_component_type")]
    pub jet_engine_component_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@particle_speed")]
    pub particle_speed: Option<f32>,
    #[serde(rename = "@inventory_type")]
    pub inventory_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@inventory_default_outfit")]
    pub inventory_default_outfit: Option<u32>, // TODO: figure this out
    #[serde(rename = "@inventory_class")]
    pub inventory_class: Option<u32>, // TODO: figure this out
    #[serde(rename = "@inventory_default_item")]
    pub inventory_default_item: Option<u32>, // TODO: figure this out
    #[serde(rename = "@electric_type")]
    pub electric_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@electric_charge_capacity")]
    pub electric_charge_capacity: Option<f32>,
    #[serde(rename = "@electric_magnitude")]
    pub electric_magnitude: Option<f32>,
    #[serde(rename = "@composite_type")]
    pub composite_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@camera_fov_min")]
    pub camera_fov_min: Option<f32>,
    #[serde(rename = "@camera_fov_max")]
    pub camera_fov_max: Option<f32>,
    #[serde(rename = "@monitor_border")]
    pub monitor_border: Option<f32>, // TODO: figure this out
    #[serde(rename = "@monitor_inset")]
    pub monitor_inset: Option<f32>, // TODO: figure this out

    #[serde(rename = "@weapon_type")]
    pub weapon_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@weapon_class")]
    pub weapon_class: Option<u32>, // TODO: figure this out
    #[serde(rename = "@weapon_belt_type")]
    pub weapon_belt_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@weapon_ammo_capacity")]
    pub weapon_ammo_capacity: Option<u32>,
    #[serde(rename = "@weapon_ammo_feed")]
    pub weapon_ammo_feed: Option<bool>,
    #[serde(rename = "@weapon_barrel_length_voxels")]
    pub weapon_barrel_length_voxels: Option<u32>,

    #[serde(rename = "@rx_range")]
    pub rx_range: Option<u32>,
    #[serde(rename = "@rx_length")]
    pub rx_length: Option<f32>,
    #[serde(rename = "@rocket_type")]
    pub rocket_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@radar_range")]
    pub radar_range: Option<u32>,
    #[serde(rename = "@radar_speed")]
    pub radar_speed: Option<f32>,
    #[serde(rename = "@engine_module_type")]
    pub engine_module_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@steam_component_type")]
    pub steam_component_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@steam_component_capacity")]
    pub steam_component_capacity: Option<f32>,
    #[serde(rename = "@nuclear_component_type")]
    pub nuclear_component_type: Option<u32>, // TODO: figure this out
    #[serde(rename = "@radar_type")]
    pub radar_type: Option<u32>, // TODO: figure this out

    #[serde(rename = "@piston_len")]
    pub piston_len: Option<f32>,
    #[serde(rename = "@piston_cam")]
    pub piston_cam: Option<f32>,

    #[serde(rename = "@tool_type")]
    pub tool_type: Option<u32>, // TODO: figure this out

    #[serde(rename = "@oil_component_type")]
    pub oil_component_type: Option<u32>, // TODO: figure this out

    pub surfaces: Surfaces,
    pub buoyancy_surfaces: Surfaces,
    pub logic_nodes: LogicNodes,
    pub voxels: Voxels,

    // TODO: these
    sfx_datas: Option<RecursiveStringMap>,
    couplings: Option<RecursiveStringMap>,

    pub voxel_min: Vector3I,
    pub voxel_max: Vector3I,
    pub voxel_physics_min: Vector3I,
    pub voxel_physics_max: Vector3I,
    pub bb_physics_min: Vector3F,
    pub bb_physics_max: Vector3F,
    pub compartment_sample_pos: Option<Vector3I>,
    pub constraint_pos_parent: Vector3F,
    pub constraint_pos_child: Vector3F,
    pub voxel_location_child: Vector3I,
    pub seat_offset: Option<Vector3F>,
    pub seat_front: Option<Vector3F>,
    pub seat_up: Option<Vector3F>,
    pub seat_camera: Option<Vector3F>,
    pub seat_render: Option<Vector3F>,
    pub seat_exit_position: Option<Vector3F>,
    pub force_dir: Vector3F,
    pub light_position: Vector3I,
    pub light_color: Vector3F,
    pub light_forward: Option<Vector3I>,
    pub door_size: Vector3F,
    pub door_normal: Option<Vector3I>,
    pub door_side: Option<Vector3I>,
    pub door_up: Option<Vector3I>,
    pub door_base_pos: Option<Vector3I>,
    pub dynamic_body_position: Vector3I,
    pub dynamic_rotation_axes: Vector3F,
    pub dynamic_side_axis: Vector3F,
    pub magnet_offset: Vector3F,
    pub connector_axis: Option<Vector3I>,
    pub connector_up: Option<Vector3I>,

    pub tooltip_properties: TooltipProperties,
    pub reward_properties: Option<RewardProperties>,

    pub jet_engine_connections_prev: Option<()>,
    pub jet_engine_connections_next: Option<()>,

    pub particle_direction: Option<Vector3I>,
    pub particle_offset: Option<Vector3F>,
    pub particle_bounds: Option<Vector3F>,
    pub weapon_breech_position: Option<Vector3F>,
    pub weapon_breech_normal: Option<Vector3F>,
    pub weapon_cart_position: Option<Vector3F>,
    pub weapon_cart_velocity: Option<Vector3F>,
    pub rope_hook_offset: Option<Vector3F>,
}

#[allow(clippy::ptr_arg)] // required due to serde
fn ser_tags<S>(tags: &Vec<String>, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    tags.join(",").serialize(ser)
}

fn de_tags<'de, D>(de: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(de).map(|t| t.split(',').map(|s| s.to_string()).collect())
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[serde(transparent)]
    pub struct Flags: u32 {
        /// Unknown purpose; some wedges and a bunch of random other parts
        const _Unknown1         = 0b0000_0000_0000_0001;
        /// All underwater propeller parts
        const WaterPropeller    = 0b0000_0000_0000_0010;
        /// Unknown purpose; only `Aircraft Propeller` and `Rotor (Tail)`
        const _Unknown4         = 0b0000_0000_0000_0100;
        /// Unknown purpose; blocks+wedges and some random other parts
        const _Unknown8         = 0b0000_0000_0000_1000;
        /// Unknown purpose; blocks+wedges and some random other parts
        const _Unknown16        = 0b0000_0000_0001_0000;
        /// Unknown purpose; blocks+wedges and some random other parts
        const _Unknown32        = 0b0000_0000_0010_0000;
        /// Components that have a child part
        const Parent            = 0b0000_0000_0100_0000;
        /// Components that have a parent part
        ///
        /// These are hidden in the vanilla build menu
        const Child             = 0b0000_0000_1000_0000;
        /// Unknown purpose; some random moving pieces
        const _Unknown256       = 0b0000_0001_0000_0000;
        /// Unknown purpose; some random moving piece bases
        const _Unknown512       = 0b0000_0010_0000_0000;
        /// Unknown purpose; all pilot/driver seats and winch ends
        const _Unknown1024      = 0b0000_0100_0000_0000;
        /// Unknown purpose; only `Static Block`, `Train Junction Controller`, and `Hose (Deprecated)`
        const _Unknown2048      = 0b0000_1000_0000_0000;
        /// Unknown purpose; only `Linear Track Base`
        const _Unknown4096      = 0b0001_0000_0000_0000;
        /// Unknown purpose
        const _Unknown8192      = 0b0010_0000_0000_0000;
        /// Unknown purpose
        const _Unknown16384     = 0b0100_0000_0000_0000;
        /// All wing parts
        const Wing              = 0b1000_0000_0000_0000;
        /// Non-wheel suspension parts
        const Suspension        = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        /// Unused
        const _Unknown131072    = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        /// Unknown purpose; only `Turbine Engine`
        const _Unknown262144    = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        /// Unknown purpose; water props, ducted fans, winches, and some anchors
        const _Unknown524288    = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        /// Unknown purpose; electric, fluid, hardpoint, and sliding connectors
        const _Unknown1048576   = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        /// Unknown purpose; only `Jet Exhaust Rotating`
        const _Unknown2097152   = 0b0000_0000_0010_0000_0000_0000_0000_0000;
        /// Unknown purpose; only `Paintable Sign`
        const _Unknown4194304   = 0b0000_0000_0100_0000_0000_0000_0000_0000;
        /// All of the 3x3, 5x5, etc. wheels and tank wheels
        const ModernWheel       = 0b0000_0000_1000_0000_0000_0000_0000_0000;
        /// The old deprecated radio recievers and `Radio Video Recv`
        const OldRadioRX        = 0b0000_0001_0000_0000_0000_0000_0000_0000;
        /// The old deprecated radio transmitters and `Radio Video Xmit`
        const OldRadioTX        = 0b0000_0010_0000_0000_0000_0000_0000_0000;
        /// Radio Video Xmit/Recv
        const RadioVideo        = 0b0000_0100_0000_0000_0000_0000_0000_0000;
        /// Unknown purpose; some cameras and lights
        const _Unknown134217728 = 0b0000_1000_0000_0000_0000_0000_0000_0000;
        /// The new radio recievers (`Radio RX ___`)
        const NewRadioRX        = 0b0001_0000_0000_0000_0000_0000_0000_0000;
        /// Parts that are hidden in the vanilla build menu
        const Hidden            = 0b0010_0000_0000_0000_0000_0000_0000_0000;
        /// Non-deprecated rotors, not including `Rotor (Tail)`
        const ModernRotor       = 0b0100_0000_0000_0000_0000_0000_0000_0000;
        /// Unknown purpose; only `Oil Rig Pumpjack`
        const _Unknown2147483648= 0b1000_0000_0000_0000_0000_0000_0000_0000;
    }
}

fn ser_flags<S>(flags: &Flags, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    flags.bits().serialize(ser)
}

fn de_flags<'de, D>(de: D) -> Result<Flags, D::Error>
where
    D: serde::Deserializer<'de>,
{
    u32::deserialize(de).map(|n| Flags::from_bits(n).unwrap())
}

#[derive(
    Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(from = "u8", into = "u8")]
pub enum Category {
    #[default]
    Blocks,
    VehicleControl,
    Mechanics,
    Propulsion,
    SpecialistEquipment,
    Logic,
    Displays,
    Sensors,
    Decorative,
    Fluid,
    Electric,
    JetEngines,
    Weapons,
    ModularEngines,
    Industry,
    Windows,
    _Other(u8),
}

impl From<u8> for Category {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Blocks,
            1 => Self::VehicleControl,
            2 => Self::Mechanics,
            3 => Self::Propulsion,
            4 => Self::SpecialistEquipment,
            5 => Self::Logic,
            6 => Self::Displays,
            7 => Self::Sensors,
            8 => Self::Decorative,
            9 => Self::Fluid,
            10 => Self::Electric,
            11 => Self::JetEngines,
            12 => Self::Weapons,
            13 => Self::ModularEngines,
            14 => Self::Industry,
            15 => Self::Windows,
            _ => Self::_Other(value),
        }
    }
}

impl From<Category> for u8 {
    fn from(value: Category) -> Self {
        match value {
            Category::Blocks => 0,
            Category::VehicleControl => 1,
            Category::Mechanics => 2,
            Category::Propulsion => 3,
            Category::SpecialistEquipment => 4,
            Category::Logic => 5,
            Category::Displays => 6,
            Category::Sensors => 7,
            Category::Decorative => 8,
            Category::Fluid => 9,
            Category::Electric => 10,
            Category::JetEngines => 11,
            Category::Weapons => 12,
            Category::ModularEngines => 13,
            Category::Industry => 14,
            Category::Windows => 15,
            Category::_Other(value) => value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(from = "u8", into = "u8")]
pub enum Type {
    BlockWedge,
    Seat,
    PropellerRotorFan,
    Float,
    WheelCoaster,
    StockEngine,
    Pipe,
    Motion,
    SimpleInput,
    DecorativeAndLinearTrackExtension,
    SmallLight,
    LargeLight,
    _Unused12,
    SlidingDoorElectric,
    Gyro,
    _Unused15,
    SensorAndLogic,
    Connector,
    Aerodynamic,
    Ladder,
    OldWinch,
    Output,
    Handle,
    _Unused23,
    Fluid,
    ElectricPump,
    _Unused26,
    MagAll,
    Paintable,
    HingedDock,
    DoorFramePanel,
    BasicElectronic,
    EquipmentInventory,
    GearboxAndTorque,
    JetPart,
    Parachute,
    TrainWheelAssembly,
    _Unused37,
    AdvancedOutput,
    Heater,
    Ski,
    Wheel,
    Camera,
    Monitor,
    Weapon,
    Radio,
    KeepActive,
    Rocket,
    AnchorAndWinch,
    FlareLauncher,
    HingedDoor,
    SlidingDoor,
    ModularEngine,
    Steam,
    Nuclear,
    FrictionPad,
    Radar,
    Sonar,
    EndEffectorAndWelder,
    OilRig,
    _Other(u8),
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::BlockWedge,
            1 => Self::Seat,
            2 => Self::PropellerRotorFan,
            3 => Self::Float,
            4 => Self::WheelCoaster,
            5 => Self::StockEngine,
            6 => Self::Pipe,
            7 => Self::Motion,
            8 => Self::SimpleInput,
            9 => Self::DecorativeAndLinearTrackExtension,
            10 => Self::SmallLight,
            11 => Self::LargeLight,
            12 => Self::_Unused12,
            13 => Self::SlidingDoorElectric,
            14 => Self::Gyro,
            15 => Self::_Unused15,
            16 => Self::SensorAndLogic,
            17 => Self::Connector,
            18 => Self::Aerodynamic,
            19 => Self::Ladder,
            20 => Self::OldWinch,
            21 => Self::Output,
            22 => Self::Handle,
            23 => Self::_Unused23,
            24 => Self::Fluid,
            25 => Self::ElectricPump,
            26 => Self::_Unused26,
            27 => Self::MagAll,
            28 => Self::Paintable,
            29 => Self::HingedDock,
            30 => Self::DoorFramePanel,
            31 => Self::BasicElectronic,
            32 => Self::EquipmentInventory,
            33 => Self::GearboxAndTorque,
            34 => Self::JetPart,
            35 => Self::Parachute,
            36 => Self::TrainWheelAssembly,
            37 => Self::_Unused37,
            38 => Self::AdvancedOutput,
            39 => Self::Heater,
            40 => Self::Ski,
            41 => Self::Wheel,
            42 => Self::Camera,
            43 => Self::Monitor,
            44 => Self::Weapon,
            45 => Self::Radio,
            46 => Self::KeepActive,
            47 => Self::Rocket,
            48 => Self::AnchorAndWinch,
            49 => Self::FlareLauncher,
            50 => Self::HingedDoor,
            51 => Self::SlidingDoor,
            52 => Self::ModularEngine,
            53 => Self::Steam,
            54 => Self::Nuclear,
            55 => Self::FrictionPad,
            56 => Self::Radar,
            57 => Self::Sonar,
            58 => Self::EndEffectorAndWelder,
            59 => Self::OilRig,
            v => Self::_Other(v),
        }
    }
}

impl From<Type> for u8 {
    fn from(value: Type) -> Self {
        match value {
            Type::BlockWedge => 0,
            Type::Seat => 1,
            Type::PropellerRotorFan => 2,
            Type::Float => 3,
            Type::WheelCoaster => 4,
            Type::StockEngine => 5,
            Type::Pipe => 6,
            Type::Motion => 7,
            Type::SimpleInput => 8,
            Type::DecorativeAndLinearTrackExtension => 9,
            Type::SmallLight => 10,
            Type::LargeLight => 11,
            Type::_Unused12 => 12,
            Type::SlidingDoorElectric => 13,
            Type::Gyro => 14,
            Type::_Unused15 => 15,
            Type::SensorAndLogic => 16,
            Type::Connector => 17,
            Type::Aerodynamic => 18,
            Type::Ladder => 19,
            Type::OldWinch => 20,
            Type::Output => 21,
            Type::Handle => 22,
            Type::_Unused23 => 23,
            Type::Fluid => 24,
            Type::ElectricPump => 25,
            Type::_Unused26 => 26,
            Type::MagAll => 27,
            Type::Paintable => 28,
            Type::HingedDock => 29,
            Type::DoorFramePanel => 30,
            Type::BasicElectronic => 31,
            Type::EquipmentInventory => 32,
            Type::GearboxAndTorque => 33,
            Type::JetPart => 34,
            Type::Parachute => 35,
            Type::TrainWheelAssembly => 36,
            Type::_Unused37 => 37,
            Type::AdvancedOutput => 38,
            Type::Heater => 39,
            Type::Ski => 40,
            Type::Wheel => 41,
            Type::Camera => 42,
            Type::Monitor => 43,
            Type::Weapon => 44,
            Type::Radio => 45,
            Type::KeepActive => 46,
            Type::Rocket => 47,
            Type::AnchorAndWinch => 48,
            Type::FlareLauncher => 49,
            Type::HingedDoor => 50,
            Type::SlidingDoor => 51,
            Type::ModularEngine => 52,
            Type::Steam => 53,
            Type::Nuclear => 54,
            Type::FrictionPad => 55,
            Type::Radar => 56,
            Type::Sonar => 57,
            Type::EndEffectorAndWelder => 58,
            Type::OilRig => 59,
            Type::_Other(v) => v,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Surfaces {
    #[serde(rename = "surface", default)]
    pub surfaces: Vec<Surface>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Surface {
    #[serde(rename = "@orientation", default)]
    pub orientation: u32, // TODO: figure this out
    #[serde(rename = "@rotation", default)]
    pub rotation: u32, // TODO: figure this out
    #[serde(rename = "@shape", default)]
    pub shape: u32, // TODO: figure this out
    #[serde(rename = "@trans_type", default)]
    pub trans_type: u32, // TODO: figure this out

    pub position: Vector3I,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogicNodes {
    #[serde(rename = "logic_node", default)]
    pub nodes: Vec<LogicNode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogicNode {
    #[serde(rename = "@orientation", default)]
    pub orientation: u32, // TODO: figure this out
    #[serde(rename = "@label", default)]
    pub label: String,
    #[serde(rename = "@mode", default)]
    pub mode: u32, // TODO: figure this out
    #[serde(rename = "@type", default)]
    pub typ: u32, // TODO: figure this out
    #[serde(rename = "@description", default)]
    pub description: String,
    #[serde(rename = "@flags", default)]
    pub flags: u32, // TODO: figure this out

    pub position: Vector3I,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Voxels {
    #[serde(rename = "voxel", default)]
    pub voxels: Vec<Voxel>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Voxel {
    #[serde(rename = "@flags", default)]
    pub flags: u32, // TODO: figure this out
    #[serde(rename = "@physics_shape", default)]
    pub physics_shape: u32, // TODO: figure this out
    #[serde(rename = "@buoy_pipes", default)]
    pub buoy_pipes: u32, // TODO: figure this out

    pub position: Vector3I,
    pub physics_shape_rotation: Option<PhysicsShapeRotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhysicsShapeRotation {
    #[serde(rename = "@00", default)]
    pub m00: i8,
    #[serde(rename = "@01", default)]
    pub m01: i8,
    #[serde(rename = "@02", default)]
    pub m02: i8,
    #[serde(rename = "@10", default)]
    pub m10: i8,
    #[serde(rename = "@11", default)]
    pub m11: i8,
    #[serde(rename = "@12", default)]
    pub m12: i8,
    #[serde(rename = "@20", default)]
    pub m20: i8,
    #[serde(rename = "@21", default)]
    pub m21: i8,
    #[serde(rename = "@22", default)]
    pub m22: i8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "tooltip_properties")]
pub struct TooltipProperties {
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@short_description")]
    pub short_description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "reward_properties")]
pub struct RewardProperties {
    #[serde(rename = "@tier")]
    pub tier: Option<u32>, // TODO: figure this out
    #[serde(rename = "@number_rewarded")]
    pub number_rewarded: u32,
}

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ComponentDefSerDeError {
    #[error(transparent)]
    SerDeError(#[from] quick_xml::DeError),
}

impl ComponentDefinition {
    /// # Errors
    /// Returns an [`Err(ComponentDefSerDeError)`] if the serialization failed, or if the definition was invalid.
    pub fn to_xml_string(&self) -> Result<String, ComponentDefSerDeError> {
        let mut se = quick_xml::se::Serializer::new(String::new());
        se.indent('\t', 1);
        se.escape(quick_xml::se::QuoteLevel::Partial);
        let header = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
        Ok(self.serialize(se).map(|s| format!("{header}\n{s}\n\n"))?)
    }

    /// # Errors
    /// Returns an [`Err(ComponentDefSerDeError)`] if the deserialization failed, or if the definition was invalid.
    pub fn from_xml_str(xml: &str) -> Result<Self, ComponentDefSerDeError> {
        let mut string = xml.into();
        // for some reason radiation_detector.xml has a duplicate `particle_bounds` which breaks quick-xml
        if xml
            .matches(r#"<particle_bounds x="0.2" y="0.2" z="0.2"/>"#)
            .count()
            > 1
        {
            string = xml.replacen(r#"<particle_bounds x="0.2" y="0.2" z="0.2"/>"#, "", 1);
        }
        let mc: Self = quick_xml::de::from_str(&string)?;
        Ok(mc)
    }
}
