mod backend;
mod guide_router;
mod components;

use crate::guide_router::App;

fn main() {
    dioxus::launch(App);
}
