#![feature(iter_intersperse)]
use leptos::{
    html::{param, Input},
    *,
};
use leptos_router::*;
use serde::{Deserialize, Serialize};

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

    view! {
        <Video id=id().unwrap() />
        <div id="full-definition-view-wrapper">
            <FullDefinitionView definition={current_definition} />
        </div>
        // <Definitions definitions set_definition=set_current_definition />
        <Transcript text="The quick brown fox jumps over the lazy dog.".into() />
    }
}

#[component]
fn Video(#[prop(into)] id: String) -> impl IntoView {
    view! {
        <iframe width="560" height="315" src=format!("https://www.youtube.com/embed/{id}?si=dyXK0B1u1LzLR42f") title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
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
fn Transcript(text: String) -> impl IntoView {
    view! {
        <div id="transcript">
            {text.split_whitespace().inspect(|x| logging::log!("{}", x)).map(|word| view!{ <TranscriptWord word=word.to_owned() /> " " }.into_view()).collect_view()}
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
                    <h2 id="full-definition-word"> {&d.word} </h2>
                    <p id="full-definition-info"> {&d.definition} </p>
                }
                .into_view()
            } else {
                ().into_view()
            }
        })
    }
}
