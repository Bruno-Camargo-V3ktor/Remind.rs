use crate::components::Iconoir;
use dioxus::prelude::*;

#[derive(Clone)]
pub struct FloatBarContext(EventHandler<(String, Signal<String>)>, Signal<String>);

//#[derive(Props, Debug, Clone, PartialEq)]
//pub struct FloatBarProps {}

#[component]
pub fn FloatBar(
    handle: EventHandler<(String, Signal<String>)>,
    default: String,
    children: Element,
) -> Element {
    let active = use_signal(|| default);
    use_context_provider(|| FloatBarContext(handle, active));

    rsx! {
        div { class: "floatbar-container",
            {children}
        }
    }
}

#[derive(Props, Debug, Clone, PartialEq)]
pub struct FloatBarButtonProps {
    icon: String,
    action: String,
}

#[component]
pub fn FloatBarButton(props: FloatBarButtonProps) -> Element {
    let mut signal_active = use_signal(|| String::new());

    let emit = use_context::<FloatBarContext>();
    let action = props.action.clone();
    let active_signal = emit.1.clone();

    if active_signal() == action && signal_active() != "active" {
        signal_active.set("active".into());
    } else if signal_active() == "active" && active_signal() != action {
        signal_active.set("".into());
    }

    rsx! {
        div { class: "floatbarbutton-container",
            button { class: "floatbarbutton-content floatbarbutton-{signal_active()}", onclick: move |_| {
                signal_active.set(String::from("active"));
                emit.0.call((action.clone(), active_signal))
            },
                Iconoir { icon: "{props.icon}" }
            }
        }
    }
}
