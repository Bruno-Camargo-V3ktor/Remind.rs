use dioxus::prelude::*;

use crate::{
    components::{Button, Subtitle, TextInput},
    pages::form::FormPageBase,
    router::Route,
};

#[component]
pub fn ResetPasswordPage(token: String) -> Element {
    // Email Input
    let pass01_value = use_signal(|| String::new());
    let pass01_error = use_signal(|| None);
    let pass01_validate = move |value| {};

    // Password Input
    let pass02_value = use_signal(|| String::new());
    let pass02_error = use_signal(|| None);
    let pass02_validate = move |value| {};

    // Button Input
    let on_click = || {};

    rsx! {
        FormPageBase {

            Subtitle { text: "Reseta senha" }
            div { class: "form-container",
                form { class: "form-content",
                    div { class: "form-fields",

                        TextInput {
                            name: "new-password",
                            label: "Nova Senha",
                            placeholder: "Digite sua nova senha",
                            value: pass01_value,
                            error: pass01_error,
                            validator: pass01_validate,
                        }

                        TextInput {
                            name: "new-password-confirme",
                            label: "Repitar sua senha",
                            placeholder: "****",
                            value: pass02_value,
                            error: pass02_error,
                            validator: pass02_validate,
                        }
                    }

                    div { class: "form-button",
                        Button {
                            icon: rsx!{},
                            text: "Definir senha",
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
