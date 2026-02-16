use crate::{contexts::backend::BackendContext, integrations::backend::Token, router::Route};
use dioxus::{logger::tracing, prelude::*};
use domain::models::{Note, Property};
use dtos::InfoUserDTO;
use gloo_storage::{LocalStorage, Storage};

#[derive(Clone)]
pub struct AuthContext(
    Option<Token>,
    Resource<Option<InfoUserDTO>>,
    Resource<Option<Vec<Property>>>,
    Resource<Option<Vec<Note>>>,
);

impl AuthContext {
    pub fn token(&self) -> &Option<Token> {
        &self.0
    }

    pub fn user_infos(&self) -> Option<InfoUserDTO> {
        self.1.read().clone().flatten()
    }

    pub fn notes(&self) -> Option<Vec<Note>> {
        self.3.read().clone().flatten()
    }

    pub fn properties(&self) -> Option<Vec<Property>> {
        self.2.read().clone().flatten()
    }
}

#[component]
pub fn AuthProvider() -> Element {
    let nav = navigator();
    let api = use_context::<BackendContext>().0;

    let value = api.clone();
    let user_info = use_resource(move || {
        let token_opt = LocalStorage::get::<Token>("token").ok();
        let api = value.clone();

        async move {
            match token_opt {
                Some(token) => {
                    let res = api.auth_user(token).await;
                    if let Err(error) = &res {
                        tracing::info!("{error:?}");
                        if error.code == "INVALID_TOKEN".to_string() {
                            LocalStorage::delete("token");
                        }
                    }

                    res.ok()
                }
                None => None,
            }
        }
    });

    let value = api.clone();
    let notes_resource = use_resource(move || {
        let token_opt = LocalStorage::get::<Token>("token").ok();
        let api = value.clone();

        async move {
            match token_opt {
                Some(token) => api.list_notes(token).await.ok(),
                None => None,
            }
        }
    });

    let value = api.clone();
    let properties_resource = use_resource(move || {
        let token_opt = LocalStorage::get::<Token>("token").ok();
        let api = value.clone();

        async move {
            match token_opt {
                Some(token) => api.list_propertys(token).await.ok(),
                None => None,
            }
        }
    });

    let token_opt = LocalStorage::get::<Token>("token").ok();
    provide_context(AuthContext(
        token_opt,
        user_info,
        properties_resource,
        notes_resource,
    ));

    use_effect(move || {
        if LocalStorage::get::<Token>("token").is_err() {
            nav.replace(Route::LoginPage {});
        }
    });

    rsx! {
        Outlet::<Route> {}
    }
}
