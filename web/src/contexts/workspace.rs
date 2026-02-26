use gloo_timers::future::sleep;
use std::time::Duration;

use crate::{
    components::{FloatBar, FloatBarButton, Title},
    contexts::auth::AuthContext,
    router::Route,
};
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

#[derive(Default, Clone)]
pub struct WorkspaceContext();

#[component]
pub fn WorkspaceLayout() -> Element {
    let auth_ctx = use_context::<AuthContext>();
    provide_context(WorkspaceContext::default());

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
                    sleep(Duration::from_millis(700)).await;

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
