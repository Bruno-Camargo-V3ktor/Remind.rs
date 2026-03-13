use std::collections::HashMap;

use crate::{
    components::{
        drag::{Draggable, Position},
        Note,
    },
    contexts::{auth::AuthContext, workspace::WorkspaceContext},
};
use dioxus::{html::input_data::MouseButton, logger::tracing, prelude::*};
use domain::models::{Note, NoteId, Property, PropertyId};

const _STYLE: Asset = asset!("./style.css");

#[component]
pub fn CorkBoardPage() -> Element {
    let auth_ctx = use_context::<AuthContext>();
    let workspace_ctx = use_context::<WorkspaceContext>();

    let interactive_notes = (workspace_ctx.interactive_notes)();

    let notes: HashMap<NoteId, Note> = if let Some(res) = (auth_ctx.notes())() {
        match res {
            Ok(notes) => notes
                .into_iter()
                .map(|note| (note.id.clone(), note))
                .collect(),
            Err(_err) => HashMap::new(),
        }
    } else {
        HashMap::new()
    };

    let _properties: HashMap<PropertyId, Property> = if let Some(res) = (auth_ctx.properties())() {
        match res {
            Ok(properties) => properties
                .into_iter()
                .map(|property| (property.id.clone(), property))
                .collect(),
            Err(_err) => HashMap::new(),
        }
    } else {
        HashMap::new()
    };

    let mut mouse_pan = use_signal(|| false);
    let mut coordenates = use_signal(|| Position { x: 0.0, y: 0.0 });
    let origin_pos = use_signal(|| Position { x: 0.0, y: 0.0 });

    let toggle_pan = move |e: Event<MouseData>| {
        e.stop_propagation();

        let data = e.data();
        match data.trigger_button() {
            Some(MouseButton::Primary) => {
                let inverse = !mouse_pan();
                mouse_pan.set(inverse);

                if inverse {
                    let coordinates = data.coordinates();
                    coordenates.set(Position {
                        x: coordinates.page().x,
                        y: coordinates.page().y,
                    });
                }
            }
            _ => {}
        }
    };

    let active_class = if mouse_pan() { "draggable-active" } else { "" };

    rsx! {
        div {
            class: format!("corkboard-content {}", active_class),
            ondoubleclick: |_| {
                tracing::info!("duplo clique");
            },
            onmousedown: toggle_pan,
            onmouseup: toggle_pan,
            onmousemove: move |e| {
                if mouse_pan() {
                    let data = e.data();
                    let coordinates = data.coordinates();
                    coordenates.set( Position { x:  coordinates.page().x, y: coordinates.page().y } );
                }
            }
        }

        Draggable { in_moving: mouse_pan, elem_pos: origin_pos, coordinates: coordenates(), style: "overflow: visible;",
            for note_id in interactive_notes.keys() {
                if let (Some(note), Some(inote)) = (notes.get(note_id), interactive_notes.get(note_id)) {
                    Note {
                        title: "{note.title}",
                        body: "{note.content}",
                        propertys: vec![],
                        widht: inote.widht,
                        height: inote.height,
                        position: inote.position.clone()
                    }
                }
            }

        }
    }
}
