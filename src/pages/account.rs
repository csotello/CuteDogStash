use crate::{components::Post, Routes};
use db::*;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

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
    let navigator = use_navigator().unwrap();
    match &props.user {
        Some(user) => {
            let author = user.username.clone();
            let props = props.clone();
            let rate = props.rate.clone();
            let delete_account = props.delete_account.clone();
            let delete_post = props.delete_post.clone();
            let edit = props.edit.clone();
            let map_post = |post: &db::Post| -> Html {
                let rate = rate.clone();
                let edit = edit.clone();
                let delete_post = delete_post.clone();
                let rate = Callback::from(move |(post_id, author, stars, comment)| {
                    rate.emit((post_id, author, stars, comment))
                });
                let edit = Callback::from(move |id: u64| edit.emit(id));
                let delete = Callback::from(move |id: u64| delete_post.emit(id));
                html! {
                    <Post post={post.clone()} rate={rate} delete={delete} user={props.user.clone()} edit={edit}/>
                }
            };
            let update_account = {
                let navigator = navigator.clone();
                Callback::from(move |_| navigator.push(&Routes::UpdateAccount))
            };
            let username = author.clone();
            let posts = props.db.get_posts(username.clone());
            let post_section: Html = if posts.is_empty() {
                html! {
                    <section class="home-empty"><h2>{"No posts yet"}</h2><p>{"Your shared dog moments will appear here."}</p></section>
                }
            } else {
                html! {
                    <section class="post-feed" aria-label="Your posts">{for posts.iter().map(map_post)}</section>
                }
            };
            let delete_username = author.clone();
            return html! {
                <main class="home-page account-page">
                    <section class="post account-summary">
                        <div class="post-content">
                            <header class="post-header">
                                <span class="post-eyebrow">{"Your account"}</span>
                                <h1 class="post-author">{&author}</h1>
                            </header>
                            <p class="post-description">{"Manage your profile and the dog moments you have shared."}</p>
                            <div class="post-actions">
                                <button onclick={update_account} class="btn btn-primary">{"Update account"}</button>
                                <button onclick={move |_| delete_account.emit(delete_username.clone())} class="btn btn-outline-danger">{"Delete account"}</button>
                            </div>
                        </div>
                    </section>
                    <header class="feed-header account-section-header">
                        <span class="post-eyebrow">{"Your gallery"}</span>
                        <h2>{"Your posts"}</h2>
                    </header>
                    {post_section}
                </main>
            };
        }
        None => {
            html! {
                <main class="home-page"><section class="home-empty"><h2>{"Sign in to view your account"}</h2></section></main>
            }
        }
    }
}
