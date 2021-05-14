#![recursion_limit = "256"] //Increase limit for rendering pages
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
use crate::pages::{Account, Edit, Home, Login, Post, SignUp};
use crate::routes::Routes;
use components::*;
use db::*;
const KEY: &'static str = "CuteDogStash_KEY";
pub enum Msg {
    SignUp(String, String),
    SetRoute(Route),
    Login(String, String),
    CreatePost(String, String, String),
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
                    self.user = Some(User {
                        id: user.id,
                        username: user.username,
                        password: "".to_string(),
                    });
                    self.error = false;
                }
                None => {
                    self.error = true;
                }
            },
            Msg::Logout => {
                self.user = None;
            }
            Msg::CreatePost(author, description, image) => {
                self.db.create_post(author, description, image);
                self.storage.store(KEY, Json(&self.db));
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
        let create_post = self
            .link
            .callback(|(author, description, image)| Msg::CreatePost(author, description, image));
        html! {
            <>
                <Navbar user=&self.user logout=logout/>
                {
                    self.map_route(self.route.as_ref(), self.user.as_ref())
                    // if let Some(route) = &self.route{
                    //     match route {
                    //         Routes::Home =>  html! {<Home error=&self.error db=&self.db/>},
                    //         Routes::Account => html! {<Account />},
                    //         Routes::Login => html! {<Login callback=login/>},
                    //         Routes::SignUp => html! {<SignUp callback=signup db=&self.db/>},
                    //         Routes::Edit => html! {<Edit />},
                    //         Routes::Post => html! {<Post db=&self.db callback=create_post user=&self.user/>},
                    //     }
                    // }
                    // else{
                    //     html!{"No page found"}
                    // }
                }
            </>
        }
    }
}

impl App {
    fn map_route(&self, route: Option<&Routes>, user: Option<&User>) -> Html {
        match user {
            Some(user) => {
                let create_post = self.link.callback(|(author, description, image)| {
                    Msg::CreatePost(author, description, image)
                });
                if let Some(route) = &route {
                    match route {
                        Routes::Home => html! {<Home error=&self.error db=&self.db/>},
                        Routes::Account => html! {<Account />},
                        Routes::Edit => html! {<Edit />},
                        Routes::Post => {
                            html! {<Post db=&self.db callback=create_post user=&self.user/>}
                        }
                        _ => html! {<p>{"Page not needed"}</p>},
                    }
                } else {
                    html! {<p>{"Error"}</p>}
                }
            }
            None => {
                let signup = self
                    .link
                    .callback(|(username, password)| Msg::SignUp(username, password));
                let login = self
                    .link
                    .callback(|(username, password)| Msg::Login(username, password));
                if let Some(route) = &route {
                    match route {
                        Routes::Home => html! {<Home error=&self.error db=&self.db/>},
                        Routes::Login => html! {<Login callback=login/>},
                        Routes::SignUp => html! {<SignUp callback=signup db=&self.db/>},
                        _ => html! {<p>{"Login to access this page"}</p>},
                    }
                } else {
                    html! {<p>{"Error"}</p>}
                }
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
