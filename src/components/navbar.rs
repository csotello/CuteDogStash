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
        // <Navbar user={props.user.clone()} logout={props.logout.clone()}/>
        <>
            <nav>
            {
                match &props.user {
                    Some(_user) => html! {<>
                        <ul>
                <li>
                    <Link<Routes> to={Routes::Home}>
                    { "Home" }
                    </Link<Routes>>
                </li>
                <li>
                    <Link<Routes> to={Routes::Account}>
                    { "Account" }
                    </Link<Routes>>
                </li>
                <li>
                    <Link<Routes> to={Routes::UpdateAccount}>
                    { "Update Account" }
                    </Link<Routes>>
                </li>
                <li>
                    <Link<Routes> to={Routes::Post}>
                    { "Create Post" }
                    </Link<Routes>>
                </li>
                <li>
                    <a onclick={props.logout.clone()} style="color: rgb(85, 26, 139); cursor: pointer;text-decoration-line: underline;">{"Logout"}</a>
                </li>
        </ul>
                        </>},
                    None => html! {
                        <ul>
                            <li>
                                <Link<Routes> to={Routes::Home}>
                                { "Home" }
                                </Link<Routes>>
                            </li>
                            <li>
                                <Link<Routes> to={Routes::SignUp}>
                                { "Signup" }
                                </Link<Routes>>
                            </li>
                            <li>
                                <Link<Routes> to={Routes::Login}>
                                { "Login" }
                                </Link<Routes>>
                            </li>
                        </ul>
                    },
                }
            }
            </nav>
            <br/><br/>
            </>
    }
}
