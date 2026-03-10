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
    coordinates: Option<Position>,
    children: Element,
    style: Option<String>,
}

#[component]
pub fn Draggable(props: DraggleProps) -> Element {
    let mut mouse_pos = use_signal(|| None);
    let mut pos = props.elem_pos;
    let in_moving = props.in_moving;

    let mut movement = move |coordinates: Position| {
        let diff_pos = mouse_pos();

        if in_moving() {
            let mouse_x = coordinates.x;
            let mouse_y = coordinates.y;

            let diff_pos = if diff_pos.is_none() {
                let pos = Position {
                    x: pos().x - mouse_x,
                    y: pos().y - mouse_y,
                };

                mouse_pos.set(Some(pos.clone()));
                pos
            } else {
                diff_pos.unwrap()
            };

            pos.set(Position {
                x: mouse_x + diff_pos.x,
                y: mouse_y + diff_pos.y,
            });
        } else if mouse_pos().is_some() {
            mouse_pos.set(None);
        }
    };

    if let Some(coordinates) = props.coordinates {
        movement(coordinates);
    }

    let zi = if (props.in_moving)() { "10" } else { "auto" };
    rsx! {
        div { class: "draggable-container", style: props.style.unwrap_or_default(), left: "{pos().x}px", top: "{pos().y}px", z_index: zi ,

            onmousemove: move |e| async move {
                let data = e.data();
                let coordinates = data.coordinates();

                movement( Position { x:  coordinates.page().x, y: coordinates.page().y } );
            },

            { props.children }
        }
    }
}
