use db::*;
use yew::prelude::*;
pub enum Msg {
    SetDescription(String),
    Submit,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub error: bool,
    pub db: Data,
}

pub struct Post {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    description: String,
    props: Props,
}

impl Component for Post {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            description: String::new(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetDescription(description) => {
                self.description = description;
            }
            Msg::Submit => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let update_description = self
            .link
            .callback(|e: InputData| Msg::SetDescription(e.value));
        let onsubmit = self.link.callback(|_| Msg::Submit);
        if self.props.error {
            html! {<p>{"Error"}</p>}
        } else {
            html! {
                <>
                    <p>{"Create Post"}</p>
                    <form onsubmit=onsubmit>
                        <fieldset>
                            <label>{"Description:"}</label>
                            <input type="text" pattern="[A-Za-z0-9]*"
                                value=&self.description
                                required=true
                                oninput=update_description/>
                            <button type="submit">{"Post"}</button>
                    </fieldset>
                </form>
                </>
            }
        }
    }
}
