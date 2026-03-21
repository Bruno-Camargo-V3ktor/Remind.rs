use crate::{
    components::{drag::Draggable, Position},
    contexts::workspace::InteractiveNote,
};
use dioxus::{html::input_data::MouseButton, prelude::*};
use domain::models::{NoteId, PropertyId};
use gloo_storage::{LocalStorage, Storage};

#[derive(Props, Clone, Debug, PartialEq)]
pub struct NoteProps {
    id: NoteId,
    title: String,
    body: String,
    #[props(default)]
    fixed: bool,
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
    let id = props.id;
    let fixed = use_signal(|| props.fixed);
    let mut in_moving = use_signal(|| false);
    let position = use_signal(|| props.position);

    let mut height = use_signal(|| props.height);
    let mut widht = use_signal(|| props.widht);

    let mut body_raw = use_signal(|| props.body);
    let mut in_focus = use_signal(|| false);

    let toggle_moving = move |e: Event<MouseData>| {
        e.stop_propagation();

        let data = e.data();
        match data.trigger_button() {
            Some(MouseButton::Primary) => {
                let inverse = !in_moving();
                in_moving.set(inverse);
            }
            _ => {}
        }
    };

    let class = if in_moving() {
        "draggable-active"
    } else {
        "draggable-disable"
    };

    use_effect(move || {
        let fixed = fixed();
        let position = position();
        let height = height();
        let widht = widht();

        let _ = LocalStorage::set(
            id.0.to_string(),
            InteractiveNote {
                fixed,
                position,
                height,
                widht,
            },
        );
    });

    rsx! {
        Draggable { in_moving: in_moving, elem_pos: position, style: "border-radius: 2rem;",
            div { class: "note-container",
                header { class: format!("note-title {}", class), onmousedown: toggle_moving, onmouseup:toggle_moving ,
                    h3 { "{props.title}" }
                }

                div { class: "note-content", height: "{props.height}px", width: "{ props.widht}px",
                    div { class: "note-body",
                        textarea {
                            class: "note-input",
                            value: body_raw,
                            oninput: move |e| { body_raw.set(e.value()); },
                            onfocusin: move |_| { in_focus.set(true); },
                            onfocusout: move |_| { in_focus.set(false); },
                            dangerous_inner_html: body_raw,

                            onresize: move |e| {
                                let data = e.data();

                                match data.get_content_box_size() {
                                    Ok(v) => {
                                        widht.set(v.width);
                                        height.set(v.height);
                                    }
                                    Err(_) => {}
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
