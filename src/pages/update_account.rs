use crate::Routes;
use db::*;
use web_sys::HtmlInputElement;
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
    let props = props.clone();
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
    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let navigator = use_navigator().unwrap();
        let id = props.user.as_ref().unwrap().id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let username = username.to_string();
            let password = password.to_string();
            props.update.emit((id, username, password));
            navigator.push(&Routes::Home);
        })
    };
    html! {
        <div class="border border-dark update">
            <br/>
            <p>{"Update Account Info"}</p>
            {if *(error) {html!{<p>{"Invalid username or password\nUsername cannot contain special characters"}</p>}} else {html!{}}}
                <fieldset>
                <label>{"Username:"}</label>
                <input type="text" pattern="[A-Za-z0-9]{1,20}"
                    value={username.to_string()} required=true
                    oninput={update_username}
                    oninvalid={update_error}/>
                <br/>
                <label>{"Password:"}</label>
                <input type="password"
                    value={password.to_string()} required=true
                    oninput={update_password}/>
                <br/>
                <button onclick={on_submit} class="btn btn-primary">{"Update"}</button>
                </fieldset>
        </div>
    }
}
