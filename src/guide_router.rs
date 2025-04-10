use crate::components::NavBar;
use dioxus::prelude::*;
use crate::components::Favorites;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    let save = move |_| async move {
        let current = img_src.cloned().unwrap();
        img_src.restart();
        _ = super::backend::save_dog(current).await;
    };

    rsx! {
        div { id: "dogview",
            img { src: "{img_src.cloned().unwrap_or_default()}" }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}
