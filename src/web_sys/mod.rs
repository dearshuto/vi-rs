use std::collections::HashMap;

use wasm_bindgen::JsCast;

pub struct Instance {
    display_table: HashMap<DisplayId, Display>,
}

impl Instance {
    pub fn new() -> Self {
        Self {
            display_table: HashMap::new(),
        }
    }

    pub fn create_display(&mut self) -> DisplayId {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        let id = DisplayId { id: canvas.id() };
        let display = Display { canvas, context };
        self.display_table.insert(id.clone(), display);
        id
    }

    pub fn try_get_display(&self, id: &DisplayId) -> Option<&Display> {
        self.display_table.get(id)
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct DisplayId {
    id: String,
}

pub struct Display {
    pub canvas: web_sys::HtmlCanvasElement,
    pub context: web_sys::WebGl2RenderingContext,
}
