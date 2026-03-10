use crate::components::{
    drag::{Draggable, Position},
    Note,
};
use dioxus::{html::input_data::MouseButton, prelude::*};

const STYLE: Asset = asset!("./style.css");

#[component]
pub fn CorkBoardPage() -> Element {
    let mut mouse_pan = use_signal(|| false);
    let mut coordenates = use_signal(|| Position { x: 0.0, y: 0.0 });
    let origin_pos = use_signal(|| Position { x: 0.0, y: 0.0 });

    let toggle_pan = move |e: Event<MouseData>| {
        e.stop_propagation();

        let data = e.data();
        match data.trigger_button() {
            Some(MouseButton::Primary) => {
                let inverse = !mouse_pan();
                mouse_pan.set(inverse);

                if inverse {
                    let coordinates = data.coordinates();
                    coordenates.set(Position {
                        x: coordinates.page().x,
                        y: coordinates.page().y,
                    });
                }
            }
            _ => {}
        }
    };

    let active_class = if mouse_pan() { "draggable-active" } else { "" };

    rsx! {
        div {
            class: format!("corkboard-content {}", active_class),
            onmousedown: toggle_pan,
            onmouseup: toggle_pan,
            onmousemove: move |e| {
                if mouse_pan() {
                    let data = e.data();
                    let coordinates = data.coordinates();
                    coordenates.set( Position { x:  coordinates.page().x, y: coordinates.page().y } );
                }
            }
        }

        Draggable { in_moving: mouse_pan, elem_pos: origin_pos, coordinates: coordenates(), style: "overflow: visible;",
            Note {
                title: "Minha Nota",
                body: "",
                propertys: vec![],
                widht: 300.0,
                height: 300.0,
            }

            Note {
                title: "Minha Nota 2",
                body: "asdasdad",
                propertys: vec![],
                widht: 100.0,
                height: 100.0,
            }
        }
    }
}
