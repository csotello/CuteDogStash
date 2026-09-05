use crate::components::*;
use crate::pages::{Account, Edit, Home, Login, Post, SignUp, UpdateAccount};
use crate::routes::Routes;
use crate::utils::*;
use db::*;
use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;
const KEY: &str = "CuteDogStash_KEY";
pub enum Msg {
    SignUp(String, String),
    Login(String, String),
    CreatePost(String, String, String),
    Rate(u64, String, u8, String),
    DeleteAccount(String),
    DeletePost(u64),
    UpdateAccount(u64, String, String),
    UpdatePost(u64, String, String),
    EditPost(u64),
    Logout,
}
//Base App which controls routing

pub struct App {
    db: Data,           //Database
    user: Option<User>, //Current user
    error: bool,
    post_id: u64, // Id of post to edit
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let data = LocalStorage::get(KEY);
        // log!(data.unwrap_or_else(|_| "".to_string()));
        let db = Data::new(data.unwrap_or_else(|_| "".to_string())); // Load inital data
        Self {
            db,
            user: None,
            error: false,
            post_id: 0,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Create a new user in the database
            Msg::SignUp(username, password) => {
                self.db.create_user(username, password);
                self.db.store(KEY);
            }
            // Retrieve user information from database
            Msg::Login(username, password) => match self.db.login(username, password) {
                Some(user) => {
                    self.user = Some(User {
                        id: user.id,
                        username: user.username,
                        password: "".to_string(),
                    });
                    self.error = false;
                }
                None => {
                    self.error = true;
                }
            },
            Msg::Logout => {
                self.user = None;
            }
            // Create a new post
            Msg::CreatePost(author, description, image) => {
                self.db.create_post(author, description, image);
                self.db.store(KEY);
            }
            // Create a new rating
            Msg::Rate(id, author, stars, comment) => {
                self.db.create_rating(id, author, stars, comment);
                // LocalStorage::set(KEY, &self.db);
                self.db.store(KEY);
            }
            // Delete an account
            Msg::DeleteAccount(username) => {
                self.db.delete_account(username);
                self.user = None;
                self.db.store(KEY);
            }
            // Delete a post
            Msg::DeletePost(id) => {
                self.db.delete_post(id);
                self.db.store(KEY);
            }
            // Update account information
            Msg::UpdateAccount(id, username, password) => {
                self.db.update_account(id, username, password);
                self.db.store(KEY);
            }
            // Switch to edit page
            Msg::EditPost(id) => {
                self.post_id = id;
                let navigator = ctx.link().navigator();
                if let Some(navigator) = navigator {
                    navigator.push(&Routes::Edit);
                } else {
                    log("Navigator not found".to_string());
                }
                // ctx.link().navigator().unwrap().replace(&Routes::Edit);
                // self.route = Some(Routes::Edit);
            }
            // Update post information
            Msg::UpdatePost(id, desc, img) => {
                self.db.update_post(id, desc, img);
                self.db.store(KEY);
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let logout = ctx.link().callback(|_| Msg::Logout);
        let link = ctx.link().clone();
        // let user = self.user.clone();
        let switch = {
            let link = link.clone();
            let db = self.db.clone();
            let user = self.user.clone();
            let error = self.error.clone();
            let post_id = self.post_id.clone();

            Callback::from(move |route: Routes| {
                let create_post = link.callback(|(author, description, image)| {
                    Msg::CreatePost(author, description, image)
                });
                let delete_account = link.callback(|username| {
                    log("Deleting account".to_string());
                    Msg::DeleteAccount(username)
                });
                let update_account = link.callback(|(id, username, password)| {
                    Msg::UpdateAccount(id, username, password)
                });
                let update_post = link.callback(|(id, desc, img)| Msg::UpdatePost(id, desc, img));
                let signup = link.callback(|(username, password)| Msg::SignUp(username, password));
                let login = link.callback(|(username, password)| Msg::Login(username, password));
                let delete_post = link.callback(|id| {
                    log("Deleting Post".to_string());
                    Msg::DeletePost(id)
                });
                let edit_post = link.callback(|id| {
                    log("Editing post".to_string());
                    Msg::EditPost(id)
                });
                let rate = link.callback(|(post_id, author, stars, comment)| {
                    Msg::Rate(post_id, author, stars, comment)
                });
                // let user = self.user.clone();
                match &user {
                    Some(_user) => {
                        // If user is logged i
                        match route {
                            Routes::Home => {
                                html! {<Home error={error} db={db.clone()} user={user.clone()} rate={rate} delete={delete_post} edit={edit_post}/>}
                            }
                            Routes::Account => {
                                html! {<Account db={db.clone()} user={user.clone()} rate={rate} delete_account={delete_account} delete_post={delete_post} edit={edit_post}/>}
                            }
                            Routes::UpdateAccount => {
                                html! {<UpdateAccount user={user.clone()} db={db.clone()} update={update_account}/>}
                            }
                            Routes::Edit => html! {
                                <Edit callback={update_post} db={db.clone()} id={post_id.clone()}/>
                            },
                            Routes::Post => {
                                html! {<Post db={db.clone()} callback={create_post} user={user.clone()}/>}
                            }
                            _ => html! {<p>{"Invalid route"}</p>},
                        }
                    }

                    None => {
                        // User is logged out
                        match route {
                            Routes::Home => {
                                html! {<Home error={error} db={db.clone()} rate={rate} delete={delete_post} user={None} edit={edit_post}/>}
                            }
                            Routes::Login => html! {<Login callback={login}/>},
                            Routes::SignUp => {
                                html! {<SignUp callback={signup} db={db.clone()}/>}
                            }
                            _ => html! {<p>{"Login to access this page"}</p>},
                        }
                    }
                    _ => html! {<p>{"Invalid route"}</p>},
                }
            })
        };
        html! {
            <>
                <Navbar user={self.user.clone()} logout={logout.clone()}/>
                <Switch<Routes> render={switch} />
            </>
        }
    }
}
