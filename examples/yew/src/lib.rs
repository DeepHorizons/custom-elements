mod component;

use component::Model;
use component::Msg;
use custom_elements::{inject_stylesheet, CustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::prelude::*;

struct ComponentWrapper {
    app: Option<AppHandle<Model>>,
}

impl ComponentWrapper {
    fn new() -> Self {
        Self { app: None }
    }
}

impl CustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        let app = yew::Renderer::<Model>::with_root(this.clone().unchecked_into()).render();
        self.app = Some(app);

        inject_stylesheet(&this, "/component_style.css");
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["value"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        match name.as_str() {
            "value" => {
                if let Some(value) = new_value {
                    if let Ok(value) = value.parse::<i64>() {
                        if let Some(app) = &self.app {
                            app.send_message(Msg::Set(value));
                        }
                    }
                }
            }
            _ => (),
        };
    }
}

impl Default for ComponentWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    ComponentWrapper::define("ce-yew");
}
