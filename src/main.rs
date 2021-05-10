#![recursion_limit = "256"]
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;
use yew_router::prelude::*;
mod utils;
use utils::*;
mod components;
mod pages;
mod routes;
use crate::pages::{Account, Edit, Home, Login, SignUp};
use crate::routes::Routes;
use components::*;
use db::*;
const KEY: &'static str = "CuteDogStash_KEY";
pub enum Msg {
    SignUp(String, String),
    SetRoute(Route),
    Login(String, String),
    Logout,
}
//Base App which controls routing
struct App {
    link: ComponentLink<Self>,
    db: Data,           //Database
    user: Option<User>, //Current user
    error: bool,
    storage: StorageService, //StorageService to persist in localstorage
    route: Option<Routes>,   //Current Route
    router_agent: Box<dyn Bridge<RouteAgent>>, //RouterAgent to switch routes
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::SetRoute));
        let route_service: RouteService = RouteService::new();
        let route = route_service.get_route();
        let storage = StorageService::new(Area::Local).unwrap();
        let Json(data) = storage.restore(KEY);
        let db = data.unwrap_or_else(|_| Data::default());
        Self {
            link,
            db: db,
            user: None,
            error: false,
            route: Routes::switch(route),
            storage,
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SignUp(username, password) => {
                self.db.create_user(username, password);
                self.storage.store(KEY, Json(&self.db));
            }
            Msg::SetRoute(route) => {
                self.route = Routes::switch(route);
            }
            Msg::Login(username, password) => match self.db.login(username, password) {
                Some(user) => {
                    self.user = Some(user);
                    self.error = false;
                }
                None => {
                    self.error = true;
                }
            },
            Msg::Logout => {
                self.user = None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false //No props to change
    }

    //Match each route to corresponding page
    fn view(&self) -> Html {
        let signup = self
            .link
            .callback(|(username, password)| Msg::SignUp(username, password));
        let login = self
            .link
            .callback(|(username, password)| Msg::Login(username, password));
        let logout = self.link.callback(|_| Msg::Logout);
        html! {
            <>
                <Navbar user=&self.user logout=logout/>
                {
                    if let Some(route) = &self.route{
                        match route {
                            Routes::Home =>  html! {<Home error=&self.error db=&self.db/>},
                            Routes::Account => html! {<Account />},
                            Routes::Login => html! {<Login callback=login/>},
                            Routes::SignUp => html! {<SignUp callback=signup db=&self.db/>},
                            Routes::Edit => html! {<Edit />}
                        }
                    }
                    else{
                        html!{"No page found"}
                    }
                }
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
