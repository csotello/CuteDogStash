use db::*;
use yew::prelude::*;
pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub post: db::Post,
}

pub struct Post {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Post {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <span>{"Author:"}{&self.props.post.author}</span><br/>
                <img src="data:image/*;base64, ".to_string() + &self.props.post.image alt=""/><br/>
                <p>{"Description:"}{&self.props.post.description}</p>
            </>
        }
    }
}
