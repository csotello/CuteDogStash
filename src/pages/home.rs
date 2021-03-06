use crate::components::Post;
use crate::utils::*;
use db::*;
use yew::prelude::*;
pub enum Msg {
    Rate(u64, String, u8, String),
    Edit(u64),
    Delete(u64),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub error: bool,
    pub db: Data,
    pub user: Option<User>,
    pub rate: Callback<(u64, String, u8, String)>,
    pub edit: Callback<u64>,
    pub delete: Callback<u64>,
}

pub struct Home {
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
            Msg::Delete(id) => {
                self.props.delete.emit(id);
            }
            Msg::Edit(id) => {
                self.props.edit.emit(id);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        // Create each post tag
        let map_post = |post: &db::Post| {
            let rate = self.link.callback(|(post_id, author, stars, comment)| {
                Msg::Rate(post_id, author, stars, comment)
            });
            let delete = self.link.callback(|id| {
                log("Deleting Post".to_string());
                Msg::Delete(id)
            });
            let edit = self.link.callback(|id| {
                log("Editing post".to_string());
                Msg::Edit(id)
            });
            html! {
                <Post post=post rate=rate delete=delete user=&self.props.user edit=edit/>
            }
        };
        html! {
            <div>
            <br/>
                {if self.props.error{html! {<p>{"Error"}</p>}} else {html!{}}}
                {for self.props.db.posts.iter().map(map_post)}
            </div>
        }
    }
}
