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

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::Utc;

    use crate::models::{
        Consumable, ConsumableId, ConsumableItem, ConsumableUnit, ConsumptionConsumable,
        ConsumptionConsumableId, ConsumptionId, ConsumptionItem, ConsumptionType, NestedConsumable,
        NestedConsumableId,
    };
    use crate::models::{Consumption, UserId};

    use super::*;

    // ── helpers ───────────────────────────────────────────────────────────────

    fn make_consumable(id: i64, consumption_type: Option<ConsumptionType>) -> Consumable {
        Consumable {
            id: ConsumableId::new(id),
            name: format!("consumable-{id}"),
            brand: None,
            barcode: None,
            is_organic: false,
            unit: ConsumableUnit::Millilitres,
            comments: None,
            created: None,
            destroyed: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            consumption_type,
        }
    }

    fn make_consumable_item(
        parent_id: i64,
        child_id: i64,
        liquid_mls: Option<BigDecimal>,
        consumption_type: Option<ConsumptionType>,
    ) -> ConsumableItem {
        ConsumableItem::new(
            NestedConsumable {
                id: NestedConsumableId::new(
                    ConsumableId::new(parent_id),
                    ConsumableId::new(child_id),
                ),
                quantity: None,
                liquid_mls,
                comments: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            make_consumable(child_id, consumption_type),
        )
    }

    fn make_consumption(
        duration_secs: i64,
        liquid_mls: Option<BigDecimal>,
        consumption_type: ConsumptionType,
    ) -> Consumption {
        Consumption {
            id: ConsumptionId::new(1),
            user_id: UserId::new(1),
            time: chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00+00:00").unwrap(),
            duration: chrono::TimeDelta::seconds(duration_secs),
            consumption_type,
            liquid_mls,
            comments: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn make_consumption_item(
        consumption_id: i64,
        consumable_id: i64,
        liquid_mls: Option<BigDecimal>,
        consumption_type: Option<ConsumptionType>,
    ) -> ConsumptionItem {
        ConsumptionItem::new(
            ConsumptionConsumable {
                id: ConsumptionConsumableId::new(
                    ConsumptionId::new(consumption_id),
                    ConsumableId::new(consumable_id),
                ),
                quantity: None,
                liquid_mls,
                comments: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            make_consumable(consumable_id, consumption_type),
        )
    }

    // ── consumable_errors ─────────────────────────────────────────────────────

    #[test]
    fn consumable_no_ingredients_returns_no_errors() {
        let parent = make_consumable(1, Some(ConsumptionType::Digest));
        assert!(consumable_errors(&parent, None).is_empty());
    }

    #[test]
    fn consumable_empty_ingredient_list_returns_no_errors() {
        let parent = make_consumable(1, Some(ConsumptionType::Digest));
        assert!(consumable_errors(&parent, Some(&vec![])).is_empty());
    }

    #[test]
    fn consumable_matching_consumption_types_returns_no_errors() {
        let parent = make_consumable(1, Some(ConsumptionType::Digest));
        let ingredient = make_consumable_item(1, 2, None, Some(ConsumptionType::Digest));
        assert!(consumable_errors(&parent, Some(&vec![ingredient])).is_empty());
    }

    #[test]
    fn consumable_mismatched_consumption_type_returns_error() {
        let parent = make_consumable(1, Some(ConsumptionType::Digest));
        let ingredient = make_consumable_item(1, 2, None, Some(ConsumptionType::InhaleNose));
        let errors = consumable_errors(&parent, Some(&vec![ingredient]));
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("consumable-2"));
        assert!(errors[0].contains(ConsumptionType::InhaleNose.as_title()));
        assert!(errors[0].contains(ConsumptionType::Digest.as_title()));
    }

    #[test]
    fn consumable_none_consumption_type_on_either_side_skips_check() {
        // Parent has no consumption type — should not flag anything.
        let parent_none = make_consumable(1, None);
        let ingredient_set = make_consumable_item(1, 2, None, Some(ConsumptionType::Digest));
        assert!(consumable_errors(&parent_none, Some(&vec![ingredient_set])).is_empty());

        // Ingredient has no consumption type — should not flag anything.
        let parent_set = make_consumable(1, Some(ConsumptionType::Digest));
        let ingredient_none = make_consumable_item(1, 2, None, None);
        assert!(consumable_errors(&parent_set, Some(&vec![ingredient_none])).is_empty());
    }

    // ── consumption_errors ────────────────────────────────────────────────────

    #[test]
    fn consumption_short_duration_returns_error() {
        let c = make_consumption(1, None, ConsumptionType::Digest);
        let errors = consumption_errors(&c, None);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("suspiciously short"));
    }

    #[test]
    fn consumption_zero_duration_returns_error() {
        let c = make_consumption(0, None, ConsumptionType::Digest);
        let errors = consumption_errors(&c, None);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("suspiciously short"));
    }

    #[test]
    fn consumption_duration_exactly_2_seconds_is_ok() {
        let c = make_consumption(2, None, ConsumptionType::Digest);
        assert!(consumption_errors(&c, None).is_empty());
    }

    #[test]
    fn consumption_no_ingredients_skips_liquid_and_type_checks() {
        let c = make_consumption(10, Some(BigDecimal::from(250)), ConsumptionType::Digest);
        assert!(consumption_errors(&c, None).is_empty());
    }

    #[test]
    fn consumption_matching_liquid_mls_returns_no_errors() {
        let c = make_consumption(10, Some(BigDecimal::from(250)), ConsumptionType::Digest);
        let item = make_consumption_item(
            1,
            1,
            Some(BigDecimal::from(250)),
            Some(ConsumptionType::Digest),
        );
        assert!(consumption_errors(&c, Some(&vec![item])).is_empty());
    }

    #[test]
    fn consumption_mismatched_liquid_mls_returns_error() {
        let c = make_consumption(10, Some(BigDecimal::from(250)), ConsumptionType::Digest);
        let item = make_consumption_item(
            1,
            1,
            Some(BigDecimal::from(200)),
            Some(ConsumptionType::Digest),
        );
        let errors = consumption_errors(&c, Some(&vec![item]));
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("200"));
        assert!(errors[0].contains("250"));
    }

    #[test]
    fn consumption_mismatched_ingredient_consumption_type_returns_error() {
        // Liquid mls match so only the type error fires.
        let c = make_consumption(10, Some(BigDecimal::from(100)), ConsumptionType::Digest);
        let item = make_consumption_item(
            1,
            1,
            Some(BigDecimal::from(100)),
            Some(ConsumptionType::InhaleNose),
        );
        let errors = consumption_errors(&c, Some(&vec![item]));
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("consumable-1"));
        assert!(errors[0].contains(ConsumptionType::InhaleNose.as_title()));
        assert!(errors[0].contains(ConsumptionType::Digest.as_title()));
    }

    #[test]
    fn consumption_matching_ingredient_consumption_type_returns_no_errors() {
        let c = make_consumption(10, Some(BigDecimal::from(100)), ConsumptionType::Digest);
        let item = make_consumption_item(
            1,
            1,
            Some(BigDecimal::from(100)),
            Some(ConsumptionType::Digest),
        );
        assert!(consumption_errors(&c, Some(&vec![item])).is_empty());
    }
}
