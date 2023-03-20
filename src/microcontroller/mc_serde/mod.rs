//! Module containing ser/de code for microcontrollers

use super::{IONode, IONodeDesign, Microcontroller};

use self::microcontroller::{
    ComponentsBridgeInnerObject, Group, IONodeInner, IONodeSerDe, MicrocontrollerSerDe,
};

pub mod microcontroller;

pub(crate) fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}

// conversion stuff for MicrocontrollerSerDe <-> Microcontroller:

impl From<Microcontroller> for MicrocontrollerSerDe {
    #[allow(clippy::too_many_lines)]
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
            nodes: microcontroller::Nodes {
                nodes: mc
                    .io
                    .iter()
                    .map(|io| IONodeSerDe {
                        id: io.design.node_id,
                        component_id: io.logic.id,
                        node: IONodeInner {
                            label: io.design.label.clone(),
                            mode: io.design.mode,
                            typ: io.design.typ,
                            description: io.design.description.clone(),
                            position: io.design.position.clone().into(),
                        },
                    })
                    .collect(),
            },
            group: Group {
                data: microcontroller::Data { typ: mc.data_type, inputs: (), outputs: () },
                groups: (),
                component_states: mc
                    .components
                    .iter()
                    .map(|c| ComponentsBridgeInnerObject {
                        id: c.id,
                        other: {
                            let mut m = c.ser_to_map();
                            let mut o = m.remove("object").unwrap().into_map().unwrap();
                            if let Some(pos) = m.remove("pos") {
                                o.insert_idx(0, "pos".into(), pos);
                            }
                            o.remove("@id");
                            o
                        },
                    })
                    .collect(),
                component_bridge_states: {
                    let mut v: Vec<_> = mc
                        .io
                        .iter()
                        .map(|ion| &ion.logic)
                        .map(|c| ComponentsBridgeInnerObject {
                            id: c.id,
                            other: {
                                let mut m = c.ser_to_map();
                                let mut o = m.remove("object").unwrap().into_map().unwrap();
                                if let Some(pos) = m.remove("pos") {
                                    o.insert_idx(0, "pos".into(), pos);
                                }
                                o.remove("@id");
                                o
                            },
                        })
                        .collect();

                    v.sort_by_key(|c| {
                        mc.components_bridge_order
                            .iter()
                            .position(|id| *id == c.id)
                            .unwrap()
                    });

                    v
                },
                components: microcontroller::Components { components: mc.components },
                components_bridge: microcontroller::ComponentsBridge {
                    components_bridge: {
                        let mut v: Vec<_> = mc.io.into_iter().map(|ion| ion.logic).collect();

                        v.sort_by_key(|c| {
                            mc.components_bridge_order
                                .iter()
                                .position(|id| *id == c.id)
                                .unwrap()
                        });

                        v
                    },
                },
                group_states: (),
            },
        }
    }
}

impl From<MicrocontrollerSerDe> for Microcontroller {
    fn from(mut sd: MicrocontrollerSerDe) -> Self {
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

            components_bridge_order: sd
                .group
                .components_bridge
                .components_bridge
                .iter()
                .map(|bc| bc.id)
                .collect(),
            io: sd
                .nodes
                .nodes
                .into_iter()
                .map(|n| {
                    let c_idx = sd
                        .group
                        .components_bridge
                        .components_bridge
                        .iter()
                        .position(|c| c.id == n.component_id)
                        .expect(&format!(
                            "Couldn't find node {}'s component with id {}",
                            n.id, n.component_id
                        ));
                    let c = sd.group.components_bridge.components_bridge.remove(c_idx);
                    assert_eq!(
                        n.component_id, c.id,
                        "Node's component_id didn't match component's id"
                    );
                    IONode {
                        design: IONodeDesign {
                            node_id: n.id,
                            label: n.node.label.clone(),
                            mode: n.node.mode,
                            typ: n.node.typ,
                            description: n.node.description.clone(),
                            position: n.node.position.into(),
                        },
                        logic: c,
                    }
                })
                .collect(),
            components: sd.group.components.components,
        }
    }
}
