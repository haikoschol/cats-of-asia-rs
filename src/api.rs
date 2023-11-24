use serde::{Deserialize, Serialize};

const URL: &str = "https://catsof.asia/images";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Image {
    pub id: usize,
    #[serde(rename= "urlLarge")]
    pub url_large: String,
    #[serde(rename = "urlMedium")]
    pub url_medium: String,
    #[serde(rename = "urlSmall")]
    pub url_small: String,
    pub sha256: String,
    pub timestamp: String,
    pub latitude: f64,
    pub longitude: f64,
    pub city: String,
    pub country: String,
}

#[cfg(feature = "ssr")]
pub async fn fetch_images() -> Vec<Image> {
    let empty = vec![];
    
    let response = reqwest::get(URL).await;
    if response.is_err() {
        log::error!("error fetching images on the server: {:?}", response.err());
        return empty;
    }
    
    let images = response.unwrap().json::<Vec<Image>>().await;
    match images {
        Ok(images) => images,
        Err(e) => {
            log::error!("error parsing api response on the server: {:?}", e);
            empty
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub async fn fetch_images() -> Vec<Image> {
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    // abort in-flight requests if e.g., we've navigated away from this page
    leptos::on_cleanup(move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    let response = gloo_net::http::Request::get(URL)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| log::error!("{e}"));

    if response.is_err() {
        return vec![];
    }

    let response = response.unwrap();
    
    if !response.ok() {
        log::error!("api response ain't ok: {:?}", response);
        return vec![];
    }

    let images = response.json::<Vec<Image>>().await;    
    if images.is_err() {
        log::error!("failed to parse api response as json: {:?}", images.err());
        return vec![];
    }
    
    images.unwrap()
}