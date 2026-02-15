use crate::{integrations::backend::Backend, router::Route};
use dioxus::prelude::*;

#[derive(Default, Clone)]
pub struct BackendContext(Backend);

#[component]
pub fn BackendProvider() -> Element {
    provide_context(BackendContext::default());

    rsx! {
        Outlet::<Route> {}
    }
}
