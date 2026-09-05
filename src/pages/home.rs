use crate::components::Post;
use crate::utils::*;
use db::*;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub error: String,
    pub db: Data,
    pub user: Option<User>,
    pub rate: Callback<(u64, String, u8, String)>,
    pub edit: Callback<u64>,
    pub delete: Callback<u64>,
}
#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    let props = props.clone();
    let map_post = |post: &db::Post| {
        let rate = {
            let rate = props.rate.clone();
            Callback::from(move |(post_id, author, stars, comment)| {
                rate.emit((post_id, author, stars, comment))
            })
        };
        let delete = {
            let delete = props.delete.clone();
            Callback::from(move |id| {
                log("Deleting Post".to_string());
                delete.emit(id)
            })
        };
        let edit = {
            let edit = props.edit.clone();
            Callback::from(move |id| {
                log("Editing post".to_string());
                edit.emit(id)
            })
        };
        html! {
            <Post post={post.clone()} rate={rate} delete={delete} user={props.user.clone()} edit={edit}/>
        }
    };
    html! {
                <div>
                <br/>
                    {if props.error.parse().unwrap_or_else(|_| false) {html! {<p>{"Error"}</p>}} else {html!{}}}
                    {for props.db.posts.iter().map(map_post)}
                </div>
    }
}
