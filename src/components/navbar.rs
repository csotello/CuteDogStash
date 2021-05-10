use crate::routes::Routes;
use db::User;
use yew::prelude::*;
use yew_router::prelude::*;
pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub user: Option<User>,
}

pub struct Navbar {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
            <nav>
            {
                match &self.props.user {
                    Some(user) => self.user_links(),
                    None => self.login_links()
                }
            }
            </nav>
            </>
        }
    }
}
impl Navbar {
    fn login_links(&self) -> Html {
        html! {
            <ul>
                <li>
                    <RouterAnchor<Routes> route=Routes::Home>
                    { "Home" }
                    </RouterAnchor<Routes>>
                </li>
                <li>
                    <RouterAnchor<Routes> route=Routes::SignUp>
                    { "Signup" }
                    </RouterAnchor<Routes>>
                </li>
                <li>
                    <RouterAnchor<Routes> route=Routes::Login>
                    { "Login" }
                    </RouterAnchor<Routes>>
                </li>
            </ul>
        }
    }
    fn user_links(&self) -> Html {
        html! {
            <ul>
            <li>
            <RouterAnchor<Routes> route=Routes::Home>
                { "Home" }
            </RouterAnchor<Routes>>
            </li>
            <li>
            <RouterAnchor<Routes> route=Routes::Account>
                { "Account" }
            </RouterAnchor<Routes>>
            </li>
        </ul>
        }
    }
}
