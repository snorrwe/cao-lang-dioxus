use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;

#[derive(Clone)]
pub struct CaoLangProgram(pub Rc<RefCell<cao_lang::prelude::CaoProgram>>);

#[component]
pub fn Editor() -> Element {
    let program = use_hook(|| CaoLangProgram(Default::default()));

    let json = format!("{:?}", program.0.borrow());
    rsx! {
        div {
            "hello"
        {json}

        }
    }
}
