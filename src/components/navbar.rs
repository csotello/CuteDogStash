use crate::routes::Routes;
use db::User;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user: Option<User>,
    pub logout: Callback<MouseEvent>,
}
#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    html! {
        <nav class="site-nav" aria-label="Main navigation">
            <div class="nav-content">
                <Link<Routes> to={Routes::Home} classes="nav-brand">{"CuteDogStash"}</Link<Routes>>
            {
                match &props.user {
                    Some(_user) => html! {
                        <ul class="nav-links">
                            <li><Link<Routes> to={Routes::Home} classes="nav-link">{"Home"}</Link<Routes>></li>
                            <li><Link<Routes> to={Routes::Account} classes="nav-link">{"Account"}</Link<Routes>></li>
                            <li><Link<Routes> to={Routes::UpdateAccount} classes="nav-link">{"Settings"}</Link<Routes>></li>
                            <li><Link<Routes> to={Routes::Post} classes="nav-link nav-link-primary">{"Create post"}</Link<Routes>></li>
                            <li><button type="button" onclick={props.logout.clone()} class="nav-link nav-logout">{"Log out"}</button></li>
                        </ul>
                    },
                    None => html! {
                        <ul class="nav-links">
                            <li><Link<Routes> to={Routes::Home} classes="nav-link">{"Home"}</Link<Routes>></li>
                            <li><Link<Routes> to={Routes::SignUp} classes="nav-link">{"Sign up"}</Link<Routes>></li>
                            <li><Link<Routes> to={Routes::Login} classes="nav-link nav-link-primary">{"Log in"}</Link<Routes>></li>
                        </ul>
                    },
                }
            }
            </div>
        </nav>
    }
}
