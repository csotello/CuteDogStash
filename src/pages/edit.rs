use crate::Routes;
use db::*;
use gloo_console::log;
use gloo_file::callbacks::{read_as_bytes, FileReader};
use gloo_file::File;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: u64,
    pub db: Data,
    pub callback: Callback<(u64, String, String)>,
}

#[function_component(Edit)]
pub fn edit(props: &Props) -> Html {
    let post = props.db.get_post(props.id);
    let initial_description = post
        .as_ref()
        .map(|post| post.description.clone())
        .unwrap_or_default();
    let initial_image = post
        .as_ref()
        .map(|post| post.image.clone())
        .unwrap_or_default();

    let description = use_state(|| initial_description);
    let replacement_image = use_state(|| None::<String>);
    let reader = use_mut_ref(|| None::<FileReader>);
    let navigator = use_navigator();

    let update_description = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    let handle_file = {
        let reader = reader.clone();
        let replacement_image = replacement_image.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let Some(web_file) = input.files().and_then(|files| files.get(0)) else {
                return;
            };

            let replacement_image = replacement_image.clone();
            *reader.borrow_mut() =
                Some(read_as_bytes(
                    &File::from(web_file),
                    move |result| match result {
                        Ok(bytes) => replacement_image.set(Some(base64::encode(bytes))),
                        Err(err) => log!(&err.to_string()),
                    },
                ));
        })
    };

    let submit = {
        let callback = props.callback.clone();
        let description = description.clone();
        let replacement_image = replacement_image.clone();
        let navigator = navigator.clone();
        let id = props.id;
        let initial_image = initial_image.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let image = (*replacement_image)
                .clone()
                .unwrap_or_else(|| initial_image.clone());
            callback.emit((id, (*description).clone(), image));
            if let Some(navigator) = navigator.clone() {
                navigator.push(&Routes::Home);
            }
        })
    };

    let replacement_src = replacement_image
        .as_ref()
        .map(|image| format!("data:image/*;base64, {image}"))
        .unwrap_or_default();

    html! {
        <div class="border border-dark login">
            <br/>
            <p>{"Edit"}</p>
            <form onsubmit={submit}>
                <fieldset>
                    <label>{"Current Picture:"}</label>
                    <img src={format!("data:image/*;base64, {initial_image}")} alt=""/><br/>
                    <label>{"New Picture:"}</label>
                    <img src={replacement_src} alt=""/><br/>
                    <input type="file" accept="image/*" onchange={handle_file}/><br/>
                    <label>{"Description:"}</label>
                    <input type="textarea"
                        rows=4
                        cols=4
                        pattern="[A-Za-z0-9!@#$%^&*(){}/|:;-_<>.,=+]*"
                        value={(*description).clone()}
                        required=true
                        oninput={update_description}/><br/>
                    <button type="submit" class="btn btn-primary">{"Update"}</button>
                </fieldset>
            </form>
        </div>
    }
}
