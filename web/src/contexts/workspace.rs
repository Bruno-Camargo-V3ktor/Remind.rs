use crate::{
    components::{drag::Position, FloatBar, FloatBarButton, Title},
    contexts::auth::AuthContext,
    router::Route,
};
use dioxus::prelude::*;
use domain::models::NoteId;
use gloo_storage::{LocalStorage, Storage};
use gloo_timers::future::sleep;
use std::{collections::HashMap, time::Duration};

#[derive(Clone)]
pub struct InteractiveNote {
    pub edited: bool,
    pub fixed: bool,
    pub height: f64,
    pub widht: f64,
    pub position: Position,
}

#[derive(Clone)]
pub struct WorkspaceContext {
    pub interactive_notes: Signal<HashMap<NoteId, InteractiveNote>>,
}

#[component]
pub fn WorkspaceLayout() -> Element {
    let auth_ctx = use_context::<AuthContext>();
    let mut notes = use_signal(|| HashMap::new());

    if let Some(Ok(ns)) = (auth_ctx.notes())() {
        for note in ns {
            notes.insert(
                note.id,
                InteractiveNote {
                    edited: false,
                    fixed: false,
                    height: 300.0,
                    widht: 300.0,
                    position: Position { x: 0.0, y: 0.0 },
                },
            );
        }
    }

    provide_context(WorkspaceContext {
        interactive_notes: notes,
    });

    let active_floatbar = use_signal(|| String::from("home"));
    let floatbar_handle = move |(action, mut state): (String, Signal<String>)| {
        let auth_ctx = use_context::<AuthContext>();

        let old_state = state();
        if old_state == action {
            return;
        }

        let nav = navigator();
        state.set(action.clone());
        match action.as_str() {
            "home" => {
                nav.push(Route::CorkBoardPage {});
            }
            "user" => {
                nav.push(Route::UserPage {});
            }
            "logout" => {
                spawn(async move {
                    sleep(Duration::from_millis(500)).await;

                    LocalStorage::delete("token");
                    auth_ctx.token().set(None);
                    nav.replace(Route::LoginPage {});
                });
            }
            _ => {}
        }
    };

    rsx! {
        div { class: "fixed-title",
            Title { }
        }

        Outlet::<Route> {}

        div { class: "fixed-floatbar",
            FloatBar { handle: floatbar_handle, state: active_floatbar,

                FloatBarButton { icon: "list", action: "home" }
                FloatBarButton { icon: "user-circle" , action: "user"}
                FloatBarButton { icon: "log-out" , action: "logout"}
            }
        }
    }
}
