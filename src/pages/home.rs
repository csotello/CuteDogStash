use crate::components::Post;
use db::*;
use yew::prelude::*;
pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub error: bool,
    pub db: Data,
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
            _ => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let map_post = |post: &db::Post| {
            html! {
                <Post post=post/>
            }
        };
        if self.props.error {
            html! {<p>{"Error"}</p>}
        } else {
            html! {
                {for self.props.db.posts.iter().map(map_post)}
            }
        }
    }
}
