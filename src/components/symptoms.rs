use chrono::{DateTime, FixedOffset, Local, Utc};
use derive_enum_all_values::AllValues;
use dioxus::prelude::*;

use crate::{
    components::events::{EventDateTimeShort, Markdown},
    forms::{
        Dialog, EditError, FieldValue, FormSaveCancelButton, InputDateTime, InputString,
        InputSymptomIntensity, InputTextArea, Saving, ValidationError, validate_comments,
        validate_fixed_offset_date_time, validate_symptom_extra_details,
        validate_symptom_intensity,
    },
    functions::symptoms::{create_symptom, delete_symptom, update_symptom},
    models::{ChangeSymptom, MaybeSet, NewSymptom, Symptom, UserId},
};
use classes::classes;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { symptom: Symptom },
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, AllValues)]
pub enum SymptomCategory {
    General,
    Respiratory,
    Digestive,
    Musculoskeletal,
    HeadFaceMouth,
    Cardiovascular,
    Neurological,
    MentalHealth,
}

impl SymptomCategory {
    pub fn as_title(&self) -> &'static str {
        match self {
            SymptomCategory::General => "General / Systemic",
            SymptomCategory::Respiratory => "Respiratory / ENT",
            SymptomCategory::Digestive => "Digestive / GI",
            SymptomCategory::Musculoskeletal => "Musculoskeletal",
            SymptomCategory::HeadFaceMouth => "Head / Face / Mouth",
            SymptomCategory::Cardiovascular => "Cardiovascular",
            SymptomCategory::Neurological => "Neurological",
            SymptomCategory::MentalHealth => "Mental Health / Sleep",
        }
    }
}
impl std::fmt::Display for SymptomCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_title())
    }
}

pub struct SymptomExtraMeta {
    pub id: &'static str,
    pub label: &'static str,
    pub accessor: fn(&Symptom) -> Option<&String>,
    pub set_new: fn(&mut NewSymptom, Option<&String>),
    pub set_change: fn(&mut ChangeSymptom, Option<&String>),
}

pub struct SymptomMeta {
    pub id: &'static str,          // stable identifier
    pub label: &'static str,       // human-friendly name
    pub category: SymptomCategory, // grouping
    pub accessor: fn(&Symptom) -> i32,
    pub set_new: fn(&mut NewSymptom, i32),
    pub set_change: fn(&mut ChangeSymptom, i32),
    pub extra: Option<SymptomExtraMeta>,
}

#[derive(Clone)]
pub struct SymptomExtraInput {
    pub id: &'static str,
    pub label: &'static str,
    pub value: Signal<String>,
    pub validate: Memo<Result<Option<String>, ValidationError>>,
    pub meta: &'static SymptomExtraMeta,
}

#[derive(Clone)]
pub struct SymptomInput {
    pub id: &'static str,
    pub label: &'static str,
    pub category: SymptomCategory,
    pub meta: &'static SymptomMeta,
    pub value: Signal<String>,
    pub validate: Memo<Result<i32, ValidationError>>,
    pub extra: Option<SymptomExtraInput>,
}

fn build_form_inputs(op: &Operation) -> (Vec<SymptomInput>, Memo<bool>) {
    let inputs: Vec<_> = SYMPTOM_DEFS
        .iter()
        .map(|meta| {
            let initial = match op {
                Operation::Create { .. } => "0".to_string(),
                Operation::Update { symptom } => (meta.accessor)(symptom).to_string(),
            };

            let value = use_signal(|| initial);
            let validate = use_memo(move || validate_symptom_intensity(&value()));

            let extra = meta.extra.as_ref().map(|extra_meta| {
                let initial = match op {
                    Operation::Create { .. } => "".to_string(),
                    Operation::Update { symptom } => {
                        (extra_meta.accessor)(symptom).cloned().unwrap_or_default()
                    }
                };
                let extra_value = use_signal(|| initial);
                let extra_validate = use_memo({
                    move || validate_symptom_extra_details(&validate(), &extra_value())
                });
                SymptomExtraInput {
                    id: extra_meta.id,
                    label: extra_meta.label,
                    value: extra_value,
                    validate: extra_validate,
                    meta: extra_meta,
                }
            });

            SymptomInput {
                id: meta.id,
                label: meta.label,
                category: meta.category,
                meta,
                value,
                validate,
                extra,
            }
        })
        .collect();

    let has_errors = {
        let deps: Vec<Memo<_>> = inputs.iter().map(|i| i.validate).collect();
        use_memo(move || {
            deps.iter().any(|v| v().is_err()) // true if any field is invalid
        })
    };

    (inputs, has_errors)
}

fn inputs_to_new_symptom(
    inputs: &[SymptomInput],
    user_id: UserId,
    time: DateTime<FixedOffset>,
    comments: Option<String>,
) -> Result<NewSymptom, ValidationError> {
    let mut s = NewSymptom {
        comments,
        ..NewSymptom::default(user_id, time)
    };

    for input in inputs {
        let v = input.validate.read().clone()?;
        (input.meta.set_new)(&mut s, v);
        if let Some(extra) = &input.extra {
            let v = extra.validate.read().clone()?;
            (extra.meta.set_new)(&mut s, v.as_ref());
        }
    }

    Ok(s)
}

fn inputs_to_change_symptom(
    inputs: &[SymptomInput],
    time: DateTime<FixedOffset>,
    comments: Option<String>,
) -> Result<ChangeSymptom, ValidationError> {
    let mut s = ChangeSymptom {
        time: MaybeSet::Set(time),
        comments: MaybeSet::Set(comments),
        ..ChangeSymptom::default()
    };

    for input in inputs {
        let v = input.validate.read().clone()?;

        (input.meta.set_change)(&mut s, v);
        if let Some(extra) = &input.extra {
            let v = extra.validate.read().clone()?;
            (extra.meta.set_change)(&mut s, v.as_ref());
        }
    }

    Ok(s)
}

pub const SYMPTOM_DEFS: &[SymptomMeta] = &[
    SymptomMeta {
        id: "appetite_loss",
        label: "Appetite Loss",
        category: SymptomCategory::General,
        accessor: |s| s.appetite_loss,
        extra: None,
        set_new: |ns, v| ns.appetite_loss = v,
        set_change: |cs, v| cs.appetite_loss = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "fever",
        label: "Fever",
        category: SymptomCategory::General,
        accessor: |s| s.fever,
        extra: None,
        set_new: |ns, v| ns.fever = v,
        set_change: |cs, v| cs.fever = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "cough",
        label: "Cough",
        category: SymptomCategory::Respiratory,
        accessor: |s| s.cough,
        extra: None,
        set_new: |ns, v| ns.cough = v,
        set_change: |cs, v| cs.cough = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "sore_throat",
        label: "Sore Throat",
        category: SymptomCategory::Respiratory,
        accessor: |s| s.sore_throat,
        extra: None,
        set_new: |ns, v| ns.sore_throat = v,
        set_change: |cs, v| cs.sore_throat = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "nasal_symptom",
        label: "Nasal Symptom",
        category: SymptomCategory::Respiratory,
        accessor: |s| s.nasal_symptom,
        set_new: |ns, v| ns.nasal_symptom = v,
        set_change: |cs, v| cs.nasal_symptom = MaybeSet::Set(v),
        extra: Some(SymptomExtraMeta {
            id: "nasal_symptom_description",
            label: "Nasal Symptom Description",
            accessor: |s| s.nasal_symptom_description.as_ref(),
            set_new: |ns, v| ns.nasal_symptom_description = v.cloned(),
            set_change: |cs, v| cs.nasal_symptom_description = MaybeSet::Set(v.cloned()),
        }),
    },
    SymptomMeta {
        id: "sneezing",
        label: "Sneezing",
        category: SymptomCategory::Respiratory,
        accessor: |s| s.sneezing,
        set_new: |ns, v| ns.sneezing = v,
        set_change: |cs, v| cs.sneezing = MaybeSet::Set(v),
        extra: None,
    },
    SymptomMeta {
        id: "heart_burn",
        label: "Heart Burn",
        category: SymptomCategory::Digestive,
        accessor: |s| s.heart_burn,
        set_new: |ns, v| ns.heart_burn = v,
        set_change: |cs, v| cs.heart_burn = MaybeSet::Set(v),
        extra: None,
    },
    SymptomMeta {
        id: "abdominal_pain",
        label: "Abdominal Pain",
        category: SymptomCategory::Digestive,
        accessor: |s| s.abdominal_pain,
        set_new: |ns, v| ns.abdominal_pain = v,
        set_change: |cs, v| cs.abdominal_pain = MaybeSet::Set(v),
        extra: Some(SymptomExtraMeta {
            id: "abdominal_pain_location",
            label: "Abdominal Pain Location",
            accessor: |s| s.abdominal_pain_location.as_ref(),
            set_new: |ns, v| ns.abdominal_pain_location = v.cloned(),
            set_change: |cs, v| cs.abdominal_pain_location = MaybeSet::Set(v.cloned()),
        }),
    },
    SymptomMeta {
        id: "diarrhea",
        label: "Diarrhea",
        category: SymptomCategory::Digestive,
        accessor: |s| s.diarrhea,
        extra: None,
        set_new: |ns, v| ns.diarrhea = v,
        set_change: |cs, v| cs.diarrhea = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "constipation",
        label: "Constipation",
        category: SymptomCategory::Digestive,
        accessor: |s| s.constipation,
        extra: None,
        set_new: |ns, v| ns.constipation = v,
        set_change: |cs, v| cs.constipation = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "lower_back_pain",
        label: "Lower Back Pain",
        category: SymptomCategory::Musculoskeletal,
        accessor: |s| s.lower_back_pain,
        extra: None,
        set_new: |ns, v| ns.lower_back_pain = v,
        set_change: |cs, v| cs.lower_back_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "upper_back_pain",
        label: "Upper Back Pain",
        category: SymptomCategory::Musculoskeletal,
        accessor: |s| s.upper_back_pain,
        extra: None,
        set_new: |ns, v| ns.upper_back_pain = v,
        set_change: |cs, v| cs.upper_back_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "neck_pain",
        label: "Neck Pain",
        category: SymptomCategory::Musculoskeletal,
        accessor: |s| s.neck_pain,
        extra: None,
        set_new: |ns, v| ns.neck_pain = v,
        set_change: |cs, v| cs.neck_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "joint_pain",
        label: "Joint Pain",
        category: SymptomCategory::Musculoskeletal,
        accessor: |s| s.joint_pain,
        extra: None,
        set_new: |ns, v| ns.joint_pain = v,
        set_change: |cs, v| cs.joint_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "headache",
        label: "Headache",
        category: SymptomCategory::HeadFaceMouth,
        accessor: |s| s.headache,
        extra: None,
        set_new: |ns, v| ns.headache = v,
        set_change: |cs, v| cs.headache = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "nausea",
        label: "Nausea",
        category: SymptomCategory::Digestive,
        accessor: |s| s.nausea,
        extra: None,
        set_new: |ns, v| ns.nausea = v,
        set_change: |cs, v| cs.nausea = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "dizziness",
        label: "Dizziness",
        category: SymptomCategory::Neurological,
        accessor: |s| s.dizziness,
        extra: None,
        set_new: |ns, v| ns.dizziness = v,
        set_change: |cs, v| cs.dizziness = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "stomach_ache",
        label: "Stomach Ache",
        category: SymptomCategory::Digestive,
        accessor: |s| s.stomach_ache,
        extra: None,
        set_new: |ns, v| ns.stomach_ache = v,
        set_change: |cs, v| cs.stomach_ache = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "chest_pain",
        label: "Chest Pain",
        category: SymptomCategory::Cardiovascular,
        accessor: |s| s.chest_pain,
        extra: None,
        set_new: |ns, v| ns.chest_pain = v,
        set_change: |cs, v| cs.chest_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "shortness_of_breath",
        label: "Shortness of Breath",
        category: SymptomCategory::Respiratory,
        accessor: |s: &Symptom| s.shortness_of_breath,
        extra: None,
        set_new: |ns, v| ns.shortness_of_breath = v,
        set_change: |cs, v| cs.shortness_of_breath = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "fatigue",
        label: "Fatigue",
        category: SymptomCategory::General,
        accessor: |s| s.fatigue,
        extra: None,
        set_new: |ns, v| ns.fatigue = v,
        set_change: |cs, v| cs.fatigue = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "anxiety",
        label: "Anxiety",
        category: SymptomCategory::MentalHealth,
        accessor: |s| s.anxiety,
        extra: None,
        set_new: |ns, v| ns.anxiety = v,
        set_change: |cs, v| cs.anxiety = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "depression",
        label: "Depression",
        category: SymptomCategory::MentalHealth,
        accessor: |s| s.depression,
        extra: None,
        set_new: |ns, v| ns.depression = v,
        set_change: |cs, v| cs.depression = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "insomnia",
        label: "Insomnia",
        category: SymptomCategory::MentalHealth,
        accessor: |s| s.insomnia,
        extra: None,
        set_new: |ns, v| ns.insomnia = v,
        set_change: |cs, v| cs.insomnia = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "shoulder_pain",
        label: "Shoulder Pain",
        category: SymptomCategory::Musculoskeletal,
        accessor: |s| s.shoulder_pain,
        extra: None,
        set_new: |ns, v| ns.shoulder_pain = v,
        set_change: |cs, v| cs.shoulder_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "hand_pain",
        label: "Hand Pain",
        category: SymptomCategory::Musculoskeletal,
        accessor: |s| s.hand_pain,
        extra: None,
        set_new: |ns, v| ns.hand_pain = v,
        set_change: |cs, v| cs.hand_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "wrist_pain",
        label: "Wrist Pain",
        category: SymptomCategory::Musculoskeletal,
        accessor: |s| s.wrist_pain,
        extra: None,
        set_new: |ns, v| ns.wrist_pain = v,
        set_change: |cs, v| cs.wrist_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "foot_pain",
        label: "Foot Pain",
        category: SymptomCategory::Musculoskeletal,
        accessor: |s| s.foot_pain,
        extra: None,
        set_new: |ns, v| ns.foot_pain = v,
        set_change: |cs, v| cs.foot_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "dental_pain",
        label: "Dental Pain",
        category: SymptomCategory::HeadFaceMouth,
        accessor: |s| s.dental_pain,
        extra: None,
        set_new: |ns, v| ns.dental_pain = v,
        set_change: |cs, v| cs.dental_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "eye_pain",
        label: "Eye Pain",
        category: SymptomCategory::HeadFaceMouth,
        accessor: |s| s.eye_pain,
        extra: None,
        set_new: |ns, v| ns.eye_pain = v,
        set_change: |cs, v| cs.eye_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "ear_pain",
        label: "Ear Pain",
        category: SymptomCategory::HeadFaceMouth,
        accessor: |s| s.ear_pain,
        extra: None,
        set_new: |ns, v| ns.ear_pain = v,
        set_change: |cs, v| cs.ear_pain = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "feeling_hot",
        label: "Feeling Hot",
        category: SymptomCategory::General,
        accessor: |s| s.feeling_hot,
        extra: None,
        set_new: |ns, v| ns.feeling_hot = v,
        set_change: |cs, v| cs.feeling_hot = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "feeling_cold",
        label: "Feeling Cold",
        category: SymptomCategory::General,
        accessor: |s| s.feeling_cold,
        extra: None,
        set_new: |ns, v| ns.feeling_cold = v,
        set_change: |cs, v| cs.feeling_cold = MaybeSet::Set(v),
    },
    SymptomMeta {
        id: "feeling_thirsty",
        label: "Feeling Thirsty",
        category: SymptomCategory::General,
        accessor: |s| s.feeling_thirsty,
        extra: None,
        set_new: |ns, v| ns.feeling_thirsty = v,
        set_change: |cs, v| cs.feeling_thirsty = MaybeSet::Set(v),
    },
];

#[derive(Debug, Clone)]
pub struct SymptomExtraField<'a> {
    pub label: &'static str,
    pub value: Option<&'a String>,
}

#[derive(Debug, Clone)]
pub struct SymptomField<'a> {
    pub label: &'static str,
    pub intensity: i32,
    pub extra: Option<SymptomExtraField<'a>>,
    // pub category: SymptomCategory,
}

pub fn collect_symptom_fields<'a>(
    symptom: &'a Symptom,
    category: SymptomCategory,
) -> Vec<SymptomField<'a>> {
    SYMPTOM_DEFS
        .iter()
        .filter(|meta| meta.category == category)
        .filter_map(|meta| {
            let intensity = (meta.accessor)(symptom);
            let extra = meta.extra.as_ref().map(|e| SymptomExtraField {
                label: e.label,
                value: (e.accessor)(symptom),
            });

            let has_value =
                intensity > 0 || extra.as_ref().map(|s| s.value.is_some()).unwrap_or(false);

            if has_value {
                Some(SymptomField {
                    label: meta.label,
                    intensity,
                    extra,
                    // category,
                })
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    comments: Memo<Result<Option<String>, ValidationError>>,
}

async fn do_save(
    op: &Operation,
    validate: &Validate,
    input: &[SymptomInput],
) -> Result<Symptom, EditError> {
    let time = validate.time.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = inputs_to_new_symptom(input, *user_id, time, comments)
                .map_err(EditError::Validation)?;
            create_symptom(updates).await.map_err(EditError::Server)
        }
        Operation::Update { symptom } => {
            let changes =
                inputs_to_change_symptom(input, time, comments).map_err(EditError::Validation)?;
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
    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { symptom } => symptom.comments.as_raw(),
    });
    let (inputs, has_errors) = build_form_inputs(&op);

    let validate = {
        Validate {
            time: use_memo(move || validate_fixed_offset_date_time(&time())),
            comments: use_memo(move || validate_comments(&comments())),
        }
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || has_errors() || disabled());

    let op_clone = op.clone();
    let validate_clone = validate.clone();
    let inputs_clone = inputs.clone();
    let on_save = use_callback(move |()| {
        let op = op_clone.clone();
        let validate = validate_clone.clone();
        let inputs = inputs_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            let result = do_save(&op, &validate, &inputs).await;

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
            InputTextArea {
                id: "comments",
                label: "Comments",
                value: comments,
                validate: validate.comments,
                disabled,
            }
            for category in SymptomCategory::all_values() {
                {
                    let fields: Vec<_> = inputs.iter().filter(|i| i.category == *category).collect();
                    rsx! {
                        if !fields.is_empty() {
                            fieldset { class: "fieldset border-2 rounded-md p-4 mb-4",
                                legend { class: "fieldset-legend px-2", "{category}" }
                                for field in fields {
                                    InputSymptomIntensity {
                                        id: field.id,
                                        label: field.label,
                                        value: field.value,
                                        validate: field.validate,
                                        disabled,
                                    }
                                    if let Some(extra) = &field.extra {
                                        InputString {
                                            id: extra.id,
                                            label: extra.label,
                                            value: extra.value,
                                            validate: extra.validate,
                                            disabled,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
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
            EventDateTimeShort { time: symptom.time }
        }
        if let Some(comments) = &symptom.comments {
            Markdown { content: comments.to_string() }
        }
    }
}

#[component]
pub fn SymptomDetails(symptom: Symptom) -> Element {
    //  extra: symptom.nasal_symptom_description.map(|desc| rsx! {
    //         div { class: "inline-block ml-2", {desc} }
    //     }),
    rsx! {
        h3 { class: "text-lg font-bold", {symptom.time.format("%Y-%m-%d %H:%M:%S %:z").to_string()} }
        for category in SymptomCategory::all_values() {
            {
                let fields = collect_symptom_fields(&symptom, *category);
                rsx! {
                    if !fields.is_empty() {
                        fieldset { class: "fieldset border-2 rounded-md px-4 pb-4 mb-4",
                            legend { class: "fieldset-legend px-2", "{category}" }
                            for field in fields {
                                SymptomDisplay {
                                    name: field.label.to_string(),
                                    intensity: field.intensity,
                                    extra: rsx! {
                                        if let Some(extra) = field.extra.as_ref() {
                                            if let Some(value) = extra.value {
                                                div {
                                                    {extra.label}
                                                    {": "}
                                                    span { {value.clone()} }
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(comments) = &symptom.comments {
            div { class: "mt-4",
                Markdown { content: comments.to_string() }
            }
        }
    }
}
