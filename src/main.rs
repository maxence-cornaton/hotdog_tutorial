mod backend;
mod guide_router;
mod components;

use crate::guide_router::App;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}
