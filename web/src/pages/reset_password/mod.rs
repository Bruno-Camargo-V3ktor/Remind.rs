use dioxus::prelude::*;
use dtos::UpdateUserDTO;

use crate::{
    components::{Button, IconSide, Iconoir, Subtitle, TextInput},
    contexts::backend::BackendContext,
    integrations::backend::Token,
    pages::form::FormPageBase,
    router::Route,
};

#[component]
pub fn ResetPasswordPage(token: String) -> Element {
    let mut send_request = use_signal(|| false);

    // Email Input
    let mut pass01_value = use_signal(|| String::new());
    let mut pass01_error = use_signal(|| None);
    let pass01_validate = move |value| {
        pass01_value.set(value);
        pass01_error.set(None);
    };

    // Password Input
    let mut pass02_value = use_signal(|| String::new());
    let mut pass02_error = use_signal(|| None);
    let pass02_validate = move |value| {
        pass02_value.set(value);
        pass02_error.set(None);
    };

    // Button Input
    let backend_ctx = use_context::<BackendContext>();

    let on_click = move || {
        let api = backend_ctx.clone().0;
        let token = token.clone();
        let nav = navigator();

        async move {
            let pass01 = pass01_value();
            let pass02 = pass02_value();
            let mut is_error = false;

            // Validate
            if pass01.len() < 4 {
                pass01_error.set(Some("Senha precisa de no mínimo 4 letras.".into()));
                is_error = true;
            }

            if pass02 != pass01 {
                pass02_error.set(Some("As Senhas não são iguais.".into()));
                is_error = true;
            }

            if pass01_error().is_some() || pass02_error().is_some() || is_error {
                return;
            }

            let dto = UpdateUserDTO {
                bio: None,
                email: None,
                name: None,
                password: Some(pass01),
                photo: None,
            };

            send_request.set(true);
            match api.update_user(Token(token), dto).await {
                Ok(_) => {
                    nav.replace(Route::LoginPage {});
                }

                Err(e) => {
                    if &e.code == "INVALID_TOKEN" {
                        pass02_error.set(Some("Tempo para redefinir senha expirado.".into()));
                    }
                }
            }

            send_request.set(false);
        }
    };

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
                            r#type: "password",
                            value: pass01_value,
                            error: pass01_error,
                            validator: pass01_validate,
                        }

                        TextInput {
                            name: "new-password-confirme",
                            label: "Repitar sua senha",
                            placeholder: "****",
                            r#type: "password",
                            value: pass02_value,
                            error: pass02_error,
                            validator: pass02_validate,
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
