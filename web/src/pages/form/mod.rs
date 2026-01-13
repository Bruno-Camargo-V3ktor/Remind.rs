use crate::components::Title;
use dioxus::prelude::*;

const STYLE: Asset = asset!("./style.css");
const DECORATION_IMG: Asset = asset!("assets/sun-tornado.png");

#[component]
pub fn FormPageBase(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }

        div { class: "form-page-container",
            div { class: "form-page-decoration",
                img { class: "img-decoration", background_image: "url({DECORATION_IMG})" }
            }

            div {class: "form-page-content",
                div { class: "form-page-title",
                    Title {}
                }

                div { class: "form-page-children",
                    {children}
                }
            }
        }
    }
}
