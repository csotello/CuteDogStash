use db::*;
use yew::prelude::*;
pub enum Msg {
    SetComment(String),
    SetRating(String),
    Submit,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub post: db::Post,
    pub callback: Callback<(u64, String, u8, String)>,
    pub user: Option<User>,
}

pub struct Post {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: Props,
    comment: String,
    rating: u8,
    error: bool,
}

impl Component for Post {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            comment: String::new(),
            rating: 0,
            error: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetComment(comment) => {
                self.comment = comment;
            }
            Msg::SetRating(rating) => {
                let rating: u8 = rating.parse().unwrap();
                self.rating = rating;
            }
            Msg::Submit => {
                if let Some(user) = &self.props.user {
                    self.error = false;
                    self.props.callback.emit((
                        self.props.post.id,
                        user.username.clone(),
                        self.rating,
                        self.comment.clone(),
                    ))
                } else {
                    self.error = true;
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let update_comment = self.link.callback(|e: InputData| Msg::SetComment(e.value));
        let update_rating = self.link.callback(|e: InputData| Msg::SetRating(e.value));
        let submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });
        let map_rating = |rating: &Rating| {
            html! {
                <div class="rating">
                    <span>{"Author:"}{&rating.author}</span><br/>
                    <span>{"Rating:"}{&rating.stars}{"/5"}</span>
                    <p>{&rating.comment}</p>
                </div>
            }
        };
        html! {
            <>
            <div class="post">
                <span>{"Author:"}{&self.props.post.author}</span><br/>
                <img src="data:image/*;base64, ".to_string() + &self.props.post.image alt=""/><br/>
                <p>{"Description:"}{&self.props.post.description}</p>
                {for self.props.post.ratings.iter().map(map_rating)}
            </div>
            <form onsubmit=submit>
                <fieldset>
                    <label>{"Comment"}</label>
                    <input type="textarea"
                        rows=4
                        cols=4
                        required=true
                        value=&self.comment
                        oninput=update_comment/>
                    <label>{"Rating"}</label>
                    <input type="number"
                        min=0
                        max=5
                        oninput=update_rating/>
                    <button type="submit">{"Rate"}</button>
                </fieldset>
                {if self.error {html!{<span>{"Must login to rate"}</span>}} else{ html!{}}}
            </form>
            </>
        }
    }
}
