use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Routes {
    #[to = "/account"]
    Account,
    #[to = "/edit"]
    Edit,
    #[to = "/login"]
    Login,
    #[to = "/signup"]
    SignUp,
    #[to = "/"]
    Home,
}
