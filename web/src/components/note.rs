use dioxus::{html::input_data::MouseButton, prelude::*};

#[derive(Clone, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[component]
pub fn Note() -> Element {
    //let mut elem = use_signal(|| None);
    let mut z_index = use_signal(|| 9);
    let mut in_moving = use_signal(|| (false, None));
    let mut elem_pos = use_signal(|| Position { x: 10.0, y: 10.0 });

    rsx! {
        div { class: "note-container", left: "{elem_pos().x}px", top: "{elem_pos().y}px", z_index: "{z_index}",

            header { class: "note-title",
                onmousedown: move |e| {
                  if !in_moving().0 {
                    let data = e.data();

                    match data.trigger_button() {
                        Some(MouseButton::Primary) => {
                            in_moving.write().0 = true;
                            z_index.set(10);
                        }
                        _ => {}
                    }
                  }
                },

                onmouseup: move |e| {
                  let data = e.data();

                  match data.trigger_button() {
                      Some(MouseButton::Primary) => {
                          in_moving.write().0 = false;
                          in_moving.write().1 = None;
                           z_index.set(9);
                      }
                      _ => {}
                  }
                },

                onmousemove: move |e| async move {
                    let (moving, mut diff) = in_moving();

                    if moving {
                        let data = e.data();
                        let coordinates = data.coordinates();

                        let mouse_x = coordinates.page().x;
                        let mouse_y = coordinates.page().y;

                        let pos = elem_pos();

                        if diff.is_none() {
                            let pos = Position {
                                x: mouse_x - pos.x,
                                y: mouse_y - pos.y,
                            };

                            in_moving.write().1 = Some(pos.clone());
                            diff = Some(pos.clone());
                        }

                        let diff = diff.unwrap();

                        elem_pos.set(Position { x: mouse_x - diff.x, y: mouse_y - diff.y });
                    }
                },

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
