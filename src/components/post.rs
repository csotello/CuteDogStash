use db::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub post: db::Post,
    pub rate: Callback<(u64, String, u8, String), ()>,
    pub edit: Callback<u64, ()>,
    pub delete: Callback<u64, ()>,
    pub user: Option<User>,
}

#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    let comment = use_state(String::new);
    let rating = use_state(|| 0_u8);
    let error = use_state(|| false);

    let update_comment = {
        let comment = comment.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_dyn_into::<HtmlTextAreaElement>()
                .map(|input| input.value())
                .unwrap_or_default();
            comment.set(value);
        })
    };

    let update_rating = {
        let rating = rating.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_dyn_into::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();
            rating.set(value.parse().unwrap_or(0));
        })
    };

    let submit = {
        let comment = comment.clone();
        let rating = rating.clone();
        let error = error.clone();
        let rate = props.rate.clone();
        let user = props.user.clone();
        let post_id = props.post.id;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(user) = &user {
                error.set(false);
                rate.emit((post_id, user.username.clone(), *rating, (*comment).clone()));
            } else {
                error.set(true);
            }
        })
    };

    let delete = {
        let delete = props.delete.clone();
        let post_id = props.post.id;
        Callback::from(move |_| delete.emit(post_id))
    };

    let edit = {
        let edit = props.edit.clone();
        let post_id = props.post.id;
        Callback::from(move |_| edit.emit(post_id))
    };

    let owned = props
        .user
        .as_ref()
        .map(|user| user.username == props.post.author)
        .unwrap_or(false);

    html! {
        <article class="post">
            <div class="post-card">
                <img class="post-image" src={"data:image/*;base64, ".to_string() + &props.post.image} alt="A post shared by a CuteDogStash member"/>
                <div class="post-content">
                    <header class="post-header">
                        <span class="post-eyebrow">{"Shared by"}</span>
                        <h2 class="post-author">{&props.post.author}</h2>
                    </header>
                    <p class="post-description">{&props.post.description}</p>
                    {if owned {
                        html! {
                            <div class="post-actions">
                                <button onclick={edit} class="btn btn-primary">{"Edit post"}</button>
                                <button onclick={delete} class="btn btn-outline-danger">{"Delete post"}</button>
                            </div>
                        }
                    } else { html! {} }}
                    <section class="ratings" aria-label="Post ratings">
                        <h3 class="ratings-title">{"Ratings"}</h3>
                        <div class="ratings-list">
                            {for props.post.ratings.iter().map(|rating| html! {
                                <div class="rating">
                                    <div class="rating-meta">
                                        <span class="rating-author">{&rating.author}</span>
                                        <span class="rating-score">{&rating.stars}{" / 5"}</span>
                                    </div>
                                    <p>{&rating.comment}</p>
                                </div>
                            })}
                        </div>
                    </section>
                </div>
            </div>

            <section class="rate-post" aria-label="Rate this post">
                <h3>{"Rate this post"}</h3>
                <div class="rate-fields">
                    <label>
                        <span>{"Comment"}</span>
                        <textarea rows="4" required=true value={(*comment).clone()} oninput={update_comment}/>
                    </label>
                    <label class="rating-field">
                        <span>{"Rating"}</span>
                        <input type="number" min="0" max="5" placeholder="0–5" oninput={update_rating}/>
                    </label>
                </div>
                <button type="submit" onclick={submit} class="btn btn-primary">{"Submit rating"}</button>
            </section>

            {if *error { html! { <p class="rate-error" role="alert">{"Please log in before leaving a rating."}</p> } } else { html! {} }}
        </article>
    }
}
