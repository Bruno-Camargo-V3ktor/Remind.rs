use dioxus::prelude::*;

use crate::{
    components::{Button, Subtitle, TextInput},
    pages::form::FormPageBase,
    router::Route,
};

#[component]
pub fn LoginPage() -> Element {
    // Email Input
    let email_value = use_signal(|| String::new());
    let email_error = use_signal(|| None);
    let email_validate = move |value| {};

    // Password Input
    let password_value = use_signal(|| String::new());
    let password_error = use_signal(|| None);
    let password_validate = move |value| {};

    // Button Input
    let on_click = || {};

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
