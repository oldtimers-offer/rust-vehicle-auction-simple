use sqlx::PgPool;
use actix_web::{web, Responder, HttpResponse};
use crate::models::{UserRegister, UserLogin, CreateVehicle, Vehicle, CreateAuction, PlaceBid};

pub async fn user_register(pool: web::Data<PgPool>, form: web::Json<UserRegister>) -> impl Responder {
    // Implement user registration logic with Argon2 password hashing
    use argon2::{password_hash::SaltString, Argon2, PasswordHasher};

    let salt = SaltString::generate(&mut rand::thread_rng());
    let hashed_password = Argon2::default().hash_password(form.password.as_bytes(), &salt).unwrap().to_string();

    sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
    .bind(&form.username)
    .bind(hashed_password)
    .execute(pool.as_ref())
    .await
    .expect("Failed to insert user");

    "User Registered"
}

pub async fn user_login(pool: web::Data<PgPool>, form: web::Json<UserLogin>) -> impl Responder {
    // Implement user login logic with Argon2 password verification
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    let user = sqlx::query!("SELECT password FROM users WHERE username = $1", form.username)
        .fetch_one(pool.as_ref())
        .await;

    match user {
        Ok(record) => {
            let parsed_hash = PasswordHash::new(&record.password).expect("Invalid hash");
            if Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_ok() {
                "User Logged In"
            } else {
                "Invalid Credentials"
            }
        }
        Err(_) => "Invalid Credentials",
    }
}

pub async fn create_vehicle(pool: web::Data<PgPool>, form: web::Json<CreateVehicle>) -> impl Responder {
    // Implement vehicle creation logic
    sqlx::query("INSERT INTO vehicles (name, description, starting_price) VALUES ($1, $2, $3)")
    .bind(&form.name)
    .bind(&form.description)
    .bind(form.starting_price)
    .execute(pool.as_ref())
    .await
    .expect("Failed to insert vehicle");

    "Vehicle Created"
}

pub async fn list_vehicles(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Vehicle, 
        "SELECT id, name, description, starting_price FROM vehicles"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(vehicles) => HttpResponse::Ok().json(vehicles),
        Err(err) => {
            eprintln!("Error fetching vehicles: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to fetch vehicles")
        }
    }
}

pub async fn delete_vehicle(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    // Implement vehicle deletion logic
    sqlx::query!("DELETE FROM vehicles WHERE id = $1", *path)
        .execute(pool.as_ref())
        .await
        .expect("Failed to delete vehicle");

    "Vehicle Deleted"
}

pub async fn create_auction(pool: web::Data<PgPool>, form: web::Json<CreateAuction>) -> impl Responder {
    // Implement auction creation logic
    sqlx::query("INSERT INTO auctions (vehicle_id, starting_price, end_time) VALUES ($1, $2, $3)")
    .bind(form.vehicle_id)
    .bind(form.starting_price)
    .bind(&form.end_time)
    .execute(pool.as_ref())
    .await
    .expect("Failed to create auction");

    "Auction Created"
}

pub async fn place_bid(pool: web::Data<PgPool>, form: web::Json<PlaceBid>) -> impl Responder {
    // Implement bid placement logic
    sqlx::query(
        "INSERT INTO bids (auction_id, user_id, bid_amount) VALUES ($1, $2, $3)")
    .bind(form.auction_id)
    .bind(form.user_id)
    .bind(form.bid_amount )
    .execute(pool.as_ref())
    .await
    .expect("Failed to place bid");

    "Bid Placed"
}

pub async fn close_auction(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    // Implement auction closing logic
    sqlx::query!("UPDATE auctions SET closed = TRUE WHERE id = $1", *path)
        .execute(pool.as_ref())
        .await
        .expect("Failed to close auction");

    "Auction Closed"
}
