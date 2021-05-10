use yew::prelude::*;

pub enum Msg {
    AddOne,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub error: bool,
}

pub struct Home {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                {
                    if self.props.error {
                        html! {<p>{"Error"}</p>}
                    }
                    else {
                        html! {}
                    }
                }
                <p>{"Home"}</p>
            </>
        }
    }
}
