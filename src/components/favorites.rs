use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let mut favorites = use_server_future(super::super::backend::list_dogs)?;
    let dogs = favorites.suspend()?;

    match dogs() {
        Ok(dogs) => {
            rsx! {
                div {
                    id: "favorites",
                    div {
                        id: "favorites-container",
                        for (id, url) in dogs {
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
        Err(_) => rsx! {},
    }
}
