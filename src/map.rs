use std::collections::BTreeMap;

use leptos::*;
use leptos_meta::*;
use web_sys::MouseEvent;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};

use crate::api::{Image, ImagesResource};
use crate::leaflet::LeafletMap;

const ACCESS_TOKEN: &str = "bob";
const MAX_ZOOM: u8 = 22;

#[derive(Copy, Clone)]
struct MapResource(Resource<(), LeafletMap>);

#[component]
pub fn MapView() -> impl IntoView {
    provide_meta_context();

    let map = create_local_resource(
        || (),
        |_| async move {
            LeafletMap::new(ACCESS_TOKEN, MAX_ZOOM)
        });

    provide_context(MapResource(map));
    on_cleanup(move || {
        if let Some(map) = map() {
            map.remove();
        }
    });

    view! {
        <Link rel="stylesheet" href="leaflet.css"/>
        <script src="leaflet.js"></script>
        <script src="map.js"></script>
        <Places/>
        <Map/>
    }
}

#[component]
fn Map() -> impl IntoView {
    let images = use_context::<ImagesResource>().expect("it to have been loaded in App");
    let map = use_context::<MapResource>().expect("it to have been created in MapView");

    create_effect(move |_| {
        if let Some(images) = images.0.get() {
            if let Some(map) = map.0.get() {
                map.set_view(images[0].latitude, images[0].longitude, 15);

                images.iter()
                    .for_each(|img| {
                        map.add_marker(&img, 12);
                    });
            }
        }
    });

    view! {
        <>
//            <PopupTemplate/>
            <div id="cattos"></div>
        </>
    }
}

#[component]
fn Places() -> impl IntoView {
    let images = use_context::<ImagesResource>().expect("it to have been loaded in App");
    let map = use_context::<MapResource>().expect("it to have been created in MapView");

    let coords_by_city = move || {
        let mut cbc = BTreeMap::<String, (f64, f64)>::new();

        if let Some(images) = images.0.get() {
            for img in images.iter() {
                cbc.insert(format_location(img), [img.latitude, img.longitude].into());
            }
        }
        cbc
    };

    let details = create_local_resource(
        || (),
        |_| async move {
            let document = window().document().expect("should have a Document");
            document.get_element_by_id("places")
        }
    );

    let make_on_click = move |latitude, longitude| {
        move |_| {
            if let Some(map) = map.0.get() {
                map.set_view(latitude, longitude, 15);
            }

            if let Some(details) = details() {
                if let Some(details) = details {
                    details.remove_attribute("open").expect("should remove 'open' attribute");
                }
            }
        }
    };

    view! {
        <details id="places" role="list" style="z-index: 1337">
            <summary aria-haspopup="listbox" role="button">
                "Places"
            </summary>
            <ul role="listbox">
                {move || {
                    coords_by_city()
                        .into_iter()
                        .map(|(k, v)| {
                            view! { <PlaceItem label=k on_click=make_on_click(v.0, v.1)/> }
                        })
                        .collect::<Vec<_>>()
                }}
            </ul>
        </details>
    }
}

#[component]
fn PlaceItem(
    label: String,
    #[prop(into)]
    on_click: Callback<MouseEvent>,
    ) -> impl IntoView {
    view! {
        <li>
            <a on:click=on_click>{label}</a>
        </li>
    }    
}

// For some reason this only works "once". When navigating to Favorites and back the renderPopup()
// JS function fails because the clone of the <template> content does not have any of the elements
// any more.
// clone.querySelector('#link') returns null and accessing that fails with:
//Uncaught TypeError: Cannot set properties of null (setting 'id')
//    at renderPopup (map.js:18:13)
//    at e._content (VM299 map.js:7:28)
//    at e._updateContent (DivOverlay.js:277:62)
//    at e.update (DivOverlay.js:187:8)
//    at e.onAdd (DivOverlay.js:113:8)
//    at e.onAdd (Popup.js:135:30)
//    at e._layerAdd (Layer.js:114:8)
//    at e.whenReady (Map.js:1477:13)
//    at e.addLayer (Layer.js:172:8)
//    at e.openOn (DivOverlay.js:63:8)
#[allow(unused)]
#[component]
fn PopupTemplate() -> impl IntoView {
    view! {
        <template id="popup">
            <div>
                <a id="link" href=""><img id="catto" src="" alt="a photo taken in Asia, showing one or more cats"/></a>
                <div id="footer" class="popup-footer">
                    <div id="description"></div>
                    <button id="favorite">
                        <img id="icon" src="" class="icon"/>
                    </button>
                    <button id="share" hidden="true">
                        <img src="share.svg" class="icon"/>
                    </button>
                </div>
            </div>
        </template>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn formatLocation(image: JsValue) -> JsValue;
}

pub fn format_location(image: &Image) -> String {
    let image = to_value(image).expect("Image struct to convert successfully");
    let loc = formatLocation(image);
    from_value(loc).expect("it to convert to a String successfully")
}
