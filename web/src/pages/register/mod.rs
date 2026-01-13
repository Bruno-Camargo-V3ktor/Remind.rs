use dioxus::prelude::*;

use crate::pages::form::FormPageBase;

const STYLE: Asset = asset!("./style.css");

#[component]
pub fn RegisterPage() -> Element {
    rsx! {
        FormPageBase {
            div { }
        }
    }
}
