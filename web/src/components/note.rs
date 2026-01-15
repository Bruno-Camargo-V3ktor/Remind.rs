use dioxus::prelude::*;

#[component]
pub fn Note() -> Element {
    rsx! {
        div { class: "note-container",
            header { class: "note-title",
                h3 { "Minha Nota" }
            }

            div { class: "note-content",

                div { class: "note-body",
                    //contenteditable: true,
                    dangerous_inner_html: r#"<stronk style="color: red">Teste</stronk>"#
                }
            }
        }
    }
}
