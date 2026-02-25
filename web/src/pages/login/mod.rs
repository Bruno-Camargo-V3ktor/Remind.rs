use dioxus::{logger::tracing, prelude::*};
use gloo_storage::{LocalStorage, Storage};

use crate::{
    components::{Button, IconSide, Iconoir, Subtitle, TextInput},
    contexts::{auth::AuthContext, backend::BackendContext},
    pages::form::FormPageBase,
    router::Route,
};

#[component]
pub fn LoginPage() -> Element {
    let mut send_request = use_signal(|| false);
    let mut falied_login = use_signal(|| false);

    // Email Input
    let mut email_value = use_signal(|| String::new());
    let mut email_error = use_signal(|| None);
    let email_validate = move |value| {
        email_value.set(value);
        email_error.set(None);
    };

    // Password Input
    let mut password_value = use_signal(|| String::new());
    let mut password_error = use_signal(|| None);
    let password_validate = move |value| {
        password_value.set(value);
        password_error.set(None);
    };

    // Button Input
    let navigator = navigator();
    let auth_ctx = use_context::<AuthContext>();
    let api_ctx = use_context::<BackendContext>();

    if auth_ctx.token().read().is_some() {
        navigator.replace(Route::CorkBoardPage {});
    }

    let on_click = move || {
        let api = api_ctx.clone().0;
        let auth = auth_ctx.clone();
        let nav = navigator.clone();

        async move {
            if email_error().is_some() || password_error().is_some() {
                return;
            }

            let email_input = email_value();
            let password_input = password_value();
            let mut is_error = false;

            if email_input.is_empty() {
                email_error.set(Some("E-mail não pode ser vazio.".into()));
                is_error = true;
            } else if !email_input.contains("@") {
                email_error.set(Some("O Campo acima não e um E-mail valido.".into()));
                is_error = true;
            }

            if password_input.is_empty() {
                password_error.set(Some("Senha não pode ser vazio.".into()));
                is_error = true;
            }

            if is_error {
                return;
            }

            send_request.set(true);
            let res = api.login_user(email_value(), password_value()).await;
            match res {
                Ok(token) => {
                    let _ = LocalStorage::set("token", token.clone());
                    nav.push(Route::CorkBoardPage {});
                    auth.token().set(Some(token));
                }

                Err(e) => {
                    if &e.code == "INVALID_CREDENTIALS" {
                        password_error.set(Some("Credencias Invalidas.".into()));
                        falied_login.set(true);
                    } else if &e.code == "USER_NOT_EXIST" {
                        email_error.set(Some("Usuario não encontrado.".into()));
                    }
                }
            }

            send_request.set(false);
        }
    };

    // Link Input
    let api_ctx = use_context::<BackendContext>();

    let click_reset_password = move |_| {
        let api = api_ctx.0.clone();
        async move {
            let email = email_value();
            let _ = api
                .request_new_password(email, "http://localhost:8080".into())
                .await;
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
                            disable: send_request(),
                            icon: rsx! {
                                if send_request() {
                                    Iconoir {
                                        icon: "hourglass",
                                        class: "rotation-icon",
                                    }
                                } else {
                                    {}
                                }
                            },
                            icon_side: IconSide::Right,
                            text: "Acessar conta",
                            onclick: on_click
                        }
                    }

                    div { class: "form-footer",
                        if falied_login() {
                            div {
                                a { onclick: click_reset_password, class: "small-link", "Clique aqui para manda um E-mail de reset de senha" }
                            }
                        }

                        div {
                            span { class: "small-text", "Não tem cadastro?" }
                            Link{ class: "small-link", to: Route::RegisterPage{} , "Criar conta" }
                        }
                    }
                }
            }
        }
    }
}
