use dioxus::prelude::*;

#[derive(Clone, Debug, Props, PartialEq)]
pub struct BlurContainerProps {
    pub children: Element,
}

#[component]
pub fn BlurContainer(props: BlurContainerProps) -> Element {
    rsx! {
        div { class: "blur-container",
            {props.children}
        }
    }
}
