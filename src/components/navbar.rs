use crate::routes::Routes;
use db::User;
use yew::prelude::*;
use yew_router::prelude::*;
pub enum Msg {
    Logout,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub user: Option<User>,
    pub logout: Callback<()>,
}

pub struct Navbar {
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
        match msg {
            Msg::Logout => {
                self.props.logout.emit(());
            }
        }
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
            <br/><br/>
            </>
        }
    }
}
impl Navbar {
    /// Show links to logged out user
    fn login_links(&self) -> Html {
        html! {
            <ul >
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
    /// Show links for logged in user
    fn user_links(&self) -> Html {
        let logout = self.link.callback(move |e: MouseEvent| {
            e.prevent_default();
            Msg::Logout
        });
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
                <li>
                    <RouterAnchor<Routes> route=Routes::UpdateAccount>
                    { "Update Account" }
                    </RouterAnchor<Routes>>
                </li>
                <li>
                    <RouterAnchor<Routes> route=Routes::Post>
                    { "Create Post" }
                    </RouterAnchor<Routes>>
                </li>
                <li>
                    <a onclick=logout style="color: rgb(85, 26, 139); cursor: pointer;text-decoration-line: underline;">{"Logout"}</a>
                </li>
        </ul>
        }
    }
}
