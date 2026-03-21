use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn random() -> Self {
        let (max_x, max_y) = {
            if let Some(window) = web_sys::window() {
                let width = window.inner_height().unwrap().as_f64().unwrap();
                let height = window.inner_height().unwrap().as_f64().unwrap();

                (width, height)
            } else {
                (1280.0, 720.0)
            }
        };

        let x = rand::random_range(0.0..max_x);
        let y = rand::random_range(0.0..max_y);

        Self { x, y }
    }
}
