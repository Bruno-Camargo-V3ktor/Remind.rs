use dioxus::prelude::*;

use crate::{
    components::{Button, Subtitle, TextInput},
    pages::form::FormPageBase,
    router::Route,
};

#[component]
pub fn RegisterPage() -> Element {
    // Email Input
    let firstN_value = use_signal(|| String::new());
    let firstN_error = use_signal(|| None);
    let firstN_validate = move |value| {};

    // Password Input
    let lastN_value = use_signal(|| String::new());
    let lastN_error = use_signal(|| None);
    let lastN_validate = move |value| {};

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
                            value: password_value,
                            error: password_error,
                            validator: password_validate,
                        }
                    }

                    div { class: "form-button",
                        Button {
                            icon: rsx!{},
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
