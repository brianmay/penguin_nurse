use dioxus::prelude::*;

use super::errors::EditError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation<T> {
    Add,
    Edit(T),
}

#[derive(Debug, Clone, Copy)]
pub enum ActiveDialog<T> {
    Editing(Operation<T>),
    Deleting(T),
    Idle,
}

pub enum Saving {
    No,
    Yes,
    Finished(Result<(), EditError>),
}

#[component]
pub fn MyForm(children: Element) -> Element {
    rsx! {
        form {
            novalidate: true,
            action: "javascript:void(0)",
            class: "space-y-4 md:space-y-6",
            {children}
        }
    }
}
