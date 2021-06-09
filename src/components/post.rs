use db::*;
use yew::prelude::*;
pub enum Msg {
    SetComment(String),
    SetRating(String),
    Edit,
    Rate,
    DeletePost,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub post: db::Post,
    pub rate: Callback<(u64, String, u8, String)>,
    pub edit: Callback<u64>,
    pub delete: Callback<u64>,
    pub user: Option<User>,
}

pub struct Post {
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
            Msg::Rate => {
                if let Some(user) = &self.props.user {
                    self.error = false;
                    self.props.rate.emit((
                        self.props.post.id,
                        user.username.clone(),
                        self.rating,
                        self.comment.clone(),
                    ))
                } else {
                    self.error = true;
                }
            }
            Msg::DeletePost => {
                let id = self.props.post.id;
                self.props.delete.emit(id);
            }
            Msg::Edit => {
                self.props.edit.emit(self.props.post.id);
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
            Msg::Rate
        });
        let map_rating = |rating: &Rating| {
            html! {
                <div class="rating border border-dark">
                    <span>{"Author:"}{&rating.author}</span><br/>
                    <span>{"Rating:"}{&rating.stars}{"/5"}</span>
                    <p>{&rating.comment}</p>
                </div>
            }
        };
        let delete = self.link.callback(|_| Msg::DeletePost);
        let owned = { // If post is owned by current user
            if let Some(user) = &self.props.user {
                user.username == self.props.post.author
            } else {
                false
            }
        };
        let edit = self.link.callback(|_| Msg::Edit);
        html! {
            <div class="post">
            <div class="card border-dark">
            <img class="card-img-top" src="data:image/*;base64, ".to_string() + &self.props.post.image alt=""/><br/>
            <div class="card-body">
                <span>{"Author:"}{&self.props.post.author}</span><br/>
                <p>{"Description:"}{&self.props.post.description}</p>
                {if owned{
                    html!{
                        <>
                            <button onclick=edit class="btn btn-primary">{"Edit Post"}</button>
                            <button onclick=delete class="btn btn-secondary">{"Delete Post"}</button>
                        </>
                    }
                } else{html!{}}}
                <p>{"Ratings:"}</p>
                {for self.props.post.ratings.iter().map(map_rating)}
            </div>
            </div>
            <form onsubmit=submit>
                <div class="mb-3">
                    <p>{"Rate Post"}</p>
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
                    <button type="submit" class="btn btn-outline-primary">{"Rate"}</button>
                </div>
            </form>
            {if self.error {html!{<span>{"Must login to rate"}</span>}} else{ html!{}}}
            </div>
        }
    }
}
