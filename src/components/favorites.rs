use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let mut favorites = use_resource(super::super::backend::list_dogs);

    match favorites.suspend() {
        Ok(dogs) => {
            rsx! {
                div {
                    id: "favorites",
                    div {
                        id: "favorites-container",
                        for (id, url) in dogs().unwrap() {
                            div {
                                key: id,
                                class: "favorite-dog",
                                img {
                                    src: "{url}"
                                }
                                button {
                                    onclick: move |_| async move {
                                        let _ = crate::backend::delete_dog(id).await;
                                        favorites.restart();
                                    },
                                    id: "dog-{id}",
                                    "❌"
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(_) => { rsx!{} }
    }
}
