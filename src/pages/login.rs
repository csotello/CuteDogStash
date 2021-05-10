use crate::Routes;
use yew::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_router::prelude::RouteAgent;
pub enum Msg {
    SetUsername(String),
    SetPassword(String),
    None,
    Submit,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<(String, String)>,
}
pub struct Login {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    username: String,
    password: String,
    props: Props,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::None)),
            link,
            username: String::new(),
            password: String::new(),
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
                        <button type="submit">{"Login"}</button>
                    </fieldset>
                </form>
            </>
        }
    }
}
