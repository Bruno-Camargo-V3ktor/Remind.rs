use dioxus::prelude::*;

#[derive(Props, Clone, Debug, PartialEq, Eq)]
pub struct IconoirProps {
    icon: String,
    class: Option<String>,
    id: Option<String>,
    style: Option<String>,
}

#[component]
pub fn Iconoir(props: IconoirProps) -> Element {
    rsx! {
        i { class: format!("icon iconoir-{} {}", props.icon, props.class.unwrap_or_default()), id: props.id.unwrap_or_default(), style: props.style.unwrap_or_default() }
    }
}
