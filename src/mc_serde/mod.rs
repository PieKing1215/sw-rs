//! Module containing ser/de code for microcontrollers

use crate::Microcontroller;

use self::microcontroller::{ComponentsBridgeInnerObject, Group, MicrocontrollerSerDe};

pub mod microcontroller;

pub(crate) fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}

// conversion stuff for MicrocontrollerSerDe <-> Microcontroller:

impl From<Microcontroller> for MicrocontrollerSerDe {
    fn from(mc: Microcontroller) -> Self {
        MicrocontrollerSerDe {
            name: mc.name,
            description: mc.description,
            width: mc.width,
            length: mc.length,
            id_counter: mc.id_counter,
            id_counter_node: mc.id_counter_node,
            sym0: mc.icon[0],
            sym1: mc.icon[1],
            sym2: mc.icon[2],
            sym3: mc.icon[3],
            sym4: mc.icon[4],
            sym5: mc.icon[5],
            sym6: mc.icon[6],
            sym7: mc.icon[7],
            sym8: mc.icon[8],
            sym9: mc.icon[9],
            sym10: mc.icon[10],
            sym11: mc.icon[11],
            sym12: mc.icon[12],
            sym13: mc.icon[13],
            sym14: mc.icon[14],
            sym15: mc.icon[15],
            nodes: mc.nodes,
            group: Group {
                data: microcontroller::Data { typ: mc.data_type, inputs: (), outputs: () },
                groups: (),
                component_states: mc
                    .components
                    .iter()
                    .map(|c| ComponentsBridgeInnerObject {
                        id: c.id(),
                        other: {
                            let mut m =
                                c.ser_to_map().remove("object").unwrap().into_map().unwrap();
                            m.remove("@id");
                            m
                        },
                    })
                    .collect(),
                component_bridge_states: mc
                    .components_bridge
                    .iter()
                    .map(|cb| cb.object.clone())
                    .collect(),
                components: microcontroller::Components { components: mc.components },
                components_bridge: microcontroller::ComponentsBridge {
                    components_bridge: mc.components_bridge,
                },
                group_states: (),
            },
        }
    }
}

impl From<MicrocontrollerSerDe> for Microcontroller {
    fn from(sd: MicrocontrollerSerDe) -> Self {
        Self {
            name: sd.name,
            description: sd.description,
            width: sd.width,
            length: sd.length,
            id_counter: sd.id_counter,
            id_counter_node: sd.id_counter_node,
            icon: [
                sd.sym0, sd.sym1, sd.sym2, sd.sym3, sd.sym4, sd.sym5, sd.sym6, sd.sym7, sd.sym8,
                sd.sym9, sd.sym10, sd.sym11, sd.sym12, sd.sym13, sd.sym14, sd.sym15,
            ],
            data_type: sd.group.data.typ,

            nodes: sd.nodes,
            components: sd.group.components.components,
            components_bridge: sd.group.components_bridge.components_bridge,
        }
    }
}
