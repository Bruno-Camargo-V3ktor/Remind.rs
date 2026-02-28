use crate::components::draggable::{Draggable, Position};
use dioxus::{html::input_data::MouseButton, logger::tracing, prelude::*};
use domain::models::PropertyId;

#[derive(Props, Clone, Debug, PartialEq)]
pub struct NoteProps {
    title: String,
    body: String,
    propertys: Vec<PropertyId>,
    #[props(default = Position { x: 0.0, y: 0.0 })]
    position: Position,
    #[props(default = 100.0)]
    widht: f64,
    #[props(default = 100.0)]
    height: f64,
}

#[component]
pub fn Note(props: NoteProps) -> Element {
    let mut in_moving = use_signal(|| false);
    let position = use_signal(|| props.position);

    let mut body_raw = use_signal(|| props.body);
    let mut in_focus = use_signal(|| false);

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
                    h3 { "{props.title}" }
                }

                div { class: "note-content", style: "height: {props.height}; widht: {props.widht};",
                    div { class: "note-body",
                        textarea {
                            class: "note-input",
                            value: body_raw,
                            oninput: move |e| { body_raw.set(e.value()); },
                            onfocusout: move |_| { in_focus.set(false); },
                            dangerous_inner_html: body_raw
                        }

                        div {
                            class: "note-input",
                            contenteditable: true,
                            onfocusin: move |_| { in_focus.set(true); },
                            dangerous_inner_html: body_raw
                        }
                    }
                }
            }
        }
    }
}
