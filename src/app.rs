use std::collections::BTreeMap;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::MouseEvent;

use crate::error_template::{AppError, ErrorTemplate};
use crate::api::{Image, fetch_images};
use crate::leaflet::{init_map, remove_map, set_view, add_marker};

#[derive(Copy, Clone)]
struct ImagesResource(Resource<(), Vec<Image>>);

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let images = create_local_resource(
        || (),
        |_| async move {
            fetch_images().await
        },
    );

    provide_context(ImagesResource(images));
    
    view! {
        <Title text="Cats of Asia"/>
        <Stylesheet id="leptos" href="/pkg/cats-of-asia.css"/>
        <Link rel="icon" href="apple-touch-icon.png"/>
        <Link rel="apple-touch-startup-image" href="apple-touch-icon.png"/>
        <Link rel="stylesheet" href="pico.min.css"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="container-fluid">
                <NavBar/>
                <Routes>
                    <Route path="" view=MapView/>
                    <Route path="/favorites" view=Favorites/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <li>
                    <strong>Cats Of Asia</strong>
                </li>
                <li>
                    <a href="/">Map</a>
                </li>
                <li>
                    <a href="/favorites">Favorites</a>
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn Map() -> impl IntoView {
    let images = use_context::<ImagesResource>().expect("it to have been loaded in App");
    create_effect(move |_| init_map());
    on_cleanup(move || remove_map());
    
    create_effect(move |_| {
        if let Some(images) = images.0.get() {
            set_view(images[0].latitude, images[0].longitude, 15);
            images.iter().for_each(|img| add_marker(img.latitude, img.longitude, 12, false));
        }
    });
    
    view! { <div id="cattos"></div> }
}

#[component]
fn Places() -> impl IntoView {
    let images = use_context::<ImagesResource>().expect("it to have been loaded in App");

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
            set_view(latitude, longitude, 15);
            
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
                Places
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

fn format_location(image: &Image) -> String {
    match image.city.len() {
        0 => image.country.clone(),
        _ => format!("{}, {}", image.city, image.country)
    }
}

#[component]
fn MapView() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link rel="stylesheet" href="leaflet.css"/>
        <script src="leaflet.js"></script>
        <script src="map.js"></script>
        <Places/>
        <Map/>
    }
}

#[component]
fn Favorites() -> impl IntoView {
    view! { <h1>TODO</h1> }
}
