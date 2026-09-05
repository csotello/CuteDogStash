use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Routes {
    #[at("/account")]
    Account,
    #[at("/edit")]
    Edit,
    #[at("/login")]
    Login,
    #[at("/signup")]
    SignUp,
    #[at("/post")]
    Post,
    #[at("/update")]
    UpdateAccount,
    #[at("/")]
    Home,
}
