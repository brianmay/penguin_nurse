use dioxus::prelude::*;

use super::errors::EditError;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Operation<T> {
//     Add,
//     Edit(T),
// }

// impl Operation<()> {
//     pub fn is_add(&self) -> bool {
//         matches!(self, Operation::Add)
//     }
//     pub fn is_edit(&self) -> bool {
//         matches!(self, Operation::Edit(_))
//     }
// }

pub enum Saving {
    No,
    Yes,
    Finished(Result<(), EditError>),
}

impl Saving {
    pub fn is_saving(&self) -> bool {
        matches!(self, Saving::Yes)
    }
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
