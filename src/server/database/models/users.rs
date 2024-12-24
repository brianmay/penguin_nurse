use axum_login::AuthUser;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, Queryable};
use diesel_async::RunQueryDsl;
use tap::Pipe;

use crate::server::database::connection::DatabaseConnection;
use crate::server::database::schema;

#[derive(Queryable, Debug)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Queryable, Debug)]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(User))]
#[diesel(primary_key(user_id, group_id))]
pub struct UserGroup {
    pub user_id: i64,
    pub group_id: i64,
}

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub password: String,
    pub oidc_id: Option<String>,
    pub email: String,
    pub is_admin: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for crate::models::User {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            full_name: user.full_name,
            oidc_id: user.oidc_id,
            email: user.email,
            is_admin: user.is_admin,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
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
