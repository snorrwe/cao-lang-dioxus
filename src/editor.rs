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

    let json = format!("{:#?}", program.read().0.borrow());
    rsx! {
        div {
            "hello"
            pre {
                {json}
            }
        }
    }
}
