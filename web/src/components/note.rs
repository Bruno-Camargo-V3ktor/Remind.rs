use dioxus::{html::input_data::MouseButton, prelude::*};

use crate::components::draggable::{Draggable, Position};

#[component]
pub fn Note() -> Element {
    let mut in_moving = use_signal(|| false);
    let position = use_signal(|| Position { x: 0.0, y: 0.0 });

    let ondown_header = move |e: Event<MouseData>| {
        if !in_moving() {
            let data = e.data();

            match data.trigger_button() {
                Some(MouseButton::Primary) => {
                    in_moving.set(true);
                }
                _ => {}
            }
        }
    };

    let onrelease_header = move |e: Event<MouseData>| {
        let data = e.data();

        match data.trigger_button() {
            Some(MouseButton::Primary) => {
                in_moving.set(false);
            }
            _ => {}
        }
    };

    rsx! {
        Draggable { in_moving: in_moving, elem_pos: position, style: "border-radius: 2rem;",
            div { class: "note-container",
                header { class: "note-title", onmousedown: ondown_header, onmouseup:onrelease_header ,
                    h3 { "Minha Nota" }
                }

                div { class: "note-content",
                    div { class: "note-body",
                        //contenteditable: true,
                        //dangerous_inner_html: r#"<stronk style="color: red">Teste</stronk>"#,
                        textarea { class: "note-input" }
                    }
                }
            }
        }
    }
}
