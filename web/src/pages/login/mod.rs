use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::{
    components::{Button, Subtitle, TextInput},
    contexts::{auth::AuthContext, backend::BackendContext},
    pages::form::FormPageBase,
    router::Route,
};

#[component]
pub fn LoginPage() -> Element {
    let navigator = navigator();
    let auth_ctx = use_context::<AuthContext>();
    let api_ctx = use_context::<BackendContext>();

    if auth_ctx.token().read().is_some() {
        navigator.replace(Route::CorkBoardPage {});
    }

    // Email Input
    let email_value = use_signal(|| String::new());
    let email_error = use_signal(|| None);
    let email_validate = move |value| {};

    // Password Input
    let password_value = use_signal(|| String::new());
    let password_error = use_signal(|| None);
    let password_validate = move |value| {};

    // Button Input
    let on_click = move || {
        let api = api_ctx.0.clone();
        let auth = auth_ctx.clone();
        let nav = navigator.clone();

        async move {
            let api = api;
            let nav = nav;
            let auth = auth;

            if email_error().is_some() || password_error().is_some() {
                return;
            }

            let res = api.login_user(email_value(), password_value()).await;

            match res {
                Ok(token) => {
                    let _ = LocalStorage::set("token", token.clone());
                    nav.push(Route::CorkBoardPage {});
                    auth.token().set(Some(token));
                }

                Err(_e) => {}
            }
        }
    };

    rsx! {
        FormPageBase {

            Subtitle { text: "Acessar conta" }
            div { class: "form-container",
                form { class: "form-content",
                    div { class: "form-fields",

                        TextInput {
                            name: "email",
                            label: "E-mail",
                            placeholder: "Digite seu e-mail",
                            value: email_value,
                            error: email_error,
                            validator: email_validate,
                        }

                        TextInput {
                            name: "password",
                            label: "Senha",
                            r#type: "password",
                            placeholder: "Inisra sua senha",
                            value: password_value,
                            error: password_error,
                            validator: password_validate,
                        }
                    }

                    div { class: "form-button",
                        Button {
                            icon: rsx!{},
                            text: "Acessar conta",
                            onclick: on_click
                        }
                    }

                    div { class: "form-footer",
                        span { class: "small-text", "Não tem cadastro?" }
                        Link{ class: "small-link", to: Route::RegisterPage{} , "Criar conta" }
                    }
                }
            }
        }
    }
}
