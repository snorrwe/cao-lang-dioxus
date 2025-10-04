use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;

const FIB_PROG: &[u8] = include_bytes!("../assets/fibonacci_program_recursive.yaml");

#[derive(Clone)]
pub struct CaoLangProgram(pub Rc<RefCell<cao_lang::prelude::CaoProgram>>);

#[component]
pub fn Editor() -> Element {
    let program = use_signal(|| {
        let program: cao_lang::prelude::CaoProgram = serde_yaml::from_slice(&FIB_PROG).unwrap();
        CaoLangProgram(Rc::new(RefCell::new(program)))
    });

    rsx! {
        div { class: "mx-auto",

            "hello"
            Program { program }
        }
    }
}

#[component]
fn Program(program: Signal<CaoLangProgram>) -> Element {
    rsx! {
        ul {
            {
                (0..program.read().0.borrow().functions.len())
                    .map(|func_idx| {
                        rsx! {
                            li {
                                Function { function_idx: func_idx, program_sig: program }
                            }
                        }
                    })
            }
        }
    }
}

#[component]
fn Function(function_idx: usize, program_sig: Signal<CaoLangProgram>) -> Element {
    let program = program_sig.read();
    let program = program.0.borrow();
    let func = program.functions.get(function_idx);
    let Some((name, func)) = func else {
        return rsx! {
            div { class: "text-red text-4xl", "Function not found" }
        };
    };
    rsx! {
        div {
            h2 { class: "text-2xl", {name.clone()} }
            FunctionName { function_idx, program_sig, name }
        }
    }
}

#[component]
fn FunctionName(function_idx: usize, program_sig: Signal<CaoLangProgram>, name: String) -> Element {
    let mut name = use_signal(move || name);
    use_effect(move || {
        let program = program_sig.write();
        let mut program = program.0.borrow_mut();
        program.functions[function_idx].0 = name.read().to_string();
    });
    rsx! {
        input {
            r#type: "text",
            class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
            value: "{name}",
            oninput: move |event| name.set(event.value()),
        }
    }
}
