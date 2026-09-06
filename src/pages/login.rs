use crate::Routes;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub callback: Callback<(String, String)>,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let props = props.clone();

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
    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let navigator = use_navigator().unwrap();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username.to_string();
            let password = password.to_string();
            props.callback.emit((username, password));
            navigator.push(&Routes::Home);
        })
    };

    html! {
        <main class="auth-page">
            <section class="post auth-card">
                <div class="post-content">
                    <header class="post-header">
                        <span class="post-eyebrow">{"Welcome back"}</span>
                        <h1 class="post-author">{"Log in"}</h1>
                    </header>
                    <p class="post-description">{"Pick up where you left off with the CuteDogStash community."}</p>
                </div>
                <form class="rate-post" onsubmit={onsubmit}>
                    <label>
                        <span>{"Username"}</span>
                        <input type="text" pattern="[A-Za-z0-9]{1,10}"
                        value={username.to_string()}
                        required=true
                        oninput={update_username}/>
                    </label>
                    <label>
                        <span>{"Password"}</span>
                        <input type="password"
                        value={password.to_string()}
                        required=true
                        oninput={update_password}/>
                    </label>
                    <button type="submit" class="btn btn-primary">{"Log in"}</button>
                </form>
            </section>
        </main>
    }
}
