use crate::Routes;
use db::*;
use yew::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_router::prelude::RouteAgent;
pub enum Msg {
    SetUsername(String),
    SetPassword(String),
    InvalidInput,
    None,
    Submit,
}

pub struct UpdateAccount {
    password: String,
    username: String,
    error: bool,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub update: Callback<(u64, String, String)>,
    pub user: Option<User>,
    pub db: Data,
}

impl Component for UpdateAccount {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        UpdateAccount {
            password: String::new(),
            username: String::new(),
            error: false,
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
                if self.props.db.check_username(self.username.clone()) {
                    let user = self.props.user.as_ref().unwrap();
                    self.props
                        .update
                        .emit((user.id, self.username.clone(), self.password.clone()));
                    self.router_agent.send(ChangeRoute(Routes::Home.into()));
                } else {
                    self.error = true;
                }
            }
            Msg::InvalidInput => {
                self.error = true;
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
        let oninvalid = self.link.callback(|_| Msg::InvalidInput);
        html! {
            <div class="border border-dark">
                <br/>
                <p>{"Update Account Info"}</p>
                {if self.error {html!{<p>{"Invalid username or password\nUsername cannot contain special characters"}</p>}} else {html!{}}}
                <form onsubmit=onsubmit>
                    <fieldset>
                    <label>{"Username:"}</label>
                    <input type="text" pattern="[A-Za-z0-9]{1,20}"
                        value=&self.username required=true
                        oninput=update_username
                        oninvalid=oninvalid/>
                    <label>{"Password:"}</label>
                    <input type="password"
                        value=&self.password required=true
                        oninput=update_password/>
                    <button type="submit" class="btn btn-primary">{"SignUp"}</button>
                    </fieldset>
                </form>
            </div>
        }
    }
}
