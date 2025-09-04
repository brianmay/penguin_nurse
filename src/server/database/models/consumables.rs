use std::collections::HashMap;

use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use chrono::DateTime;
use chrono::Utc;
use tap::Pipe;

use crate::models;
use crate::server::database::{connection::DatabaseConnection, schema};

use super::nested_consumables::NestedConsumable;

#[derive(diesel_derive_enum::DbEnum, Debug, Copy, Clone)]
#[db_enum(existing_type_path = "schema::sql_types::ConsumableUnit")]
pub enum ConsumableUnit {
    Millilitres,
    Grams,
    InternationalUnits,
    Number,
}

impl From<ConsumableUnit> for models::ConsumableUnit {
    fn from(unit: ConsumableUnit) -> models::ConsumableUnit {
        match unit {
            ConsumableUnit::Millilitres => models::ConsumableUnit::Millilitres,
            ConsumableUnit::Grams => models::ConsumableUnit::Grams,
            ConsumableUnit::InternationalUnits => models::ConsumableUnit::InternationalUnits,
            ConsumableUnit::Number => models::ConsumableUnit::Number,
        }
    }
}

impl From<models::ConsumableUnit> for ConsumableUnit {
    fn from(unit: models::ConsumableUnit) -> ConsumableUnit {
        match unit {
            models::ConsumableUnit::Millilitres => ConsumableUnit::Millilitres,
            models::ConsumableUnit::Grams => ConsumableUnit::Grams,
            models::ConsumableUnit::InternationalUnits => ConsumableUnit::InternationalUnits,
            models::ConsumableUnit::Number => ConsumableUnit::Number,
        }
    }
}

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::consumables)]
pub struct Consumable {
    pub id: i64,
    pub name: String,
    pub brand: Option<String>,
    pub barcode: Option<String>,
    pub is_organic: bool,
    pub unit: ConsumableUnit,
    pub comments: Option<String>,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub destroyed: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Consumable> for crate::models::Consumable {
    fn from(consumable: Consumable) -> Self {
        Self {
            id: models::ConsumableId::new(consumable.id),
            name: consumable.name,
            brand: consumable.brand,
            barcode: consumable.barcode,
            is_organic: consumable.is_organic,
            unit: consumable.unit.into(),
            created: consumable.created,
            destroyed: consumable.destroyed,
            created_at: consumable.created_at,
            updated_at: consumable.updated_at,
            comments: consumable.comments,
        }
    }
}

pub async fn search_consumables_with_nested(
    conn: &mut DatabaseConnection,
    search: &str,
    include_only_created: bool,
    include_destroyed: bool,
) -> Result<Vec<(Consumable, Vec<(NestedConsumable, Consumable)>)>, diesel::result::Error> {
    use crate::server::database::schema::consumables::dsl as q;
    use crate::server::database::schema::consumables::table;
    use crate::server::database::schema::nested_consumables::dsl as q_nested;
    use crate::server::database::schema::nested_consumables::table as nested_table;

    let consumables =
        search_consumables(conn, search, include_only_created, include_destroyed).await?;

    let nested: Vec<(NestedConsumable, Consumable)> = nested_table
        .filter(q_nested::parent_id.eq_any(consumables.iter().map(|x| x.id)))
        .inner_join(table.on(q::id.eq(q_nested::consumable_id)))
        .load(conn)
        .await?;

    let id_indices: HashMap<_, _> = consumables
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id, i))
        .collect();

    let mut result: Vec<_> = consumables.into_iter().map(|c| (c, Vec::new())).collect();

    for child in nested {
        if let Some(index) = child.0.parent_id.pipe(|i| id_indices.get(&i)) {
            result[*index].1.push(child);
        }
    }

    Ok(result)
}

pub async fn search_consumables(
    conn: &mut DatabaseConnection,
    search: &str,
    include_only_created: bool,
    include_destroyed: bool,
) -> Result<Vec<Consumable>, diesel::result::Error> {
    use crate::server::database::schema::consumables::dsl as q;
    use crate::server::database::schema::consumables::table;

    table
        .filter(
            q::name.ilike(format!("%{}%", search)).or(q::brand
                .ilike(format!("%{}%", search))
                .or(q::barcode.eq(search))),
        )
        .order((q::created.desc(), q::destroyed.desc(), q::name.asc()))
        .limit(10)
        .into_boxed()
        .pipe(|x| {
            if include_only_created {
                x.filter(q::created.is_not_null())
            } else {
                x
            }
        })
        .pipe(|x| {
            if include_destroyed {
                x
            } else {
                x.filter(q::destroyed.is_null())
            }
        })
        .get_results(conn)
        .await
}

pub async fn get_consumable_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
) -> Result<Option<Consumable>, diesel::result::Error> {
    use crate::server::database::schema::consumables::id as q_id;
    use crate::server::database::schema::consumables::table;

    table.filter(q_id.eq(id)).get_result(conn).await.optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::consumables)]
pub struct NewConsumable<'a> {
    pub name: &'a str,
    pub brand: Option<&'a str>,
    pub barcode: Option<&'a str>,
    pub is_organic: bool,
    pub unit: ConsumableUnit,
    pub comments: Option<&'a str>,
    pub created: Option<DateTime<Utc>>,
    pub destroyed: Option<DateTime<Utc>>,
}

impl<'a> NewConsumable<'a> {
    pub fn from_front_end(consumable: &'a crate::models::NewConsumable) -> Self {
        Self {
            name: consumable.name.as_ref(),
            brand: consumable.brand.as_deref(),
            barcode: consumable.barcode.as_deref(),
            is_organic: consumable.is_organic,
            unit: consumable.unit.into(),
            comments: consumable.comments.as_deref(),
            created: consumable.created.as_ref().copied(),
            destroyed: consumable.destroyed.as_ref().copied(),
        }
    }
}

pub async fn create_consumable(
    conn: &mut DatabaseConnection,
    update: &NewConsumable<'_>,
) -> Result<Consumable, diesel::result::Error> {
    diesel::insert_into(schema::consumables::table)
        .values(update)
        .returning(Consumable::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::consumables)]
pub struct ChangeConsumable<'a> {
    pub name: Option<&'a str>,
    pub brand: Option<Option<&'a str>>,
    pub barcode: Option<Option<&'a str>>,
    pub is_organic: Option<bool>,
    pub unit: Option<ConsumableUnit>,
    pub comments: Option<Option<&'a str>>,
    pub created: Option<Option<DateTime<Utc>>>,
    pub destroyed: Option<Option<DateTime<Utc>>>,
}

impl<'a> ChangeConsumable<'a> {
    pub fn from_front_end(consumable: &'a crate::models::ChangeConsumable) -> Self {
        Self {
            name: consumable.name.as_deref().into_option(),
            brand: consumable.brand.map_inner_deref().into_option(),
            barcode: consumable.barcode.map_inner_deref().into_option(),
            is_organic: consumable.is_organic.into_option(),
            unit: consumable.unit.map_into().into_option(),
            comments: consumable.comments.map_inner_deref().into_option(),
            created: consumable.created.into_option(),
            destroyed: consumable.destroyed.into_option(),
        }
    }
}

pub async fn update_consumable(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeConsumable<'_>,
) -> Result<Consumable, diesel::result::Error> {
    diesel::update(schema::consumables::table.filter(schema::consumables::id.eq(id)))
        .set(update)
        .returning(Consumable::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_consumable(
    conn: &mut DatabaseConnection,
    id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::consumables::id as q_id;
    use schema::consumables::table;

    diesel::delete(table.filter(q_id.eq(id)))
        .execute(conn)
        .await?;
    Ok(())
}
