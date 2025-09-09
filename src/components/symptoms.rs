use chrono::{DateTime, FixedOffset, Local, Utc};
use dioxus::prelude::*;

use crate::{
    components::events::{Markdown, event_date_time_short},
    forms::{
        Dialog, EditError, FieldValue, FormSaveCancelButton, InputDateTime, InputNumber,
        InputString, InputTextArea, Saving, ValidationError, validate_comments,
        validate_fixed_offset_date_time, validate_symptom_abdominal_pain_location,
        validate_symptom_intensity,
    },
    functions::symptoms::{create_symptom, delete_symptom, update_symptom},
    models::{ChangeSymptom, MaybeSet, NewSymptom, Symptom, UserId},
};
use classes::classes;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { symptom: Symptom },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    appetite_loss: Memo<Result<i32, ValidationError>>,
    fever: Memo<Result<i32, ValidationError>>,
    cough: Memo<Result<i32, ValidationError>>,
    sore_throat: Memo<Result<i32, ValidationError>>,
    runny_nose: Memo<Result<i32, ValidationError>>,
    sneezing: Memo<Result<i32, ValidationError>>,
    heart_burn: Memo<Result<i32, ValidationError>>,
    abdominal_pain: Memo<Result<i32, ValidationError>>,
    abdominal_pain_location: Memo<Result<Option<String>, ValidationError>>,
    diarrhea: Memo<Result<i32, ValidationError>>,
    constipation: Memo<Result<i32, ValidationError>>,
    lower_back_pain: Memo<Result<i32, ValidationError>>,
    upper_back_pain: Memo<Result<i32, ValidationError>>,
    neck_pain: Memo<Result<i32, ValidationError>>,
    joint_pain: Memo<Result<i32, ValidationError>>,
    headache: Memo<Result<i32, ValidationError>>,
    nausea: Memo<Result<i32, ValidationError>>,
    dizziness: Memo<Result<i32, ValidationError>>,
    stomach_ache: Memo<Result<i32, ValidationError>>,
    chest_pain: Memo<Result<i32, ValidationError>>,
    shortness_of_breath: Memo<Result<i32, ValidationError>>,
    fatigue: Memo<Result<i32, ValidationError>>,
    anxiety: Memo<Result<i32, ValidationError>>,
    depression: Memo<Result<i32, ValidationError>>,
    insomnia: Memo<Result<i32, ValidationError>>,
    comments: Memo<Result<Option<String>, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<Symptom, EditError> {
    let time = validate.time.read().clone()?;
    let appetite_loss = validate.appetite_loss.read().clone()?;
    let fever = validate.fever.read().clone()?;
    let cough = validate.cough.read().clone()?;
    let sore_throat = validate.sore_throat.read().clone()?;
    let runny_nose = validate.runny_nose.read().clone()?;
    let sneezing = validate.sneezing.read().clone()?;
    let heart_burn = validate.heart_burn.read().clone()?;
    let abdominal_pain = validate.abdominal_pain.read().clone()?;
    let abdominal_pain_location = validate.abdominal_pain_location.read().clone()?;
    let diarrhea = validate.diarrhea.read().clone()?;
    let constipation = validate.constipation.read().clone()?;
    let lower_back_pain = validate.lower_back_pain.read().clone()?;
    let upper_back_pain = validate.upper_back_pain.read().clone()?;
    let neck_pain = validate.neck_pain.read().clone()?;
    let joint_pain = validate.joint_pain.read().clone()?;
    let headache = validate.headache.read().clone()?;
    let nausea = validate.nausea.read().clone()?;
    let dizziness = validate.dizziness.read().clone()?;
    let stomach_ache = validate.stomach_ache.read().clone()?;
    let chest_pain = validate.chest_pain.read().clone()?;
    let shortness_of_breath = validate.shortness_of_breath.read().clone()?;
    let fatigue = validate.fatigue.read().clone()?;
    let anxiety = validate.anxiety.read().clone()?;
    let depression = validate.depression.read().clone()?;
    let insomnia = validate.insomnia.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewSymptom {
                user_id: *user_id,
                appetite_loss,
                fever,
                cough,
                sore_throat,
                runny_nose,
                sneezing,
                heart_burn,
                abdominal_pain,
                abdominal_pain_location,
                diarrhea,
                constipation,
                time,
                lower_back_pain,
                upper_back_pain,
                neck_pain,
                joint_pain,
                headache,
                nausea,
                dizziness,
                stomach_ache,
                chest_pain,
                shortness_of_breath,
                fatigue,
                anxiety,
                depression,
                insomnia,
                comments,
            };
            create_symptom(updates).await.map_err(EditError::Server)
        }
        Operation::Update { symptom } => {
            let changes = ChangeSymptom {
                user_id: MaybeSet::NoChange,
                time: MaybeSet::Set(time),
                appetite_loss: MaybeSet::Set(appetite_loss),
                fever: MaybeSet::Set(fever),
                cough: MaybeSet::Set(cough),
                sore_throat: MaybeSet::Set(sore_throat),
                runny_nose: MaybeSet::Set(runny_nose),
                sneezing: MaybeSet::Set(sneezing),
                heart_burn: MaybeSet::Set(heart_burn),
                abdominal_pain: MaybeSet::Set(abdominal_pain),
                abdominal_pain_location: MaybeSet::Set(abdominal_pain_location),
                diarrhea: MaybeSet::Set(diarrhea),
                constipation: MaybeSet::Set(constipation),
                lower_back_pain: MaybeSet::Set(lower_back_pain),
                upper_back_pain: MaybeSet::Set(upper_back_pain),
                neck_pain: MaybeSet::Set(neck_pain),
                joint_pain: MaybeSet::Set(joint_pain),
                headache: MaybeSet::Set(headache),
                nausea: MaybeSet::Set(nausea),
                dizziness: MaybeSet::Set(dizziness),
                stomach_ache: MaybeSet::Set(stomach_ache),
                chest_pain: MaybeSet::Set(chest_pain),
                shortness_of_breath: MaybeSet::Set(shortness_of_breath),
                fatigue: MaybeSet::Set(fatigue),
                anxiety: MaybeSet::Set(anxiety),
                depression: MaybeSet::Set(depression),
                insomnia: MaybeSet::Set(insomnia),
                comments: MaybeSet::Set(comments),
            };
            update_symptom(symptom.id, changes)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn SymptomUpdate(op: Operation, on_cancel: Callback, on_save: Callback<Symptom>) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_raw(),
        Operation::Update { symptom } => symptom.time.as_raw(),
    });
    let appetite_loss = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.appetite_loss.to_string(),
    });
    let fever = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.fever.to_string(),
    });
    let cough = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.cough.to_string(),
    });
    let sore_throat = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.sore_throat.to_string(),
    });
    let runny_nose = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.runny_nose.to_string(),
    });
    let sneezing = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.sneezing.to_string(),
    });
    let heart_burn = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.heart_burn.to_string(),
    });
    let abdominal_pain = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.abdominal_pain.to_string(),
    });
    let abdominal_pain_location = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { symptom } => symptom
            .abdominal_pain_location
            .as_ref()
            .map_or(String::new(), |s| s.to_string()),
    });
    let diarrhea = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.diarrhea.to_string(),
    });
    let constipation = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.constipation.to_string(),
    });
    let lower_back_pain = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.lower_back_pain.to_string(),
    });
    let upper_back_pain = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.upper_back_pain.to_string(),
    });
    let neck_pain = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.neck_pain.to_string(),
    });
    let joint_pain = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.joint_pain.to_string(),
    });
    let headache = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.headache.to_string(),
    });
    let nausea = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.nausea.to_string(),
    });
    let dizziness = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.dizziness.to_string(),
    });
    let stomach_ache = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.stomach_ache.to_string(),
    });
    let chest_pain = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.chest_pain.to_string(),
    });
    let shortness_of_breath = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.shortness_of_breath.to_string(),
    });
    let fatigue = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.fatigue.to_string(),
    });
    let anxiety = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.anxiety.to_string(),
    });
    let depression = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.depression.to_string(),
    });
    let insomnia = use_signal(|| match &op {
        Operation::Create { .. } => "0".to_string(),
        Operation::Update { symptom } => symptom.insomnia.to_string(),
    });
    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { symptom } => symptom.comments.as_raw(),
    });

    let validate = {
        let v_abdominal_pain = use_memo(move || validate_symptom_intensity(&abdominal_pain()));
        Validate {
            time: use_memo(move || validate_fixed_offset_date_time(&time())),
            appetite_loss: use_memo(move || validate_symptom_intensity(&appetite_loss())),
            fever: use_memo(move || validate_symptom_intensity(&fever())),
            cough: use_memo(move || validate_symptom_intensity(&cough())),
            sore_throat: use_memo(move || validate_symptom_intensity(&sore_throat())),
            runny_nose: use_memo(move || validate_symptom_intensity(&runny_nose())),
            sneezing: use_memo(move || validate_symptom_intensity(&sneezing())),
            heart_burn: use_memo(move || validate_symptom_intensity(&heart_burn())),
            abdominal_pain: v_abdominal_pain,
            abdominal_pain_location: use_memo(move || {
                validate_symptom_abdominal_pain_location(
                    &v_abdominal_pain(),
                    &abdominal_pain_location(),
                )
            }),
            diarrhea: use_memo(move || validate_symptom_intensity(&diarrhea())),
            constipation: use_memo(move || validate_symptom_intensity(&constipation())),
            lower_back_pain: use_memo(move || validate_symptom_intensity(&lower_back_pain())),
            upper_back_pain: use_memo(move || validate_symptom_intensity(&upper_back_pain())),
            neck_pain: use_memo(move || validate_symptom_intensity(&neck_pain())),
            joint_pain: use_memo(move || validate_symptom_intensity(&joint_pain())),
            headache: use_memo(move || validate_symptom_intensity(&headache())),
            nausea: use_memo(move || validate_symptom_intensity(&nausea())),
            dizziness: use_memo(move || validate_symptom_intensity(&dizziness())),
            stomach_ache: use_memo(move || validate_symptom_intensity(&stomach_ache())),
            chest_pain: use_memo(move || validate_symptom_intensity(&chest_pain())),
            shortness_of_breath: use_memo(move || {
                validate_symptom_intensity(&shortness_of_breath())
            }),
            fatigue: use_memo(move || validate_symptom_intensity(&fatigue())),
            anxiety: use_memo(move || validate_symptom_intensity(&anxiety())),
            depression: use_memo(move || validate_symptom_intensity(&depression())),
            insomnia: use_memo(move || validate_symptom_intensity(&insomnia())),
            comments: use_memo(move || validate_comments(&comments())),
        }
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err()
            || validate.appetite_loss.read().is_err()
            || validate.fever.read().is_err()
            || validate.cough.read().is_err()
            || validate.sore_throat.read().is_err()
            || validate.runny_nose.read().is_err()
            || validate.sneezing.read().is_err()
            || validate.heart_burn.read().is_err()
            || validate.abdominal_pain.read().is_err()
            || validate.abdominal_pain_location.read().is_err()
            || validate.diarrhea.read().is_err()
            || validate.constipation.read().is_err()
            || validate.lower_back_pain.read().is_err()
            || validate.upper_back_pain.read().is_err()
            || validate.neck_pain.read().is_err()
            || validate.joint_pain.read().is_err()
            || validate.headache.read().is_err()
            || validate.nausea.read().is_err()
            || validate.dizziness.read().is_err()
            || validate.stomach_ache.read().is_err()
            || validate.chest_pain.read().is_err()
            || validate.shortness_of_breath.read().is_err()
            || validate.fatigue.read().is_err()
            || validate.anxiety.read().is_err()
            || validate.depression.read().is_err()
            || validate.insomnia.read().is_err()
            || validate.comments.read().is_err()
            || disabled()
    });

    let op_clone = op.clone();
    let validate_clone = validate.clone();
    let on_save = use_callback(move |()| {
        let op = op_clone.clone();
        let validate = validate_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save(&op, &validate).await;

            match result {
                Ok(symptom) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(symptom);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            match &op {
                Operation::Create { .. } => "Create Symptom".to_string(),
                Operation::Update { symptom } => format!("Edit Symptom {}", symptom.id),
            }
        }
        p { class: "py-4", "Press ESC key or click the button below to close" }
        form {
            novalidate: true,
            action: "javascript:void(0)",
            method: "dialog",
            onkeyup: move |event| {
                if event.key() == Key::Escape {
                    on_cancel(());
                }
            },
            InputDateTime {
                id: "time",
                label: "Time",
                value: time,
                validate: validate.time,
                disabled,
            }
            InputNumber {
                id: "appetite_loss",
                label: "Appetite Loss (0-10)",
                value: appetite_loss,
                validate: validate.appetite_loss,
                disabled,
            }
            InputNumber {
                id: "fever",
                label: "Fever (0-10)",
                value: fever,
                validate: validate.fever,
                disabled,
            }
            InputNumber {
                id: "cough",
                label: "Cough (0-10)",
                value: cough,
                validate: validate.cough,
                disabled,
            }
            InputNumber {
                id: "sore_throat",
                label: "Sore Throat (0-10)",
                value: sore_throat,
                validate: validate.sore_throat,
                disabled,
            }
            InputNumber {
                id: "runny_nose",
                label: "Runny Nose (0-10)",
                value: runny_nose,
                validate: validate.runny_nose,
                disabled,
            }
            InputNumber {
                id: "sneezing",
                label: "Sneezing (0-10)",
                value: sneezing,
                validate: validate.sneezing,
                disabled,
            }
            InputNumber {
                id: "heart_burn",
                label: "Heart Burn (0-10)",
                value: heart_burn,
                validate: validate.heart_burn,
                disabled,
            }
            InputNumber {
                id: "abdominal_pain",
                label: "Abdominal Pain (0-10)",
                value: abdominal_pain,
                validate: validate.abdominal_pain,
                disabled,
            }
            InputString {
                id: "abdominal_pain_location",
                label: "Abdominal Pain Location",
                value: abdominal_pain_location,
                validate: validate.abdominal_pain_location,
                disabled,
            }
            InputNumber {
                id: "diarrhea",
                label: "Diarrhea (0-10)",
                value: diarrhea,
                validate: validate.diarrhea,
                disabled,
            }
            InputNumber {
                id: "constipation",
                label: "Constipation (0-10)",
                value: constipation,
                validate: validate.constipation,
                disabled,
            }
            InputNumber {
                id: "lower_back_pain",
                label: "Lower Back Pain (0-10)",
                value: lower_back_pain,
                validate: validate.lower_back_pain,
                disabled,
            }
            InputNumber {
                id: "upper_back_pain",
                label: "Upper Back Pain (0-10)",
                value: upper_back_pain,
                validate: validate.upper_back_pain,
                disabled,
            }
            InputNumber {
                id: "neck_pain",
                label: "Neck Pain (0-10)",
                value: neck_pain,
                validate: validate.neck_pain,
                disabled,
            }
            InputNumber {
                id: "joint_pain",
                label: "Joint Pain (0-10)",
                value: joint_pain,
                validate: validate.joint_pain,
                disabled,
            }
            InputNumber {
                id: "headache",
                label: "Headache (0-10)",
                value: headache,
                validate: validate.headache,
                disabled,
            }
            InputNumber {
                id: "nausea",
                label: "Nausea (0-10)",
                value: nausea,
                validate: validate.nausea,
                disabled,
            }
            InputNumber {
                id: "dizziness",
                label: "Dizziness (0-10)",
                value: dizziness,
                validate: validate.dizziness,
                disabled,
            }
            InputNumber {
                id: "stomach_ache",
                label: "Stomach Ache (0-10)",
                value: stomach_ache,
                validate: validate.stomach_ache,
                disabled,
            }
            InputNumber {
                id: "chest_pain",
                label: "Chest Pain (0-10)",
                value: chest_pain,
                validate: validate.chest_pain,
                disabled,
            }
            InputNumber {
                id: "shortness_of_breath",
                label: "Shortness of Breath (0-10)",
                value: shortness_of_breath,
                validate: validate.shortness_of_breath,
                disabled,
            }
            InputNumber {
                id: "fatigue",
                label: "Fatigue (0-10)",
                value: fatigue,
                validate: validate.fatigue,
                disabled,
            }
            InputNumber {
                id: "anxiety",
                label: "Anxiety (0-10)",
                value: anxiety,
                validate: validate.anxiety,
                disabled,
            }
            InputNumber {
                id: "depression",
                label: "Depression (0-10)",
                value: depression,
                validate: validate.depression,
                disabled,
            }
            InputNumber {
                id: "insomnia",
                label: "Insomnia (0-10)",
                value: insomnia,
                validate: validate.insomnia,
                disabled,
            }
            InputTextArea {
                id: "comments",
                label: "Comments",
                value: comments,
                validate: validate.comments,
                disabled,
            }

            FormSaveCancelButton {
                disabled: disabled_save,
                on_save: move |()| on_save(()),
                on_cancel: move |()| on_cancel(()),
                title: match &op {
                    Operation::Create { .. } => "Create",
                    Operation::Update { .. } => "Save",
                },
                saving,
            }
        }
    }
}

#[component]
pub fn SymptomDelete(
    symptom: Symptom,
    on_cancel: Callback,
    on_delete: Callback<Symptom>,
) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let symptom_clone = symptom.clone();
    let on_save = use_callback(move |()| {
        let symptom_clone = symptom_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_symptom(symptom_clone.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(symptom_clone.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            "Delete symptom "
            {symptom.id.to_string()}
        }
        p { class: "py-4", "Press ESC key or click the button below to close" }
        SymptomSummary { symptom: symptom.clone() }
        form {
            novalidate: true,
            action: "javascript:void(0)",
            method: "dialog",
            onkeyup: move |event| {
                if event.key() == Key::Escape {
                    on_cancel(());
                }
            },
            FormSaveCancelButton {
                disabled,
                on_save: move |()| on_save(()),
                on_cancel: move |_| on_cancel(()),
                title: "Delete",
                saving,
            }
        }
    }
}

const SYMPTOM_SVG: Asset = asset!("/assets/symptom.svg");

#[component]
pub fn symptom_icon() -> Element {
    let alt = symptom_title();
    rsx! {
        img { alt, src: SYMPTOM_SVG }
    }
}

pub fn symptom_title() -> &'static str {
    "Symptom"
}

#[component]
pub fn SymptomIntensity(intensity: i32) -> Element {
    let clamped_intensity = intensity.clamp(0, 10);
    let color = match intensity {
        0..=4 => classes!["text-success"],
        5..=8 => classes!["text-warning"],
        9..=10 => classes!["text-error"],
        _ => classes!["text-error"],
    };

    let mut icons = Vec::new();
    for _ in 0..clamped_intensity {
        icons.push(rsx! {
            svg {
                class: classes!["fill-current", "w-4", "inline-block"] + &color,
                "xml:space": "preserve",
                "enable-background": "new 0 0 120 120",
                "viewBox": "0 0 120 120",
                width: "800px",
                xmlns: "http://www.w3.org/2000/svg",
                version: "1.1",
                "xmlns:xlink": "http://www.w3.org/1999/xlink",
                id: "Layer_1",
                polygon { points: "41.504,39.537 60.062,0 78.618,39.538 120.115,45.877 90.088,76.653 97.176,120.107 60.061,99.593 22.946,120.107\n\t30.035,76.653 0.01,45.878 " }
            }
        });
    }
    for _ in clamped_intensity..10 {
        icons.push(rsx! {
            svg {
                class: classes!["fill-current", "w-4", "inline-block", "opacity-25"],
                "xml:space": "preserve",
                "enable-background": "new 0 0 120 120",
                "viewBox": "0 0 120 120",
                width: "800px",
                xmlns: "http://www.w3.org/2000/svg",
                version: "1.1",
                "xmlns:xlink": "http://www.w3.org/1999/xlink",
                id: "Layer_1",
                polygon { points: "41.504,39.537 60.062,0 78.618,39.538 120.115,45.877 90.088,76.653 97.176,120.107 60.061,99.593 22.946,120.107\n\t30.035,76.653 0.01,45.878 " }
            }
        });
    }

    rsx! {
        div { class: classes!["flex", "flex-row", "flex-wrap", "gap-1"] + " " + &color,
            div { class: "w-10", {intensity.to_string()} }
            {icons.iter()}
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(Symptom),
    Idle,
}

#[component]
pub fn SymptomDialog(
    dialog: ActiveDialog,
    on_close: Callback,
    on_change: Callback<Symptom>,
    on_delete: Callback<Symptom>,
) -> Element {
    match dialog.clone() {
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    SymptomUpdate { op, on_cancel: on_close, on_save: on_change }
                }
            }
        }
        ActiveDialog::Delete(symptom) => {
            rsx! {
                Dialog {
                    SymptomDelete { symptom, on_cancel: on_close, on_delete }
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}

#[component]
pub fn SymptomDisplay(name: String, intensity: i32, extra: Option<Element>) -> Element {
    rsx! {
        if intensity > 0 {
            div {
                {name}
                SymptomIntensity { intensity }
                if let Some(extra) = extra {
                    div { {extra} }
                }
            }
        }
    }
}

#[component]
pub fn SymptomSummary(symptom: Symptom) -> Element {
    rsx! {
        div { {symptom_title()} }
        div {
            event_date_time_short { time: symptom.time }
        }
        if let Some(comments) = &symptom.comments {
            Markdown { content: comments.to_string() }
        }
    }
}

#[component]
pub fn SymptomDetails(symptom: Symptom) -> Element {
    rsx! {
        h3 { class: "text-lg font-bold", {symptom.time.format("%Y-%m-%d %H:%M:%S %:z").to_string()} }
        SymptomDisplay {
            name: "Appetite Loss: ".to_string(),
            intensity: symptom.appetite_loss,
            extra: None,
        }
        SymptomDisplay {
            name: "Fever: ".to_string(),
            intensity: symptom.fever,
            extra: None,
        }
        SymptomDisplay {
            name: "Cough: ".to_string(),
            intensity: symptom.cough,
            extra: None,
        }
        SymptomDisplay {
            name: "Sore Throat: ".to_string(),
            intensity: symptom.sore_throat,
            extra: None,
        }
        SymptomDisplay {
            name: "Runny Nose: ".to_string(),
            intensity: symptom.runny_nose,
            extra: None,
        }
        SymptomDisplay {
            name: "Sneezing: ".to_string(),
            intensity: symptom.sneezing,
            extra: None,
        }
        SymptomDisplay {
            name: "Heart Burn: ".to_string(),
            intensity: symptom.heart_burn,
            extra: None,
        }
        SymptomDisplay {
            name: "Abdominal Pain: ".to_string(),
            intensity: symptom.abdominal_pain,
            extra: symptom.abdominal_pain_location.map(|location| rsx! {
                div { class: "inline-block ml-2", {location} }
            }),
        }
        SymptomDisplay {
            name: "Diarrhea: ".to_string(),
            intensity: symptom.diarrhea,
            extra: None,
        }
        SymptomDisplay {
            name: "Constipation: ".to_string(),
            intensity: symptom.constipation,
            extra: None,
        }
        SymptomDisplay {
            name: "Lower Back Pain: ".to_string(),
            intensity: symptom.lower_back_pain,
            extra: None,
        }
        SymptomDisplay {
            name: "Upper Back Pain: ".to_string(),
            intensity: symptom.upper_back_pain,
            extra: None,
        }
        SymptomDisplay {
            name: "Neck Pain: ".to_string(),
            intensity: symptom.neck_pain,
            extra: None,
        }
        SymptomDisplay {
            name: "Joint Pain: ".to_string(),
            intensity: symptom.joint_pain,
            extra: None,
        }
        SymptomDisplay {
            name: "Headache: ".to_string(),
            intensity: symptom.headache,
            extra: None,
        }
        SymptomDisplay {
            name: "Nausea: ".to_string(),
            intensity: symptom.nausea,
            extra: None,
        }
        SymptomDisplay {
            name: "Dizziness: ".to_string(),
            intensity: symptom.dizziness,
            extra: None,
        }
        SymptomDisplay {
            name: "Stomach Ache: ".to_string(),
            intensity: symptom.stomach_ache,
            extra: None,
        }
        SymptomDisplay {
            name: "Chest Pain: ".to_string(),
            intensity: symptom.chest_pain,
            extra: None,
        }
        SymptomDisplay {
            name: "Shortness of Breath: ".to_string(),
            intensity: symptom.shortness_of_breath,
            extra: None,
        }
        SymptomDisplay {
            name: "Fatigue: ".to_string(),
            intensity: symptom.fatigue,
            extra: None,
        }
        SymptomDisplay {
            name: "Anxiety: ".to_string(),
            intensity: symptom.anxiety,
            extra: None,
        }
        SymptomDisplay {
            name: "Depression: ".to_string(),
            intensity: symptom.depression,
            extra: None,
        }
        SymptomDisplay {
            name: "Insomnia: ".to_string(),
            intensity: symptom.insomnia,
            extra: None,
        }
        if let Some(comments) = &symptom.comments {
            div { class: "mt-4",
                Markdown { content: comments.to_string() }
            }
        }
    }
}
