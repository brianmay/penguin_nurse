use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use penguin_nurse::server::database::connection::DatabaseConnection;
use penguin_nurse::server::database::models::{
    consumables::Consumable, consumption_consumables::ConsumptionConsumable,
    consumptions::Consumption, exercises::Exercise, nested_consumables::NestedConsumable,
    poos::Poo, refluxs::Reflux, wees::Wee,
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

    // Check all exercises
    println!("\nChecking exercises...");
    let exercise_errors = check_exercises(&mut conn).await?;
    total_errors += exercise_errors;

    // Check all refluxs
    println!("\nChecking refluxs...");
    let reflux_errors = check_refluxs(&mut conn).await?;
    total_errors += reflux_errors;

    // Check all wees
    println!("\nChecking wees...");
    let wee_errors = check_wees(&mut conn).await?;
    total_errors += wee_errors;

    // Check all poos
    println!("\nChecking poos...");
    let poo_errors = check_poos(&mut conn).await?;
    total_errors += poo_errors;

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
    conn: &mut DatabaseConnection,
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
        let errors =
            penguin_nurse::validation::consumable_errors(&frontend_consumable, Some(&nested_items));

        if !errors.is_empty() {
            println!(
                "\n  Consumable: {} (ID: {})",
                consumable.name, consumable.id
            );
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
    conn: &mut DatabaseConnection,
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
            .select((ConsumptionConsumable::as_select(), Consumable::as_select()))
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

async fn check_exercises(
    conn: &mut DatabaseConnection,
) -> Result<usize, Box<dyn std::error::Error>> {
    use schema::exercises::dsl::*;

    let all_exercises: Vec<Exercise> = exercises.select(Exercise::as_select()).load(conn).await?;
    let mut error_count = 0;

    for exercise in all_exercises {
        let frontend_exercise = penguin_nurse::models::Exercise::from(exercise.clone());
        let errors = penguin_nurse::validation::exercise_errors(&frontend_exercise);

        if !errors.is_empty() {
            println!(
                "\n  Exercise: {} (ID: {})",
                frontend_exercise.exercise_type.as_title(),
                exercise.id
            );
            for error in errors {
                println!("    ✗ {}", error);
                error_count += 1;
            }
        }
    }

    if error_count == 0 {
        println!("  ✓ No exercise errors found");
    }

    Ok(error_count)
}

async fn check_refluxs(conn: &mut DatabaseConnection) -> Result<usize, Box<dyn std::error::Error>> {
    use schema::refluxs::dsl::*;

    let all_refluxs: Vec<Reflux> = refluxs.select(Reflux::as_select()).load(conn).await?;
    let mut error_count = 0;

    for reflux in all_refluxs {
        let frontend_reflux = penguin_nurse::models::Reflux::from(reflux.clone());
        let errors = penguin_nurse::validation::reflux_errors(&frontend_reflux);

        if !errors.is_empty() {
            println!("\n  Reflux: (ID: {})", reflux.id);
            for error in errors {
                println!("    ✗ {}", error);
                error_count += 1;
            }
        }
    }

    if error_count == 0 {
        println!("  ✓ No reflux errors found");
    }

    Ok(error_count)
}

async fn check_wees(conn: &mut DatabaseConnection) -> Result<usize, Box<dyn std::error::Error>> {
    use schema::wees::dsl::*;

    let all_wees: Vec<Wee> = wees.select(Wee::as_select()).load(conn).await?;
    let mut error_count = 0;

    for wee in all_wees {
        let frontend_wee = penguin_nurse::models::Wee::from(wee.clone());
        let errors = penguin_nurse::validation::wee_errors(&frontend_wee);

        if !errors.is_empty() {
            println!("\n  Wee: (ID: {})", wee.id);
            for error in errors {
                println!("    ✗ {}", error);
                error_count += 1;
            }
        }
    }

    if error_count == 0 {
        println!("  ✓ No wee errors found");
    }

    Ok(error_count)
}

async fn check_poos(conn: &mut DatabaseConnection) -> Result<usize, Box<dyn std::error::Error>> {
    use schema::poos::dsl::*;

    let all_poos: Vec<Poo> = poos.select(Poo::as_select()).load(conn).await?;
    let mut error_count = 0;

    for poo in all_poos {
        let frontend_poo = penguin_nurse::models::Poo::from(poo.clone());
        let errors = penguin_nurse::validation::poo_errors(&frontend_poo);

        if !errors.is_empty() {
            println!("\n  Poo: (ID: {})", poo.id);
            for error in errors {
                println!("    ✗ {}", error);
                error_count += 1;
            }
        }
    }

    if error_count == 0 {
        println!("  ✓ No poo errors found");
    }

    Ok(error_count)
}
