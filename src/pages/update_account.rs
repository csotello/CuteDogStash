use crate::Routes;
use db::*;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub update: Callback<(u64, String, String)>,
    pub user: Option<User>,
    pub db: Data,
}

#[function_component(UpdateAccount)]
pub fn update_account(props: &Props) -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let confirm_password = use_state(String::new);
    let props = props.clone();
    let error = use_state(|| false);

    let update_error = {
        let error = error.clone();
        Callback::from(move |_| error.set(true))
    };
    // let update_error_ref = &update_error;
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
        let navigator = use_navigator().unwrap();
        let id = props.user.as_ref().unwrap().id.clone();
        let confirm_password = confirm_password.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username.to_string();
            let password = password.to_string();

            let error = error.clone();
            if password.to_string() != confirm_password.to_string() {
                error.set(true);
            } else {
                props.update.emit((id, username, password));
                navigator.push(&Routes::Home);
            }
            // props.update.emit((id, username, password));
            // let error = error.clone();
            // error.set(false);
            // navigator.push(&Routes::Home);
        })
    };
    let update_error = update_error.clone();
    let error = error.clone();
    html! {
        <main class="auth-page">
            <section class="post auth-card">
                <div class="post-content">
                    <header class="post-header">
                        <span class="post-eyebrow">{"Account settings"}</span>
                        <h1 class="post-author">{"Update your account"}</h1>
                    </header>
                    <p class="post-description">{"Choose a new username or password to keep your profile current."}</p>
                </div>
                {if *error { html! { <p class="rate-error" role="alert">{"Use a valid username and matching passwords."}</p> } } else { html! {} }}
                <form class="rate-post" onsubmit={on_submit}>
                    <label>
                        <span>{"Username"}</span>
                        <input type="text" pattern="[A-Za-z0-9]{1,20}" value={username.to_string()} required=true oninput={update_username} oninvalid={Callback::from(move |_e: Event| update_error.emit(()))}/>
                    </label>
                    <label>
                        <span>{"Password"}</span>
                        <input type="password" value={password.to_string()} required=true oninput={update_password}/>
                    </label>
                    <label>
                        <span>{"Confirm password"}</span>
                        <input type="password" value={confirm_password.to_string()} required=true oninput={update_confirm_password}/>
                    </label>
                    <button type="submit" class="btn btn-primary">{"Save changes"}</button>
                </form>
            </section>
        </main>
    }
}
