use crate::components::Post;
use db::*;
use yew::prelude::*;
pub enum Msg {
    Rate(u64, String, u8, String),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub error: bool,
    pub db: Data,
    pub user: Option<User>,
    pub rate: Callback<(u64, String, u8, String)>,
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
            Msg::Rate(id, author, stars, comment) => {
                self.props.rate.emit((id, author, stars, comment));
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let map_post = |post: &db::Post| {
            let callback = self.link.callback(|(post_id, author, stars, comment)| {
                Msg::Rate(post_id, author, stars, comment)
            });
            html! {
                <Post post=post callback=callback user=&self.props.user/>
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
