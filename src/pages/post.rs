use crate::routes::Routes;
use crate::utils::*;
use db::*;
use yew::events::ChangeData;
use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_router::prelude::RouteAgent;
extern crate base64;
pub enum Msg {
    SetDescription(String),
    SetFile(FileData),
    LoadFile(File),
    ResetFile,
    Submit,
    None,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub db: Data,
    pub callback: Callback<(String, String, String)>,
    pub user: Option<User>,
}

pub struct Post {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    description: String,
    file: String,
    task: Vec<ReaderTask>,
    error: bool,
    props: Props,
}

impl Component for Post {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::None)),
            link,
            description: String::new(),
            file: String::new(),
            task: Vec::new(),
            error: false,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetDescription(description) => {
                self.description = description;
            }
            Msg::SetFile(file) => {
                self.file = base64::encode(file.content);
            }
            Msg::LoadFile(file) => {
                let callback = self.link.callback(Msg::SetFile);
                let mut reader = ReaderService::new();
                let task = reader.read_file(file, callback).unwrap();
                self.task.push(task);
            }
            Msg::ResetFile => {
                self.file = "".to_string();
            }
            Msg::Submit => match &self.props.user {
                Some(user) => {
                    self.props.callback.emit((
                        user.username.clone(),
                        self.description.clone(),
                        self.file.clone(),
                    ));
                    self.router_agent.send(ChangeRoute(Routes::Home.into()));
                }
                None => {
                    log("User not logged in".to_string());
                }
            },
            Msg::None => {}
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
        let onsubmit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });
        if self.error {
            html! {<p>{"Error"}</p>}
        } else {
            html! {
                <div class="border border-dark create">
                    <br/>
                    <p>{"Create Post"}</p>
                    <form onsubmit=onsubmit>
                        <fieldset>
                            <label>{"Picture:"}</label>
                            <img src="data:image/*;base64, ".to_string() + &self.file alt=""/><br/>
                            <input type="file" accept="image/*" onchange=self.link.callback(move |data: ChangeData| {
                                match data {
                                    ChangeData::Files(files) => {
                                        Msg::LoadFile(files.get(0).unwrap())
                                    }
                                    _ => Msg::ResetFile
                                }
                            }) /><br/>
                            <label>{"Description:"}</label>
                            <input type="textarea"
                                rows=4
                                cols=4
                                pattern="[A-Za-z0-9!@#$%^&*(){}/|:;-_<>.,=+]*"
                                value=&self.description
                                required=true
                                oninput=update_description/><br/>
                            <button type="submit" class="btn btn-primary">{"Post"}</button>
                        </fieldset>
                    </form>
                </div>
            }
        }
    }
}
