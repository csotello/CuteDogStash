use crate::components::Post;
use crate::utils::*;
use crate::Routes;
use db::*;
use yew::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_router::prelude::RouteAgent;
pub enum Msg {
    Rate(u64, String, u8, String),
    SetAuthor(String),
    SetSearch(String),
    DeleteAccount,
    DeletePost(u64),
    Edit(u64),
    None,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub db: Data,
    pub user: Option<User>,
    pub rate: Callback<(u64, String, u8, String)>,
    pub delete_account: Callback<String>,
    pub edit: Callback<u64>,
    pub delete_post: Callback<u64>,
}
pub struct Account {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
    author: String,
    search: String,
}

impl Component for Account {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let author = {
            if let Some(user) = &props.user {
                user.username.clone()
            } else {
                "".to_string()
            }
        };
        let search = author.clone();
        Self {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::None)),
            link,
            props,
            author,
            search,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Rate(id, author, stars, comment) => {
                self.props.rate.emit((id, author, stars, comment));
            }
            Msg::SetAuthor(author) => {
                self.author = author;
            }
            Msg::SetSearch(val) => {
                self.search = val;
            }
            Msg::DeleteAccount => {
                let author = self.author.clone();
                self.props.delete_account.emit(author);
                self.router_agent.send(ChangeRoute(Routes::Home.into()));
            }
            Msg::DeletePost(id) => {
                self.props.delete_post.emit(id);
            }
            Msg::Edit(id) => {
                self.props.edit.emit(id);
            }
            Msg::None => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if let Some(user) = &self.props.user {
            // Create each post tag
            let map_post = |post: &db::Post| {
                let rate = self.link.callback(|(post_id, author, stars, comment)| {
                    Msg::Rate(post_id, author, stars, comment)
                });
                let delete = self.link.callback(|id| {
                    log("Deleting Post".to_string());
                    Msg::DeletePost(id)
                });
                let edit = self.link.callback(|id| {
                    log("Editing post".to_string());
                    Msg::Edit(id)
                });
                html! {
                    <Post post=post rate=rate delete=delete user=&self.props.user edit=edit/>
                }
            };
            let search = self.search.clone();
            let update_author = self.link.callback(move |_| Msg::SetAuthor(search.clone()));
            let update_search = self
                .link
                .callback(|event: InputData| Msg::SetSearch(event.value));
            let delete_account = self.link.callback(|_| Msg::DeleteAccount);
            html! {
                <>
                    <br/>
                    <div class="border border-dark account">
                    <p>{"Account"}</p>
                    <input type="text" value=&self.search oninput=update_search/>
                    <button onclick=update_author class="btn btn-outline-primary">{"Search"}</button><br/>
                    {if self.author == "" {html!{<p>{"Enter an account to search"}</p>}}
                    else if self.author == user.username{
                        html!{
                            <button onclick=delete_account class="btn btn-outline-danger">{"Delete Account"}</button>
                        }
                    }
                    else {html!{}}}
                    <p>{"Username:"}{&self.author}</p>
                    </div>
                    {for self.props.db.get_posts(self.author.clone()).iter().map(map_post)}

                </>
            }
        } else {
            html! {
                <p>{"Sign in to view your account"}</p>
            }
        }
    }
}
