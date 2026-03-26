use crate::{
    components::{drag::Draggable, Iconoir, Position},
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
    let id = props.id.clone();
    let mut fixed = use_signal(|| props.fixed);

    let mut hover_icon_fixed = use_signal(|| false);
    let mut hover_icon_color = use_signal(|| false);

    let mut in_moving = use_signal(|| false);
    let position = use_signal(|| props.position);

    let mut height = use_signal(|| props.height);
    let mut widht = use_signal(|| props.widht);

    let mut body_raw = use_signal(|| props.body);
    let mut in_focus = use_signal(|| false);

    let mut color_value = use_signal(|| 0_u32);

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

    let draggable_class = if in_moving() {
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
            div { class: format!("note-container note-container-{}  ", if fixed() {"fixed"} else {""} ),
                header { class: format!("note-title {}", draggable_class), onmousedown: toggle_moving, onmouseup:toggle_moving ,
                    div {}
                    h3 { "{props.title}" }
                    div {
                        div {
                            class: "action-btn-note",
                            onmouseenter: move |_| { hover_icon_color.set(true); },
                            onmouseleave: move |_| { hover_icon_color.set(false); },

                            input {
                                id: format!("color-input-{:?}", id.clone().0),
                                r#type: "color",
                                value: format!("#{:06x}", color_value()),
                                style: "display: none;",
                                onchange: move |e| {
                                    let data = e.data();

                                    let hex_value = data.value().replace("#", "");
                                    let color = u32::from_str_radix(&hex_value, 16).unwrap();

                                    color_value.set(color);
                                    dioxus::logger::tracing::info!("{}", color);
                                }
                            }
                            label { for: format!("color-input-{:?}", id.clone().0),
                                Iconoir { style: "transform: rotateY(3.142rad);", icon: if hover_icon_color() {"fill-color-solid"} else {"fill-color"} }
                            }
                        }

                        button {
                            class: "action-btn-note",
                            onmouseenter: move |_| { hover_icon_fixed.set(true); },
                            onmouseleave: move |_| { hover_icon_fixed.set(false); },

                            onclick: move |_| { fixed.set(!fixed()) },

                            Iconoir { icon: if fixed() || hover_icon_fixed() {"pin-solid"} else {"pin"} }
                        }
                    }
                }

                div { class: "note-content", height: "{props.height}px", width: "{ props.widht}px",
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

                    div { class: "note-body",
                        textarea {
                            class: "note-input",
                            value: body_raw,

                            oninput: move |e| { body_raw.set(e.value()); },
                            onfocusin: move |_| { in_focus.set(true); },
                            onfocusout: move |_| { in_focus.set(false); },
                            dangerous_inner_html: body_raw,
                        }
                    }
                }
            }
        }
    }
}
