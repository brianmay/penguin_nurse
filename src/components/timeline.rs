use dioxus::prelude::*;

use crate::components::{consumptions::ConsumptionDialog, poos::PooDialog, wees::WeeDialog};

use super::{consumptions, poos, wees};

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Wee(wees::ActiveDialog),
    Poo(poos::ActiveDialog),
    Consumption(consumptions::ActiveDialog),
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
        ActiveDialog::Consumption(consumption_dialog) => {
            rsx! {
                ConsumptionDialog {
                    dialog: consumption_dialog,
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
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}
