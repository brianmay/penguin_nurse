use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use penguin_nurse::server::database::models::{
    consumables::Consumable, consumptions::Consumption, consumption_consumables::ConsumptionConsumable,
    nested_consumables::NestedConsumable,
};
use penguin_nurse::server::database::schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection
    let pool = penguin_nurse::server::database::connection::init().await;
    let mut conn = pool.get().await?;

    println!("=== Penguin Nurse Data Lint ===\n");

    let mut total_errors = 0;

    // Check all consumables
    println!("Checking consumables...");
    let consumable_errors = check_consumables(&mut conn).await?;
    total_errors += consumable_errors;

    // Check all consumptions
    println!("\nChecking consumptions...");
    let consumption_errors = check_consumptions(&mut conn).await?;
    total_errors += consumption_errors;

    // Summary
    println!("\n=== Summary ===");
    if total_errors == 0 {
        println!("✓ No errors found!");
    } else {
        println!("✗ Found {} total error(s)", total_errors);
    }

    Ok(())
}

async fn check_consumables(
    conn: &mut diesel_async::pooled_connection::mobc::PooledConnection<diesel_async::AsyncPgConnection>,
) -> Result<usize, Box<dyn std::error::Error>> {
    use schema::consumables::dsl::*;
    use schema::nested_consumables;

    // Get all consumables
    let all_consumables: Vec<Consumable> = consumables.load(conn).await?;

    let mut error_count = 0;

    for consumable in all_consumables {
        // Get nested consumables for this consumable
        let nested: Vec<(NestedConsumable, Consumable)> = nested_consumables::table
            .inner_join(
                consumables
                    .on(schema::nested_consumables::consumable_id.eq(schema::consumables::id)),
            )
            .filter(schema::nested_consumables::parent_id.eq(consumable.id))
            .select((NestedConsumable::as_select(), Consumable::as_select()))
            .load(conn)
            .await?;

        // Convert to frontend models
        let frontend_consumable = penguin_nurse::models::Consumable::from(consumable.clone());
        let nested_items: Vec<penguin_nurse::models::ConsumableItem> = nested
            .into_iter()
            .map(|(nc, c)| {
                penguin_nurse::models::ConsumableItem::new(
                    penguin_nurse::models::NestedConsumable::from(nc),
                    penguin_nurse::models::Consumable::from(c),
                )
            })
            .collect();

        // Run error checks using validation module
        let errors = penguin_nurse::validation::consumable_errors(
            &frontend_consumable,
            Some(&nested_items),
        );

        if !errors.is_empty() {
            println!("\n  Consumable: {} (ID: {})", consumable.name, consumable.id);
            for error in errors {
                println!("    ✗ {}", error);
                error_count += 1;
            }
        }
    }

    if error_count == 0 {
        println!("  ✓ No consumable errors found");
    }

    Ok(error_count)
}

async fn check_consumptions(
    conn: &mut diesel_async::pooled_connection::mobc::PooledConnection<diesel_async::AsyncPgConnection>,
) -> Result<usize, Box<dyn std::error::Error>> {
    use schema::consumption_consumables;
    use schema::consumptions::dsl::*;

    // Get all consumptions
    let all_consumptions: Vec<Consumption> = consumptions.load(conn).await?;

    let mut error_count = 0;

    for consumption in all_consumptions {
        // Get consumption_consumables for this consumption
        let items: Vec<(ConsumptionConsumable, Consumable)> = consumption_consumables::table
            .inner_join(
                schema::consumables::table
                    .on(consumption_consumables::consumable_id.eq(schema::consumables::id)),
            )
            .filter(consumption_consumables::parent_id.eq(consumption.id))
            .select((
                ConsumptionConsumable::as_select(),
                Consumable::as_select(),
            ))
            .load(conn)
            .await?;

        // Convert to frontend models
        let frontend_consumption = penguin_nurse::models::Consumption::from(consumption.clone());
        let consumption_items: Vec<penguin_nurse::models::ConsumptionItem> = items
            .into_iter()
            .map(|(cc, c)| {
                penguin_nurse::models::ConsumptionItem::new(
                    penguin_nurse::models::ConsumptionConsumable::from(cc),
                    penguin_nurse::models::Consumable::from(c),
                )
            })
            .collect();

        // Run error checks using validation module
        let errors = penguin_nurse::validation::consumption_errors(
            &frontend_consumption,
            Some(&consumption_items),
        );

        if !errors.is_empty() {
            println!(
                "\n  Consumption: {} (ID: {})",
                frontend_consumption.name(),
                consumption.id
            );
            for error in errors {
                println!("    ✗ {}", error);
                error_count += 1;
            }
        }
    }

    if error_count == 0 {
        println!("  ✓ No consumption errors found");
    }

    Ok(error_count)
}
