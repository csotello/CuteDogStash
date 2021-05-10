use crate::Routes;
use db::*;
use yew::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_router::prelude::RouteAgent;
pub enum Msg {
    SetUsername(String),
    SetPassword(String),
    None,
    Submit,
}

pub struct SignUp {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    password: String,
    username: String,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<(String, String)>,
}

impl Component for SignUp {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SignUp {
            password: String::new(),
            username: String::new(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::None)),
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetUsername(username) => {
                self.username = username;
            }
            Msg::SetPassword(password) => {
                self.password = password;
            }
            Msg::Submit => {
                self.props
                    .callback
                    .emit((self.username.clone(), self.password.clone()));
                self.router_agent.send(ChangeRoute(Routes::Home.into()));
            }
            _ => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let update_username = self
            .link
            .callback(|event: InputData| Msg::SetUsername(event.value));
        let update_password = self
            .link
            .callback(|event: InputData| Msg::SetPassword(event.value));
        let onsubmit = self.link.callback(|event: FocusEvent| {
            event.prevent_default();
            Msg::Submit
        });
        html! {
            <>
                <form onsubmit=onsubmit>
                    <fieldset>
                    <label>{"Username:"}</label>
                    <input type="text" pattern="[A-Za-z0-9]{1,10}"
                        value=&self.username
                        oninput=update_username/>
                    <label>{"Password:"}</label>
                    <input type="password"
                        value=&self.password
                        oninput=update_password/>
                    <button type="submit">{"SignUp"}</button>
                    </fieldset>
                </form>
            </>
        }
    }
}
