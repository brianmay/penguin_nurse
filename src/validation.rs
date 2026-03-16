/// Validation/linting utilities for consumables and consumptions
use crate::models::{Consumable, ConsumableItem, Consumption, ConsumptionItem};

pub fn consumable_errors(
    consumable: &Consumable,
    nested_consumables: Option<&Vec<ConsumableItem>>,
) -> Vec<String> {
    let mut errors = Vec::new();

    if let Some(nested_consumables) = nested_consumables {
        for nc in nested_consumables {
            if let (Some(nc_type), Some(consumable_type)) =
                (nc.consumable.consumption_type, consumable.consumption_type)
                && nc_type != consumable_type
            {
                errors.push(format!(
                    "Ingredient {} has consumption type {} which does not match parent consumption type {}",
                    nc.consumable.name,
                    nc_type.as_title(),
                    consumable_type.as_title(),
                ));
            }
        }
    }

    errors
}

pub fn consumption_errors(
    consumption: &Consumption,
    consumption_consumables: Option<&Vec<ConsumptionItem>>,
) -> Vec<String> {
    let mut errors = Vec::new();

    if consumption.duration.num_seconds() < 2 {
        errors.push(format!(
            "Duration {} is suspiciously short",
            consumption.duration
        ));
    }

    if let Some(consumption_consumables) = &consumption_consumables {
        let zero = bigdecimal::BigDecimal::from(0);
        let expected_mls = consumption.liquid_mls.as_ref().unwrap_or(&zero);
        let total_nested_mls: bigdecimal::BigDecimal = consumption_consumables
            .iter()
            .filter_map(|ci| ci.nested.liquid_mls.as_ref())
            .cloned()
            .sum();
        if *expected_mls != total_nested_mls {
            errors.push(format!(
                "Liquid ml total from ingredients {}ml does not match consumption liquid ml {}ml",
                total_nested_mls, expected_mls,
            ));
        }
    }

    // check for any nested consumables with consumption type that doesn't match parent
    if let Some(consumption_consumables) = &consumption_consumables {
        for ci in consumption_consumables.iter() {
            if let Some(consumption_type) = ci.consumable.consumption_type
                && consumption_type != consumption.consumption_type
            {
                errors.push(format!(
                    "Ingredient {} has consumption type {} which does not match parent consumption type {}",
                    ci.consumable.name,
                    consumption_type.as_title(),
                    consumption.consumption_type.as_title(),
                ));
            }
        }
    }

    errors
}
