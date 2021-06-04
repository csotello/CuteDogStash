use crate::Routes;
use db::*;
use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_router::prelude::RouteAgent;
extern crate base64;
#[derive(Properties, Clone)]
pub struct Props {
    pub id: u64,
    pub db: Data,
    pub callback: Callback<(u64, String, String)>,
}

pub enum Msg {
    SetDescription(String),
    SetFile(FileData),
    LoadFile(File),
    ResetFile,
    Submit,
    None,
}

pub struct Edit {
    link: ComponentLink<Self>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    description: String,
    image: String,
    file: String,
    task: Vec<ReaderTask>,
    props: Props,
}

impl Component for Edit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = &props.id;
        let post = props.db.get_post(*id).unwrap();
        Self {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::None)),
            link,
            description: post.description,
            image: post.image,
            file: String::new(),
            task: Vec::new(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetDescription(desc) => {
                self.description = desc;
            }
            Msg::LoadFile(file) => {
                let callback = self.link.callback(Msg::SetFile);
                let mut reader = ReaderService::new();
                let task = reader.read_file(file, callback).unwrap();
                self.task.push(task);
            }
            Msg::SetFile(file) => {
                self.file = base64::encode(file.content);
            }
            Msg::ResetFile => {
                self.file = "".to_string();
            }
            Msg::Submit => {
                self.props.callback.emit((
                    self.props.id,
                    self.description.clone(),
                    self.file.clone(),
                ));
                self.router_agent.send(ChangeRoute(Routes::Home.into()));
            }
            Msg::None => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let set_desc = self
            .link
            .callback(|e: InputData| Msg::SetDescription(e.value));
        let submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });
        html! {
            <>
                <p>{"Edit"}</p>
                <form onsubmit=submit>
                    <fieldset>
                        <label>{"Current Picture:"}</label>
                        <img src="data:image/*;base64, ".to_string() + &self.image alt=""/><br/>
                        <label>{"New Picture:"}</label>
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
                            oninput=set_desc/><br/>
                       <button type="submit">{"Update"}</button>
                    </fieldset>
                </form>
            </>
        }
    }
}
