use dioxus::prelude::*;

use crate::contexts::auth::AuthContext;

const _STYLE: Asset = asset!("./style.css");

#[component]
pub fn UserPage() -> Element {
    let auth_ctx = use_context::<AuthContext>();
    let _user_infos = auth_ctx.user_infos();

    rsx! {}
}
