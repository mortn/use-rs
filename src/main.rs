
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing,
    Router,
};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

// Database Schema (assuming you have a 'users' table)
table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
    }
}

#[derive(Queryable, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
struct NewUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

type DbPool = Pool<ConnectionManager<PgConnection>>;
type DbConn = PooledConnection<ConnectionManager<PgConnection>>; // Corrected

async fn create_user(
    State(pool): State<DbPool>,
    Json(new_user): Json<NewUser>,
) -> Result<impl IntoResponse, StatusCode> {
    use crate::users::dsl::*;
    //let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut conn: DbConn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Updated

    let inserted_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(inserted_user)))
}

async fn read_users(State(pool): State<DbPool>) -> Result<impl IntoResponse, StatusCode> {
    use crate::users::dsl::*;
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let all_users = users
        .load::<User>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(all_users))
}

async fn read_user(
    Path(user_id): Path<i32>,
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, StatusCode> {
    use crate::users::dsl::*;
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = users
        .filter(id.eq(user_id))
        .first::<User>(&mut conn)
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

async fn update_user(
    Path(user_id): Path<i32>,
    State(pool): State<DbPool>,
    Json(update_user): Json<UpdateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    use crate::users::dsl::*;
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    diesel::update(users.filter(id.eq(user_id)))
        .set((
            update_user.name.map(|n| name.eq(n)),
            update_user.email.map(|e| email.eq(e)),
        ))
        .execute(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let updated_user = users
        .filter(id.eq(user_id))
        .first::<User>(&mut conn)
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(updated_user))
}

async fn delete_user(
    Path(user_id): Path<i32>,
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, StatusCode> {
    use crate::users::dsl::*;
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let app = Router::new()
        .route("/users", routing::post(create_user).get(routing::get(read_users)))
        .route("/users/:id", routing::get(read_user).put(routing::put(update_user)).delete(routing::delete(delete_user)))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
