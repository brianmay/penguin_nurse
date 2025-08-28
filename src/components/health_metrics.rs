use chrono::{DateTime, FixedOffset, Local, Utc};
use dioxus::prelude::*;

use crate::{
    forms::{
        Dialog, EditError, FieldValue, FormSaveCancelButton, InputDateTime, InputNumber,
        InputTextArea, Saving, ValidationError, validate_blood_glucose, validate_comments,
        validate_diastolic_bp, validate_fixed_offset_date_time, validate_height, validate_pulse,
        validate_systolic_bp, validate_weight,
    },
    functions::health_metrics::{create_health_metric, delete_health_metric, update_health_metric},
    models::{
        ChangeHealthMetric, HealthMetric, Maybe, MaybeDecimal, MaybeI32, MaybeString,
        NewHealthMetric, UserId,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Create { user_id: UserId },
    Update { health_metric: HealthMetric },
}

#[derive(Debug, Clone)]
struct Validate {
    time: Memo<Result<DateTime<FixedOffset>, ValidationError>>,
    pulse: Memo<Result<MaybeI32, ValidationError>>,
    blood_glucose: Memo<Result<MaybeDecimal, ValidationError>>,
    systolic_bp: Memo<Result<MaybeI32, ValidationError>>,
    diastolic_bp: Memo<Result<MaybeI32, ValidationError>>,
    weight: Memo<Result<MaybeDecimal, ValidationError>>,
    height: Memo<Result<MaybeI32, ValidationError>>,
    comments: Memo<Result<MaybeString, ValidationError>>,
}

async fn do_save(op: &Operation, validate: &Validate) -> Result<HealthMetric, EditError> {
    let time = validate.time.read().clone()?;
    let pulse = validate.pulse.read().clone()?;
    let blood_glucose = validate.blood_glucose.read().clone()?;
    let systolic_bp = validate.systolic_bp.read().clone()?;
    let diastolic_bp = validate.diastolic_bp.read().clone()?;
    let weight = validate.weight.read().clone()?;
    let height = validate.height.read().clone()?;
    let comments = validate.comments.read().clone()?;

    match op {
        Operation::Create { user_id } => {
            let updates = NewHealthMetric {
                user_id: *user_id,
                time,
                pulse,
                blood_glucose,
                systolic_bp,
                diastolic_bp,
                weight,
                height,
                comments,
            };
            create_health_metric(updates)
                .await
                .map_err(EditError::Server)
        }
        Operation::Update { health_metric } => {
            let changes = ChangeHealthMetric {
                user_id: None,
                time: Some(time),
                pulse: Some(pulse),
                blood_glucose: Some(blood_glucose),
                systolic_bp: Some(systolic_bp),
                diastolic_bp: Some(diastolic_bp),
                weight: Some(weight),
                height: Some(height),
                comments: Some(comments),
            };
            update_health_metric(health_metric.id, changes)
                .await
                .map_err(EditError::Server)
        }
    }
}

#[component]
pub fn HealthMetricUpdate(
    op: Operation,
    on_cancel: Callback,
    on_save: Callback<HealthMetric>,
) -> Element {
    let time = use_signal(|| match &op {
        Operation::Create { .. } => Utc::now().with_timezone(&Local).fixed_offset().as_string(),
        Operation::Update { health_metric } => health_metric.time.as_string(),
    });
    let pulse = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { health_metric } => health_metric.pulse.as_string(),
    });
    let blood_glucose = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { health_metric } => health_metric.blood_glucose.as_string(),
    });
    let systolic_bp = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { health_metric } => health_metric.systolic_bp.as_string(),
    });
    let diastolic_bp = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { health_metric } => health_metric.diastolic_bp.as_string(),
    });
    let weight = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { health_metric } => health_metric.weight.as_string(),
    });
    let height = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { health_metric } => health_metric.height.as_string(),
    });
    let comments = use_signal(|| match &op {
        Operation::Create { .. } => String::new(),
        Operation::Update { health_metric } => health_metric.comments.as_string(),
    });

    let validate_systolic_bp = use_memo(move || validate_systolic_bp(&systolic_bp()));

    let validate = Validate {
        time: use_memo(move || validate_fixed_offset_date_time(&time())),
        pulse: use_memo(move || validate_pulse(&pulse())),
        blood_glucose: use_memo(move || validate_blood_glucose(&blood_glucose())),
        systolic_bp: validate_systolic_bp,
        diastolic_bp: use_memo(move || {
            let v = validate_diastolic_bp(&diastolic_bp());
            match (validate_systolic_bp(), v.as_ref()) {
                (Ok(Maybe::Some(systolic)), Ok(Maybe::Some(diastolic))) => {
                    if *diastolic >= systolic {
                        return Err(ValidationError(
                            "Diastolic BP must be less than Systolic BP".to_string(),
                        ));
                    }
                }
                (Ok(Maybe::None), Ok(Maybe::None)) => { /* no extra validation needed */ }
                (Ok(Maybe::None), Ok(Maybe::Some(_))) => {
                    return Err(ValidationError(
                        "Diastolic BP cannot be set if Systolic BP is not set".to_string(),
                    ));
                }
                (Ok(Maybe::Some(_)), Ok(Maybe::None)) => {
                    return Err(ValidationError(
                        "Diastolic BP must be set if Systolic BP is set".to_string(),
                    ));
                }
                (Err(_), _) | (_, Err(_)) => { /* no extra validation needed */ }
            };
            v
        }),
        weight: use_memo(move || validate_weight(&weight())),
        height: use_memo(move || validate_height(&height())),
        comments: use_memo(move || validate_comments(&comments())),
    };

    let mut saving = use_signal(|| Saving::No);

    // disable form while waiting for response
    let disabled = use_memo(move || saving.read().is_saving());
    let disabled_save = use_memo(move || {
        validate.time.read().is_err()
            || validate.pulse.read().is_err()
            || validate.blood_glucose.read().is_err()
            || validate.systolic_bp.read().is_err()
            || validate.diastolic_bp.read().is_err()
            || validate.weight.read().is_err()
            || validate.height.read().is_err()
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
                Ok(health_metric) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_save(health_metric);
                }
                Err(err) => saving.set(Saving::Finished(Err(err))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            match &op {
                Operation::Create { .. } => "Create HealthMetric".to_string(),
                Operation::Update { health_metric } => {
                    format!("Edit HealthMetric {}", health_metric.id)
                }
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
                id: "pulse",
                label: "Pulse (bpm)",
                value: pulse,
                validate: validate.pulse,
                disabled,
            }
            InputNumber {
                id: "blood_glucose",
                label: "Blood Glucose (mmol/L)",
                value: blood_glucose,
                validate: validate.blood_glucose,
                disabled,
            }
            InputNumber {
                id: "systolic_bp",
                label: "Systolic BP (mmHg)",
                value: systolic_bp,
                validate: validate.systolic_bp,
                disabled,
            }
            InputNumber {
                id: "diastolic_bp",
                label: "Diastolic BP (mmHg)",
                value: diastolic_bp,
                validate: validate.diastolic_bp,
                disabled,
            }
            InputNumber {
                id: "weight",
                label: "Weight (kg)",
                value: weight,
                validate: validate.weight,
                disabled,
            }
            InputNumber {
                id: "height",
                label: "Height (cm)",
                value: height,
                validate: validate.height,
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
pub fn HealthMetricDelete(
    health_metric: HealthMetric,
    on_cancel: Callback,
    on_delete: Callback<HealthMetric>,
) -> Element {
    let mut saving = use_signal(|| Saving::No);

    let disabled = use_memo(move || saving.read().is_saving());

    let health_metric_clone = health_metric.clone();
    let on_save = use_callback(move |()| {
        let health_metric_clone = health_metric_clone.clone();
        spawn(async move {
            saving.set(Saving::Yes);

            match delete_health_metric(health_metric_clone.id).await {
                Ok(_) => {
                    saving.set(Saving::Finished(Ok(())));
                    on_delete(health_metric_clone.clone());
                }
                Err(err) => saving.set(Saving::Finished(Err(EditError::Server(err)))),
            }
        });
    });

    rsx! {
        h3 { class: "text-lg font-bold",
            "Delete health_metric "
            {health_metric.id.to_string()}
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

const WEE_SVG: Asset = asset!("/assets/health_metric.svg");

#[component]
pub fn health_metric_icon() -> Element {
    let alt = health_metric_title();
    rsx! {
        img { class: "w-10 dark:invert inline-block", alt, src: WEE_SVG }
    }
}

pub fn health_metric_title() -> &'static str {
    "Health Metric"
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveDialog {
    Change(Operation),
    Delete(HealthMetric),
    Idle,
}

#[component]
pub fn HealthMetricDialog(
    dialog: ActiveDialog,
    on_close: Callback,
    on_change: Callback<HealthMetric>,
    on_delete: Callback<HealthMetric>,
) -> Element {
    match dialog.clone() {
        ActiveDialog::Change(op) => {
            rsx! {
                Dialog {
                    HealthMetricUpdate { op, on_cancel: on_close, on_save: on_change }
                }
            }
        }
        ActiveDialog::Delete(health_metric) => {
            rsx! {
                Dialog {
                    HealthMetricDelete {
                        health_metric,
                        on_cancel: on_close,
                        on_delete,
                    }
                }
            }
        }
        ActiveDialog::Idle => {
            rsx! {}
        }
    }
}
