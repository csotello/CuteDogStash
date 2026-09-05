use crate::routes::Routes;
use db::*;
use gloo_console::log;
use gloo_file::callbacks::{read_as_bytes, FileReader};
use gloo_file::File;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
extern crate base64;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub db: Data,
    pub callback: Callback<(String, String, String)>,
    pub user: Option<User>,
}
#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    let description = use_state(String::new);
    let file = use_state(|| None::<String>);
    let error = use_state(|| false);
    let reader = use_mut_ref(|| None::<FileReader>);
    let navigator = use_navigator();

    let update_description = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            description.set(value);
        })
    };

    let handle_file = {
        let reader = reader.clone();
        let file = file.clone();
        let error = error.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let current_file = input.files().and_then(|files| files.get(0)).unwrap();
            let file = file.clone();
            let error = error.clone();
            let set_file = {
                Callback::from(
                    move |result: Result<Vec<u8>, gloo_file::FileReadError>| match result {
                        Ok(bytes) => {
                            file.set(Some(base64::encode(bytes)));
                        }
                        Err(err) => {
                            log!(&err.to_string());
                            error.set(true);
                            file.set(None);
                        }
                    },
                )
            };
            *reader.borrow_mut() = Some(read_as_bytes(&File::from(current_file), move |result| {
                set_file.emit(result)
            }));
        })
    };

    let submit = {
        let description = description.clone();
        let file = file.clone();
        let navigator = navigator.clone();
        let callback_clone = props.callback.clone();
        let user = props.user.clone();

        Callback::from(move |_e: SubmitEvent| match &user {
            Some(user) => {
                let description = description.clone();
                let Some(file) = (*file).clone() else {
                    log!("No file");
                    return;
                };
                callback_clone.emit((user.username.clone(), (*description).clone(), file.clone()));
                if let Some(navigator) = navigator.clone() {
                    navigator.push(&Routes::Home);
                }
            }
            None => {
                log!("User not logged in");
            }
        })
    };

    let image = format!(
        "data:image/*;base64, {}",
        file.as_deref().unwrap_or(&String::new())
    );

    html! {
        <div class="border border-dark create">
            <br/>
            <p>{"Create Post"}</p>
            <form onsubmit={submit}>
                <fieldset>
                    <label>{"Picture:"}</label>
                    <img src={image} alt=""/><br/>
                    <input type="file" accept="image/*" onchange={handle_file}/><br/>
                    <label>{"Description:"}</label>
                    <input type="textarea"
                        rows=4
                        cols=4
                        pattern="[A-Za-z0-9]@#$%^&*(){}/|:;-_<>.,=+!*"
                        value={(*description).clone()}
                        required=true
                        oninput={update_description}/><br/>
                    <button type="submit" class="btn btn-primary">{"Post"}</button>
                </fieldset>
            </form>
        </div>
    }
}
