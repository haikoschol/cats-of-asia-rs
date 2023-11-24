use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn init_map();

    #[wasm_bindgen]
    pub fn remove_map();

    #[wasm_bindgen]
    pub fn set_view(latitude: f64, longitude: f64, zoom_level: u8);

    #[wasm_bindgen]
    pub fn add_marker(latitude: f64, longitude: f64, radius: u8, randomized: bool);

    //    type L;

    //    #[wasm_bindgen(static_method_of = L)]
    //    pub fn tileLayer(url_template: &str, options: &JsValue) -> TileLayer;
    //
    //    #[wasm_bindgen(static_method_of = L)]
    //    pub fn map(element: &str, options: &JsValue) -> Map;
    //
    //    type TileLayer;
    //    type Map;
    //
    //    #[wasm_bindgen(method)]
    //    fn setView(this: &Map, center: &JsValue, zoom: i32);
}
