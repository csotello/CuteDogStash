use db::*;
use web_sys::HtmlInputElement;
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
                .target_dyn_into::<HtmlInputElement>()
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
            rating.set(value.parse().unwrap());
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
        <div class="post">
            <div class="card border-dark">
                <img class="card-img-top" src={"data:image/*;base64, ".to_string() + &props.post.image} alt=""/><br/>
                <div class="card-body">
                    <span>{"Author:"}{&props.post.author}</span><br/>
                    <p>{"Description:"}{&props.post.description}</p>
                    {if owned {
                        html! {
                            <>
                                <button onclick={edit} class="btn btn-primary">{"Edit Post"}</button>
                                <button onclick={delete} class="btn btn-secondary">{"Delete Post"}</button>
                            </>
                        }
                    } else {
                        html! {}
                    }}
                    <p>{"Ratings:"}</p>
                    {for props.post.ratings.iter().map(|rating| html! {
                        <div class="rating border border-dark">
                            <span>{"Author:"}{&rating.author}</span><br/>
                            <span>{"Rating:"}{&rating.stars}{"/5"}</span>
                            <p>{&rating.comment}</p>
                        </div>
                    })}
                </div>
            </div>

            <div class="mb-3">
                <p>{"Rate Post"}</p>
                <label>{"Comment"}</label>
                <input type="textarea" rows=4 cols=4 required=true value={(*comment).clone()} oninput={update_comment}/>
                <label>{"Rating"}</label>
                <input type="number" min=0 max=5 oninput={update_rating}/>
                <button type="submit" onclick={submit} class="btn btn-outline-primary">{"Rate"}</button>
            </div>

            {if *error { html! { <span>{"Must login to rate"}</span> } } else { html! {} }}
        </div>
    }
}
