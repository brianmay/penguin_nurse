use dioxus::prelude::*;

use crate::components::{poos::PooDialog, wees::WeeDialog};

use super::{
    poos::{self},
    wees::{self},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Wee(wees::ActiveDialog),
    Poo(poos::ActiveDialog),
    Idle,
}

#[component]
pub fn TimelineDialog(dialog: Signal<ActiveDialog>, on_change: Callback<()>) -> Element {
    match dialog() {
        ActiveDialog::Wee(wee_dialog) => {
            rsx! {
                WeeDialog {
                    dialog: wee_dialog,
                    on_close: move || dialog.set(ActiveDialog::Idle),
                    on_change: move |_wee| {
                        on_change(());
                        dialog.set(ActiveDialog::Idle);
                    },
                    on_delete: move |_wee| {
                        on_change(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
        ActiveDialog::Poo(poo_dialog) => {
            rsx! {
                PooDialog {
                    dialog: poo_dialog,
                    on_close: move || dialog.set(ActiveDialog::Idle),
                    on_change: move |_poo| {
                        on_change(());
                        dialog.set(ActiveDialog::Idle);
                    },
                    on_delete: move |_poo| {
                        on_change(());
                        dialog.set(ActiveDialog::Idle);
                    },
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}
