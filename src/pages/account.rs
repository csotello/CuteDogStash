use crate::components::Post;
use db::*;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub db: Data,
    pub user: Option<User>,
    pub rate: Callback<(u64, String, u8, String)>,
    pub delete_account: Callback<String>,
    pub edit: Callback<u64>,
    pub delete_post: Callback<u64>,
}

#[function_component(Account)]
pub fn account(props: &Props) -> Html {
    match &props.user {
        Some(user) => {
            let author = user.username.clone();
            let props = props.clone();
            let rate = props.rate.clone();
            let delete_account = props.delete_account.clone();
            let delete_post = props.delete_post.clone();
            let edit = props.edit.clone();
            let map_post = |post: &mut db::Post| -> Html {
                let rate = Callback::from(move |(post_id, author, stars, comment)| {
                    rate.emit((post_id, author, stars, comment))
                });
                let edit = Callback::from(move |id: u64| edit.emit(id));
                let delete = Callback::from(move |id: u64| delete_post.emit(id));
                return html! {
                    <Post post={post.clone()} rate={rate} delete={delete} user={props.user.clone()} edit={edit}/>
                };
            };
            let username = author.clone();
            html! {
                <>
                    <br/>
                    <div class="border border-dark account">
                    <p>{"Account"}</p>
                    <p>{"Username:"}{&author}</p>
                    <button onclick={move |_| delete_account.emit(author.clone())} class="btn btn-outline-danger">{"Delete Account"}</button>
                    </div>
                    {for props.db.get_posts(username).iter().map(|post| map_post.clone()(&mut post.clone()))}

                </>
            }
        }
        None => {
            html! {
                <p>{"Sign in to view your account"}</p>
            }
        }
    }
}
