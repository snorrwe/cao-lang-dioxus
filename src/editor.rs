use cao_lang::prelude::{Card as CaoLangCard, CardIndex};
use dioxus::prelude::*;

const FIB_PROG: &[u8] = include_bytes!("../assets/fibonacci_program_recursive.yaml");

#[derive(Clone)]
pub struct CaoLangProgram(pub cao_lang::prelude::CaoProgram);

#[component]
pub fn Editor() -> Element {
    let program = use_signal(|| {
        let program: cao_lang::prelude::CaoProgram = serde_yaml::from_slice(&FIB_PROG).unwrap();
        CaoLangProgram(program)
    });

    rsx! {
        div { class: "mx-auto",

            "hello"
            Program { program }
            DebugProgram { program }
        }
    }
}

#[component]
fn Program(program: Signal<CaoLangProgram>) -> Element {
    use_context_provider(|| program);
    let functions_it = (0..program.read().0.functions.len()).map(|func_idx| {
        rsx! {
            li {
                Function { function_idx: func_idx }
            }
        }
    });
    rsx! {
        ul { {functions_it} }
    }
}

#[component]
fn Function(function_idx: usize) -> Element {
    let program_sig = use_context::<Signal<CaoLangProgram>>();
    let program = program_sig.read();
    let func = program.0.functions.get(function_idx);
    let Some((name, _func)) = func else {
        return rsx! {
            div { class: "text-red text-4xl", "Function not found" }
        };
    };
    rsx! {
        div {
            FunctionName { function_idx, name }
            FunctionBody { function_idx }
        }
    }
}

#[component]
fn FunctionBody(function_idx: usize) -> Element {
    let program_sig = use_context::<Signal<CaoLangProgram>>();
    let (_name, func) = &program_sig.read().0.functions[function_idx];
    let cards = (0..func.cards.len()).map(|i| {
        rsx! {
            li {
                Card { idx: CardIndex::new(function_idx, i) }
            }
        }
    });
    rsx! {
        ul { class: "ml-4", {cards} }
    }
}

#[component]
fn Card(idx: CardIndex) -> Element {
    let mut program_sig = use_context::<Signal<CaoLangProgram>>();
    let program = &program_sig.read().0;
    let card = program.get_card(&idx).unwrap();

    let children = (0..card.num_children() as usize).map({
        let idx = idx.clone();
        move |i| {
            let idx = idx.clone().with_sub_index(i);
            rsx! {
                li {
                    Card { idx }
                }
            }
        }
    });

    let body = match card {
        CaoLangCard::ReadVar(name) => {
            let mut name = use_signal(|| name.clone());
            use_effect(move || {
                let mut program = program_sig.write();
                let card = program.0.get_card_mut(&idx).unwrap();
                if let CaoLangCard::ReadVar(card_name) = card {
                    *card_name = name.read().to_string();
                }
            });

            rsx! {
                div { class: "flex justify-start gap-4 w-full items-center ",
                    h3 { class: "text-xl flex-none", {card.name()} }
                    input {
                        r#type: "text",
                        class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        value: "{name}",
                        oninput: move |event| name.set(event.value()),
                    }
                }
            }
        }
        _ => {
            rsx!(
                h3 { class: "text-xl", {card.name()} }
            )
        }
    };

    rsx! {
        div {
            div { {body} }
            ul { class: "ml-4 mt-2", {children} }
        }
    }
}

#[component]
fn FunctionName(function_idx: usize, name: String) -> Element {
    let mut program_sig = use_context::<Signal<CaoLangProgram>>();
    let mut name = use_signal(move || name);
    use_effect(move || {
        let mut program = program_sig.write();
        program.0.functions[function_idx].0 = name.read().to_string();
    });
    rsx! {
        h2 { class: "text-2xl", {name} }
        input {
            r#type: "text",
            class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
            value: "{name}",
            oninput: move |event| name.set(event.value()),
        }
    }
}

#[component]
fn DebugProgram(program: Signal<CaoLangProgram>) -> Element {
    let s = format!("{:#?}", program.read().0);
    rsx! {
        pre { {s} }
    }
}
