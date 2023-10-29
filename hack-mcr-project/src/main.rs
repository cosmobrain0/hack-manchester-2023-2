#![feature(iter_intersperse)]
use leptos::{
    html::{param, Input},
    *,
};
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Definition {
    word: String,
    definition: String,
    phonetics: String,
}
impl Definition {
    pub fn new(word: String, definition: String, phonetics: String) -> Self {
        Self {
            word,
            definition,
            phonetics,
        }
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/watch/:id" view=ViewPage></Route>
                <Route path="/" view=MainPage></Route>
            </Routes>
        </Router>
    }
}

#[component]
fn MainPage() -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    let submit = move |_| {
        let node = input_ref.get().unwrap();
        let path = node.value();
        (use_navigate())(&format!("/watch/{path}"), Default::default());
    };
    view! {
        <input _ref=input_ref type="text" pattern=r#"[\w\d_]+"# />
        <button on:click=submit> "Watch Video!" </button>
    }
}

#[component]
fn ViewPage() -> impl IntoView {
    let data_element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("dictionary-data")
        .unwrap();
    let params_map = use_params_map();
    let id = move || params_map.with(|params| params.get("id").cloned());
    if id().is_none() {
        (use_navigate())("/", Default::default());
    }

    let data = Signal::derive(move || data_element.inner_html());
    let definitions: Signal<Vec<Definition>> =
        Signal::derive(move || data.with(|x| serde_json::from_str(x).unwrap()));

    let (current_definition, set_current_definition): (
        ReadSignal<Option<Definition>>,
        WriteSignal<Option<Definition>>,
    ) = create_signal(None);
    provide_context(set_current_definition);

    let (caption_data, set_caption_data) = create_signal(None);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        request_animation_frame(f.borrow().as_ref().unwrap());
        let Some(data) = get_caption_data() else {
            return;
        };
        if caption_data.with_untracked(|x| x.is_some()) {
            return;
        }
        logging::log!("Hi there! {}", &data);
        let caption_data = Caption::parse_from_data(data);
        logging::log!("Data: {:#?}", &caption_data);
        set_caption_data.set(Some(caption_data));
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    view! {
        <h1 class="title">Ducktitles</h1>
        <div id="video-and-transcript-flex-wrapper">
            <div id="video-wrapper">
                <Video id=id().unwrap() />
            </div>


            // <Definitions definitions set_definition=set_current_definition />
            <div id = "transcript-flex-wrapper">
                <div id="full-definition-view-wrapper">
                    <FullDefinitionView definition={current_definition} />
                </div>
                <Transcript text=caption_data />

            </div>
        </div>
        <footer>

            <p>Created for <a href = "https://hackmanchester.co.uk" target = "_blank">Hac JR Manchester 2023</a></p>
            <p>By Affan Siddiqui and Felix Geupel</p>
        </footer>
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn get_caption_data() -> Option<String> {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("captions")
        .map(|x| x.inner_html())
        .filter(|x| !x.is_empty() && !x.chars().all(|x| x.is_whitespace()))
}

#[component]
fn Video(#[prop(into)] id: String) -> impl IntoView {
    view! {
        <iframe id="video-player" src="https://player.vimeo.com/video/877547033?title=0&amp;byline=0&amp;portrait=0&amp;speed=0&amp;badge=0&amp;autopause=0&amp;player_id=0&amp;app_id=58479&amp;transcript=false" width={(1920/2).to_string()} height={(1080/2).to_string()} frameborder="0" allow="fullscreen; picture-in-picture" allowfullscreen title="Test Video Title" />
    }
}

#[component]
fn DefinitionView(
    definition: Definition,
    set_definition: WriteSignal<Option<Definition>>,
) -> impl IntoView {
    let word = definition.word.clone();
    view! {
        <div class="definition-wrapper" on:click = move |_| set_definition.set(Some(definition.clone()))>
            <h2 class="definition-word"> {word} </h2>
            // <p class="definition-info"> {definition.definition} </p>
        </div>
    }
}

#[component]
fn Transcript(text: ReadSignal<Option<Vec<Caption>>>) -> impl IntoView {
    let text_output = move || {
        let thing = text.with(|x| {
            x.as_ref().map(|x| {
                x.iter()
                    .map(|x| format!("{data}\n", data = x.data))
                    .collect::<String>()
                    .split_whitespace()
                    .map(|word| view! { <TranscriptWord word=word.to_owned() /> " " }.into_view())
                    .collect_view()
            })
        });
        if let Some(result) = thing {
            result
        } else {
            "Loading...".into_view()
        }
    };
    view! {
        <div id="transcript">
            // {text.split_whitespace().inspect(|x| logging::log!("{}", x)).map(|word| view!{ <TranscriptWord word=word.to_owned() /> " " }.into_view()).collect_view()}
            {text_output}
        </div>
    }
}

#[component]
fn TranscriptWord(word: String) -> impl IntoView {
    let set_definition = expect_context::<WriteSignal<Option<Definition>>>();
    let word_to_show = word.clone();
    view! {
        <span on:click=move |_| set_definition_from_word(set_definition, &word) class="transcript-word">{word_to_show}</span>
    }
}

fn get_definitions() -> Vec<Definition> {
    let data_element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("dictionary-data")
        .unwrap();
    let values: Vec<Definition> = serde_json::from_str(&data_element.inner_html()).unwrap();
    values
}

fn set_definition_from_word(set_definition: WriteSignal<Option<Definition>>, word: &str) {
    let word_pattern = regex::Regex::new(r#"\w+"#);
    let thing = word_pattern
        .unwrap()
        .captures_iter(word)
        .map(|x| x.extract::<0>().0)
        .collect::<String>()
        .to_lowercase();
    logging::log!("Regex match: {thing}");

    let Some(definition) = get_definitions()
        .into_iter()
        .find(|x| x.word.as_str() == thing)
    else {
        return;
    };
    set_definition.set(Some(definition));
}

#[component]
fn Definitions(
    definitions: Signal<Vec<Definition>>,
    set_definition: WriteSignal<Option<Definition>>,
) -> impl IntoView {
    view! {
        <div id="definition-list-wrapper">
            <For each={move || definitions.get().into_iter()} key={move |definition| definition.word.to_owned()} children={move |item| view!{ <DefinitionView definition=item.clone() set_definition /> }} />
        </div>
    }
}

#[component]
fn FullDefinitionView(definition: ReadSignal<Option<Definition>>) -> impl IntoView {
    move || {
        definition.with(|d| {
            if let Some(d) = d {
                view! {
                    <div id="word-phonetics-wrapper">
                        <h2 id="full-definition-word"> {&d.word} </h2>
                        <p>{&d.phonetics}</p>
                    </div>
                    <p id="full-definition-info"> {&d.definition} </p>
                }
                .into_view()
            } else {
                ().into_view()
            }
        })
    }
}

#[derive(Debug, Clone)]
struct Caption {
    start_time: (usize, f32),
    end_time: (usize, f32),
    data: String,
}

impl Caption {
    fn new(start_time: (usize, f32), end_time: (usize, f32), data: String) -> Self {
        Self {
            start_time,
            end_time,
            data,
        }
    }

    fn parse_from_data(data: String) -> Vec<Caption> {
        let blank = regex::Regex::new(r#"^[ \t]?+$"#).unwrap();
        let timestamp = regex::Regex::new(r#"(\d{2}):([\d.]+) --&gt; (\d{2}):([\d.]+)"#).unwrap();
        // let caption = regex::Regex::new(r#"( +- +)?(.*)"#).unwrap();
        let caption = regex::Regex::new(r#"(.*)"#).unwrap();

        let mut current_timestamp = None;
        let mut captions = Vec::new();
        let mut current_caption = String::new();

        for line in data.lines() {
            if blank.is_match(line) {
                if !current_caption.is_empty() && current_timestamp.is_some() {
                    // logging::log!("Inserting caption!");
                    captions.push(Caption::with(current_timestamp.unwrap(), &current_caption));
                } else {
                    // logging::log!(
                    //     "Found blank line, but (timestamp is some: {}) and (caption is: {})",
                    //     current_timestamp.is_some(),
                    //     &current_caption
                    // );
                }
                current_timestamp = None;
                current_caption = String::new();
                continue;
            }
            if timestamp.is_match(line) {
                current_timestamp = Some(Timestamp::try_from(line).unwrap());
            } else if current_timestamp.is_some() && caption.is_match(line) {
                // logging::log!("With line... {line}");
                let captures: Vec<_> = caption
                    .captures(line)
                    .into_iter()
                    .next()
                    .unwrap()
                    .iter()
                    .filter_map(|x| x)
                    .map(|x| x.as_str().to_owned())
                    .collect();
                // logging::log!("Line Captures: {:#?}", &captures);
                let line = &captures[captures.len() - 1];
                current_caption.push_str("\n");
                current_caption.push_str(line);
            }
        }
        if !current_caption.is_empty() && current_timestamp.is_some() {
            // logging::log!("Inserting caption!");
            captions.push(Caption::with(current_timestamp.unwrap(), &current_caption));
        }

        captions
    }

    fn with(timestamp: Timestamp, line: &str) -> Self {
        Self {
            start_time: (timestamp.start_minute, timestamp.start_second),
            end_time: (timestamp.end_minute, timestamp.end_second),
            data: line.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Timestamp {
    start_minute: usize,
    start_second: f32,
    end_minute: usize,
    end_second: f32,
}
impl TryFrom<&str> for Timestamp {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let timestamp = regex::Regex::new(r#"(\d{2}):([\d.]+) --&gt; (\d{2}):([\d.]+)"#).unwrap();
        let Some(capture) = timestamp.captures(value).into_iter().next() else {
            // logging::log!("Not a timestamp! {}", &value);
            return Err(());
        };
        let parts: [_; 4] = capture.extract().1;
        // logging::log!("Parts: {:#?}", &parts);
        Ok(Self {
            start_minute: parts[0].parse().unwrap(),
            start_second: parts[1].parse().unwrap(),
            end_minute: parts[2].parse().unwrap(),
            end_second: parts[3].parse().unwrap(),
        })
    }
}
