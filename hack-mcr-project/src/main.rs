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
    let (current_definition, set_current_definition) = create_signal(None);
    view! {
        <Video id="tI8OqpkOVzs" />
        <div id="full-definition-view-wrapper">
            <FullDefinitionView definition={current_definition} />
        </div>
        <Definitions definitions set_definition=set_current_definition />
    }
}

#[component]
fn Video(#[prop(into)] id: String) -> impl IntoView {
    view! {
        <iframe width="560" height="315" src="https://www.youtube.com/embed/tI8OqpkOVzs?si=dyXK0B1u1LzLR42f" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
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
fn Definitions(
    definitions: ReadSignal<Vec<Definition>>,
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
