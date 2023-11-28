use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};
use crate::api::{ImagesResource, fetch_images};
use crate::map::MapView;
use crate::favorites::Favorites;

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
                    <Route path="/" view=MapView/>
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
