use penguin_nurse::models::{
    ConsumableItem, ConsumptionItem, consumable_errors, consumption_errors,
};
use penguin_nurse::server::database::connection;
use penguin_nurse::server::database::models::consumables::get_all_consumables_with_nested;
use penguin_nurse::server::database::models::consumptions::get_all_consumptions_with_items;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let db = connection::init().await;
    let mut conn = db.get().await?;

    let mut total_errors = 0usize;

    // Lint consumables
    let consumables = get_all_consumables_with_nested(&mut conn).await?;
    for (consumable, nested) in consumables {
        let items: Vec<ConsumableItem> = nested
            .into_iter()
            .map(|(nc, c)| ConsumableItem::new(nc.into(), c.into()))
            .collect();
        let consumable_model = penguin_nurse::models::Consumable::from(consumable);
        let errors = consumable_errors(&consumable_model, Some(&items));
        for err in &errors {
            println!("Consumable {}: {}", consumable_model.name, err);
        }
        total_errors += errors.len();
    }

    // Lint consumptions
    let consumptions = get_all_consumptions_with_items(&mut conn).await?;
    for (consumption, items) in consumptions {
        let consumption_items: Vec<ConsumptionItem> = items
            .into_iter()
            .map(|(cc, c)| ConsumptionItem::new(cc.into(), c.into()))
            .collect();
        let consumption_model = penguin_nurse::models::Consumption::from(consumption);
        let errors = consumption_errors(&consumption_model, Some(&consumption_items));
        for err in &errors {
            println!(
                "Consumption {} ({}): {}",
                consumption_model.id, consumption_model.time, err
            );
        }
        total_errors += errors.len();
    }

    if total_errors == 0 {
        println!("No errors found.");
    } else {
        println!("{} error(s) found.", total_errors);
        std::process::exit(1);
    }

    Ok(())
}
