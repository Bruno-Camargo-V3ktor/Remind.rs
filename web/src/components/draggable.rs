pub use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Props, Clone, Debug, PartialEq)]
pub struct DraggleProps {
    in_moving: Signal<bool>,
    elem_pos: Signal<Position>,
    children: Element,
    style: Option<String>,
}

#[component]
pub fn Draggable(props: DraggleProps) -> Element {
    //let diff = use_signal(|| None);
    let mut pos = props.elem_pos;
    let in_moving = props.in_moving;

    let zi = if (props.in_moving)() { "10" } else { "auto" };

    rsx! {
        div { class: "draggable-container", style: props.style.unwrap_or_default(), left: "{pos().x}px", top: "{pos().y}px", z_index: zi ,

            onmousemove: move |e| async move {
                let moving = in_moving();
                //let mut diff = diff();

                if moving {
                    let data = e.data();
                    let coordinates = data.coordinates();

                    let mouse_x = coordinates.page().x;
                    let mouse_y = coordinates.page().y;

                    pos.set(Position { x: mouse_x, y: mouse_y });
                }
            },

            { props.children }
        }
    }
}
