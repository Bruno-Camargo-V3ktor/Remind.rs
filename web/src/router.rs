use crate::contexts::auth::AuthProvider;
use crate::contexts::backend::BackendProvider;
use crate::pages::{
    corkboard::CorkBoardPage, login::LoginPage, preview::PreviewPage, register::RegisterPage,
    reset_password::ResetPasswordPage,
};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(BackendProvider)]
    #[layout(AuthProvider)]
        #[route("/preview")]
        PreviewPage { },

        #[route("/")]
        CorkBoardPage {},

        #[route("/register")]
        RegisterPage { },

        #[route("/login")]
        LoginPage { },

        #[route("/reset-password/:token")]
        ResetPasswordPage { token: String },
    #[end_layout]
    #[end_layout]

    #[route("/:..params")]
    NotFound{ params: Vec<String> }
}

#[component]
fn NotFound(params: Vec<String>) -> Element {
    let _ = params;
    let nav = navigator();
    nav.replace(Route::CorkBoardPage {});

    rsx! {}
}
