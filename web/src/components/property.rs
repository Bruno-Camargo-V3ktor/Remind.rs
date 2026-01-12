use dioxus::prelude::*;

#[derive(Props, Debug, Clone, PartialEq)]
pub struct PropertyProps {
    text: String,
    color: String,
}

#[component]
pub fn Property(props: PropertyProps) -> Element {
    rsx! {
        div { class: "property-container", style: "background-color: {props.color};",
                p {class: "property-text", "{props.text}"}
        }
    }
}
