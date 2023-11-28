use leptos::*;
use web_sys::MouseEvent;
use gloo_storage::{Storage, LocalStorage};

use crate::api::ImagesResource;

#[component]
pub fn Favorites() -> impl IntoView {
    let reload = create_rw_signal(0);
    
    let favorites = create_local_resource(
        reload,
        |_| async move {load_favorites()}
    );

    view! {
        <>
            <script src="map.js"></script>
    
            <Show
                when=move || { favorites().is_some() && has_favorites() }
                fallback=NoFavorites
            >
                <For
                    each=move || favorites().unwrap().into_iter()
                    key=move |hash| hash.clone()
    
                    children=move |hash| {
                        view! {
                            <Favorite
                                hash=hash.clone()
                                on_delete=move |_| {
                                    remove_favorite(&hash);
                                    reload.set(reload.get()+1);
                                }
                            />
                        }
                    }
                />
            </Show>
        </>
    }
}

#[component]
fn Favorite(
    hash: String,
    #[prop(into)]
    on_delete: Callback<MouseEvent>
) -> impl IntoView {
    let images = use_context::<ImagesResource>().expect("it to have been loaded in App");

    // this all seems very wrong
    let image = (|| {
        if let Some(images) = images.0.get() {
            for img in images {
                if img.sha256 == hash {
                    return Some(img.clone());
                }
            }
        }
        None
    })();

    // <Show> hates me
    if let Some(image) = image {
        let alt = format!("photo #{} showing one or more cats", image.id);
        let url_medium = image.url_medium.clone();
        let url_large = image.url_large.clone();

        view! {
            <div class="fav-card">
                <article>
                    <a href={url_large}><img src={url_medium} alt={alt} /></a>
                </article>
                <footer>
                    <button on:click=on_delete>"Remove"</button>
                </footer>
            </div>
        }.into_view()
    } else {
        view! {
            <></>
        }.into_view()
    }
}

#[component]
pub fn NoFavorites() -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: center;">
            "You don't have any favorites yet. Find some on the"<a href="/">"map"</a>"!"
        </div>
    }
}

fn has_favorites() -> bool {
    !load_favorites().is_empty()
}

fn load_favorites() -> Vec<String> {
    LocalStorage::get("favorites").unwrap_or_default()
}

fn remove_favorite(hash: &str) {
    let favs: Vec<String> = load_favorites().into_iter().filter(|h| h != hash).collect();
    LocalStorage::set("favorites", favs).ok();
}
