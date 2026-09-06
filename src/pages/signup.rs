use crate::Routes;
use db::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub callback: Callback<(String, String)>,
    pub db: Data,
}

#[function_component(SignUp)]
pub fn signup(props: &Props) -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let confirm_password = use_state(String::new);
    let props = props.clone();
    let navigator = use_navigator().unwrap();
    let error = use_state(|| false);

    let update_error = {
        let error = error.clone();
        Callback::from(move |_| error.set(true))
    };
    let update_username = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            username.set(value);
        })
    };
    let update_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            password.set(value);
        })
    };
    let update_confirm_password = {
        let confirm_password = confirm_password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            confirm_password.set(value);
        })
    };
    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username.to_string();
            let password = password.to_string();
            if password.to_string() != confirm_password.to_string() {
                error.set(true);
                navigator.push(&Routes::SignUp);
            } else {
                props.callback.emit((username, password));
                navigator.push(&Routes::Home);
            }
        })
    };
    html! {
        <main class="auth-page">
            <section class="post auth-card">
                <div class="post-content">
                    <header class="post-header">
                        <span class="post-eyebrow">{"Join the pack"}</span>
                        <h1 class="post-author">{"Create an account"}</h1>
                    </header>
                    <p class="post-description">{"Save posts, leave ratings, and share dog moments with the community."}</p>
                </div>
                {if *error { html! { <p class="rate-error" role="alert">{"Username or password is unacceptable"}</p> } } else { html! {} }}
                <form class="rate-post" onsubmit={on_submit}>
                    <label>
                        <span>{"Username"}</span>
                        <input type="text" pattern="[A-Za-z0-9]{1,20}" value={username.to_string()} required=true oninput={update_username} oninvalid={update_error}/>
                    </label>
                    <label>
                        <span>{"Password"}</span>
                        <input type="password" value={password.to_string()} required=true oninput={update_password}/>
                    </label>
                    <label>
                        <span>{"Confirm Password"}</span>
                        <input type="password" value={confirm_password.to_string()} required=true oninput={update_confirm_password}/>
                    </label>
                    <button type="submit" class="btn btn-primary">{"Create account"}</button>
                </form>
            </section>
        </main>
    }
}
