use dioxus::prelude::*;

#[derive(Props, Debug, Clone, PartialEq)]
pub struct SubtitleProps {
    text: String,
    #[props(default = 2.0)]
    size: f32,
    #[props(default = "var(--accent-oragen)".to_string())]
    color: String,
}

#[component]
pub fn Subtitle(props: SubtitleProps) -> Element {
    rsx! {
        div { class: "subtitle-container",
            h4 {class: "subtitle-text", "{props.text}"}
            span { class: "subtitle-line", style: "width: {props.size}rem; height: 0.2rem; background-color: {props.color}; " }
        }
    }
}
