use leptos::*;

fn main() {
    mount_to_body(|| view! { <App /> });
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Definition {
    word: String,
    definition: String,
}
impl Definition {
    pub fn new(word: String, definition: String) -> Self {
        Self { word, definition }
    }
}

#[component]
fn App() -> impl IntoView {
    let (definitions, set_definitions) = create_signal(vec![
        Definition::new("Potato".into(), "A cool vegetable!".into()),
        Definition::new("Apple".into(), "A really cool fruit!\nSuper cool!".into()),
    ]);
    view! {
        <Video id="tI8OqpkOVzs" />
        <Definitions definitions />
    }
}

#[component]
fn Video(#[prop(into)] id: String) -> impl IntoView {
    view! {
        <iframe width="560" height="315" src="https://www.youtube.com/embed/tI8OqpkOVzs?si=dyXK0B1u1LzLR42f" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
    }
}

#[component]
fn DefinitionView(definition: Definition) -> impl IntoView {
    view! {
        <div class="definition-wrapper">
            <h2 class="definition-word"> {definition.word} </h2>
            <p class="definition-info"> {definition.definition} </p>
        </div>
    }
}

#[component]
fn Definitions(definitions: ReadSignal<Vec<Definition>>) -> impl IntoView {
    view! {
        <div id="definition-list-wrapper">
            <For each={move || definitions.get().into_iter()} key={move |definition| definition.word.to_owned()} children={move |item| view!{ <DefinitionView definition=item.clone() /> }} />
        </div>
    }
}
