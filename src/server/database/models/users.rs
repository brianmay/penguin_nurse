use axum_login::AuthUser;
use diesel::prelude::{AsChangeset, Insertable};
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, Queryable, Selectable, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use tap::Pipe;

use crate::server::database::connection::DatabaseConnection;
use crate::server::database::schema;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::groups)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::user_groups)]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(User))]
#[diesel(primary_key(user_id, group_id))]
pub struct UserGroup {
    pub user_id: i64,
    pub group_id: i64,
}

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub oidc_id: Option<String>,
    pub email: String,
    pub is_admin: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

impl From<User> for crate::models::User {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            full_name: user.full_name,
            oidc_id: user.oidc_id.into(),
            email: user.email,
            is_admin: user.is_admin,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub full_name: &'a str,
    pub oidc_id: Option<&'a str>,
    pub email: &'a str,
    pub is_admin: bool,
}

impl<'a> NewUser<'a> {
    pub fn from_front_end(user: &'a crate::models::NewUser, hashed_password: &'a str) -> Self {
        Self {
            username: &user.username,
            password: hashed_password,
            full_name: &user.full_name,
            oidc_id: user.oidc_id.as_deref(),
            email: &user.email,
            is_admin: user.is_admin,
        }
    }
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::users)]
pub struct UpdateUser<'a> {
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
    pub full_name: Option<&'a str>,
    pub oidc_id: Option<Option<&'a str>>,
    pub email: Option<&'a str>,
    pub is_admin: Option<bool>,
}

impl<'a> UpdateUser<'a> {
    pub fn from_front_end(
        user: &'a crate::models::UpdateUser,
        hashed_password: Option<&'a str>,
    ) -> Self {
        Self {
            username: user.username.as_deref(),
            password: hashed_password,
            full_name: user.full_name.as_deref(),
            oidc_id: user.oidc_id.as_ref().map(|x| x.as_deref()),
            email: user.email.as_deref(),
            is_admin: user.is_admin,
        }
    }
}

pub async fn get_user_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
) -> Result<Option<User>, diesel::result::Error> {
    use schema::users::id as q_id;
    use schema::users::table;

    table
        .filter(q_id.eq(id))
        .get_result(conn)
        .await
        .optional()?
        .pipe(Ok)
}

pub async fn get_user_by_username(
    conn: &mut DatabaseConnection,
    username: &str,
) -> Result<Option<User>, diesel::result::Error> {
    use schema::users::table;
    use schema::users::username as q_username;

    table
        .filter(q_username.eq(username))
        .get_result(conn)
        .await
        .optional()?
        .pipe(Ok)
}

pub async fn get_users(conn: &mut DatabaseConnection) -> Result<Vec<User>, diesel::result::Error> {
    use schema::users::table;
    table.load(conn).await
}

pub async fn create_user(
    conn: &mut DatabaseConnection,
    updates: NewUser<'_>,
) -> Result<User, diesel::result::Error> {
    use schema::users::table;

    diesel::insert_into(table)
        .values(&updates)
        .returning(User::as_returning())
        .get_result(conn)
        .await
}

pub async fn update_user(
    conn: &mut DatabaseConnection,
    id: i64,
    updates: UpdateUser<'_>,
) -> Result<User, diesel::result::Error> {
    use schema::users::id as q_id;
    use schema::users::table;

    diesel::update(table)
        .filter(q_id.eq(id))
        .set(&updates)
        .returning(User::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_user(
    conn: &mut DatabaseConnection,
    id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::users::id as q_id;
    use schema::users::table;

    table
        .filter(q_id.eq(id))
        .pipe(diesel::delete)
        .execute(conn)
        .await
        .pipe(|_| Ok(()))
}
