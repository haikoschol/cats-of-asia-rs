use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

use crate::api::Image;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn bindPopup(circle: JsValue, image: JsValue, map: Map);

    #[wasm_bindgen]
    pub fn removeMap(map: Map);

    type L;

    #[wasm_bindgen(static_method_of = L)]
    pub fn map(element: &str) -> Map;

    #[wasm_bindgen(static_method_of = L)]
    pub fn tileLayer(urlTemplate: &str, options: JsValue) -> TileLayer;

    #[wasm_bindgen(static_method_of = L)]
    pub fn circle(center: JsValue, options: JsValue) -> Circle;

    type TileLayer;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &TileLayer, map: Map);

    #[derive(Clone, Debug)]
    pub type Map;

    #[wasm_bindgen(method)]
    fn setView(this: &Map, center: JsValue, zoom: u8);

    #[wasm_bindgen(method)]
    fn remove(this: &Map);

    type Circle;

    #[wasm_bindgen(method)]
    pub fn bindPopup(this: &Circle, cb: &js_sys::Function);

    #[wasm_bindgen(method)]
    pub fn addTo(this: &Circle, map: Map);
}

#[derive(Serialize, Deserialize)]
struct MapOptions {
    #[serde(rename="maxZoom")]
    pub max_zoom: u8,
    pub id: String,
    #[serde(rename="tileSize")]
    pub tile_size: u32,
    #[serde(rename="zoomOffset")]
    pub zoom_offset: i8,
}

#[derive(Serialize, Deserialize)]
struct CircleOptions {
    pub radius: u8,
//    pub color: String,
}

#[derive(Clone)]
pub struct LeafletMap {
    map: Map,
}

impl LeafletMap {
    pub fn new(access_token: &str, max_zoom: u8) -> LeafletMap {
        let url = format!("https://api.mapbox.com/styles/v1/{{id}}/tiles/{{z}}/{{x}}/{{y}}?access_token={access_token}");
        let map = L::map("cattos");

        let options = MapOptions{
            max_zoom,
            id: "mapbox/streets-v11".into(),
            tile_size: 512,
            zoom_offset: -1,
        };

        let options = to_value(&options).expect("static value to convert successfully");
        let tile_layer = L::tileLayer(&url, options);
        tile_layer.addTo(map.clone());

        LeafletMap{map}
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }

    pub fn set_view(&self, latitude: f64, longitude: f64, zoom_level: u8) {
        let center = vec![latitude, longitude];
        let center = to_value(&center).expect("f64 to convert successfully");
        self.map.setView(center, zoom_level);
    }

    pub fn add_marker(
        &self,
        image: &Image,
        radius: u8,
        ) {
        let center = vec![image.latitude, image.longitude];
        let center = to_value(&center).expect("f64 to convert successfully");

        let options = CircleOptions{
            radius,
        };

        let options = to_value(&options).expect("static value to convert successfully");
        let circle = L::circle(center, options);
        let image = to_value(image).expect("Image struct to convert to a JS object");

        bindPopup(circle.clone(), image, self.map.clone());
        circle.addTo(self.map.clone());
    }

    pub fn remove(&self) {
        removeMap(self.map.clone())
    }
}
