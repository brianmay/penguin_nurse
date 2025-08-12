use dioxus::prelude::*;

use crate::{
    Route,
    components::{
        buttons::{ChangeButton, DeleteButton},
        poos::{self, ActiveDialog, DialogReference, Operation, PooDialog},
    },
    functions::poos::get_poo_by_id,
    models::{Poo, PooId},
};
