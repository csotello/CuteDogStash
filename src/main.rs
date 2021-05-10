#![recursion_limit="256"]
use yew::prelude::*;
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


pub enum Msg {
    SignUp(String, String),
    ChangeRoute(Route),
    Login(String, String),
}
//Base App which controls routing
struct App {
    link: ComponentLink<Self>,
    db: Data,           //Database
    user: Option<User>, //Current user
    error: bool,
    route: Option<Routes>,                     //Current Route
    router_agent: Box<dyn Bridge<RouteAgent>>, //RouterAgent to switch routes
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::ChangeRoute));
        let route_service: RouteService = RouteService::new();
        let route = route_service.get_route();
        Self {
            link,
            db: Data::default(),
            user: None,
            error: false,
            route: Routes::switch(route),
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SignUp(username, password) => {
                log("Creating Account:".to_string());
                log(format!("username:{}", username.clone()));
                log(format!("password:{}", password.clone()));
                self.db.create_user(username, password);
            }
            Msg::ChangeRoute(route) => {
                self.route = Routes::switch(route);
            }
            Msg::Login(username, password) => match self.db.login(username, password) {
                Some(user) => {
                    log("Found Account:".to_string());
                    log(format!("username:{}", user.username.clone()));
                    log(format!("password:{}", user.password.clone()));
                    self.user = Some(user);
                    self.error = false;
                }
                None => {
                    self.error = true;
                }
            },
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
        html! {
            <>
                <Navbar user=&self.user/>
                {
                    if let Some(route) = &self.route{
                        match route {
                            Routes::Home =>  html! {<Home error=&self.error db=&self.db/>},
                            Routes::Account => html! {<Account />},
                            Routes::Login => html! {<Login callback=login/>},
                            Routes::SignUp => html! {<SignUp callback=signup/>},
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
