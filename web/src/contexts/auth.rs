use crate::{contexts::backend::BackendContext, integrations::backend::Token, router::Route};
use dioxus::prelude::*;
use domain::models::{Note, Property};
use dtos::InfoUserDTO;
use gloo_storage::{LocalStorage, Storage};
use http::error::ErrorInfos;

#[derive(Clone)]
pub struct AuthContext(
    Signal<Option<Token>>,
    Signal<Option<Result<InfoUserDTO, ErrorInfos>>>,
    Signal<Option<Result<Vec<Property>, ErrorInfos>>>,
    Signal<Option<Result<Vec<Note>, ErrorInfos>>>,
);

impl AuthContext {
    pub fn token(&self) -> Signal<Option<Token>> {
        self.0
    }

    pub fn user_infos(&self) -> Signal<Option<Result<InfoUserDTO, ErrorInfos>>> {
        self.1
    }
    pub fn properties(&self) -> Signal<Option<Result<Vec<Property>, ErrorInfos>>> {
        self.2
    }

    pub fn notes(&self) -> Signal<Option<Result<Vec<Note>, ErrorInfos>>> {
        self.3
    }
}

#[component]
pub fn AuthProvider() -> Element {
    let nav = navigator();
    let api = use_context::<BackendContext>().0;

    let mut token_signal = use_signal(|| None);
    let mut user_infos_signal = use_signal(|| None);
    let mut properties_signal = use_signal(|| None);
    let mut notes_signal = use_signal(|| None);

    provide_context(AuthContext(
        token_signal,
        user_infos_signal,
        properties_signal,
        notes_signal,
    ));

    let value = api.clone();
    use_future(move || {
        let api = value.clone();

        async move {
            let token_opt = LocalStorage::get::<Token>("token").ok();
            match token_opt {
                Some(token) => {
                    let user_res = api.auth_user(token.clone()).await;

                    if let Err(_) = &user_res {
                        nav.replace(Route::LoginPage {});
                        token_signal.set(None);
                    } else {
                        token_signal.set(Some(token.clone()));
                        let propertys_res = api.list_propertys(token.clone()).await;
                        let notes_res = api.list_notes(token.clone()).await;

                        user_infos_signal.set(Some(user_res));
                        properties_signal.set(Some(propertys_res));
                        notes_signal.set(Some(notes_res));
                    }
                }
                None => {
                    token_signal.set(None);
                    nav.replace(Route::LoginPage {});
                }
            };
        }
    });

    rsx! {
        Outlet::<Route> {}
    }
}
