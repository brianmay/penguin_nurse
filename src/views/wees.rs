use dioxus::prelude::*;

use crate::{
    Route,
    components::{
        buttons::{ChangeButton, DeleteButton},
        wees::{self, ActiveDialog, DialogReference, Operation, WeeDialog},
    },
    functions::wees::get_wee_by_id,
    models::{Wee, WeeId},
};
