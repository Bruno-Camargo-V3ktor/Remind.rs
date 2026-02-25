use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::{
    components::{Button, IconSide, Iconoir, Subtitle, TextInput},
    contexts::{auth::AuthContext, backend::BackendContext},
    pages::form::FormPageBase,
    router::Route,
};

#[component]
pub fn RegisterPage() -> Element {
    let mut send_request = use_signal(|| false);

    // Email Input
    let mut firstN_value = use_signal(|| String::new());
    let mut firstN_error = use_signal(|| None);
    let firstN_validate = move |value| {
        firstN_value.set(value);
        firstN_error.set(None);
    };

    // Password Input
    let mut lastN_value = use_signal(|| String::new());
    let mut lastN_error = use_signal(|| None);
    let lastN_validate = move |value| {
        lastN_value.set(value);
        lastN_error.set(None);
    };

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
            if email_error().is_some()
                || password_error().is_some()
                || firstN_error().is_some()
                || lastN_error().is_some()
            {
                return;
            }

            let email_input = email_value();
            let password_input = password_value();
            let firstN_input = firstN_value();
            let lastN_input = lastN_value();
            let mut is_error = false;

            // FirstName Validations...
            if firstN_input.len() <= 3 {
                firstN_error.set(Some("O Nome precisa ter no mínimo 3 letras.".into()));
                is_error = true;
            }

            // LastName Validations...
            if lastN_input.len() <= 3 {
                lastN_error.set(Some("O Sobrenome precisa ter no mínimo 3 letras.".into()));
                is_error = true;
            }

            // Email Validations...
            if email_input.is_empty() {
                email_error.set(Some("E-mail não pode ser vazio.".into()));
                is_error = true;
            } else if !email_input.contains("@") {
                email_error.set(Some("O Campo acima não e um E-mail valido.".into()));
                is_error = true;
            }

            // Password Validations...
            if password_input.is_empty() {
                password_error.set(Some("Senha não pode ser vazio.".into()));
                is_error = true;
            }

            // Flag
            if is_error {
                return;
            }

            send_request.set(true);
            let res = api
                .register_user(firstN_input, lastN_input, email_input, password_input)
                .await;
            match res {
                Ok(token) => {
                    let _ = LocalStorage::set("token", token.clone());
                    nav.push(Route::CorkBoardPage {});
                    auth.token().set(Some(token));
                }

                Err(_) => {}
            }

            send_request.set(false);
        }
    };

    rsx! {
        FormPageBase {

            Subtitle { text: "Criar conta" }
            div { class: "form-container",
                form { class: "form-content",
                    div { class: "form-fields",
                        div { class: "form-row",
                            TextInput {
                                name: "first-name",
                                label: "Nome",
                                placeholder: "Paulette",
                                value: firstN_value,
                                error: firstN_error,
                                validator: firstN_validate,
                            }

                            TextInput {
                                name: "last-name",
                                label: "Sobrenome",
                                placeholder: "Hermisston",
                                value: lastN_value,
                                error: lastN_error,
                                validator: lastN_validate,
                            }
                        }


                        TextInput {
                            name: "email",
                            label: "E-mail",
                            placeholder: "paulette_hermiston-smith@hotmail.com",
                            value: email_value,
                            error: email_error,
                            validator: email_validate,
                        }

                        TextInput {
                            name: "password",
                            label: "Senha",
                            placeholder: "****",
                            r#type: "password",
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
                            text: "Criar conta",
                            onclick: on_click
                        }
                    }

                    div { class: "form-footer",
                        span { class: "small-text", "Já tem cadastro?" }
                        Link{ class: "small-link", to: Route::LoginPage { } , "Acessar conta" }
                    }
                }
            }
        }
    }
}
