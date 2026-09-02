#![recursion_limit = "512"]

mod app;
mod components;
mod pages;
mod routes;
mod utils;

use yew::function_component;
use yew::prelude::*;
use yew_router::BrowserRouter;

use crate::app::App;
pub(crate) use crate::routes::Routes;

#[function_component(Main)]
fn main_page() -> Html {
    html! {
        <BrowserRouter>
            <App/>
        </BrowserRouter>
    }
}
fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<Main>::new().render();
}
