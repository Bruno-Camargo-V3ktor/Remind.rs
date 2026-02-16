use crate::{integrations::backend::Backend, router::Route};
use dioxus::prelude::*;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct BackendContext(pub Arc<Backend>);

#[component]
pub fn BackendProvider() -> Element {
    provide_context(BackendContext::default());

    rsx! {
        Outlet::<Route> {}
    }
}
