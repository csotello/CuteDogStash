use yew::prelude::*;
use yew_router::prelude::*;
mod pages;
mod routes;
use crate::pages::Home;

use crate::routes::Route;
//Base App which controls routing
struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    //Match each route to corresponding page
    fn view(&self) -> Html {
        let render = Router::render(move |switch: Route| match switch {
            Route::Home => {
                html! {<Home />}
            }
        });

        html! {
            <>
                <Router<Route, ()> render=render/>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
