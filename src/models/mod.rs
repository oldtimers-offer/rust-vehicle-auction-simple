use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;

#[derive(Deserialize)]
pub struct UserRegister {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateVehicle {
    pub name: String,
    pub description: String,
    pub starting_price: f64,
}

#[derive(Serialize)]
pub struct Vehicle {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub starting_price: BigDecimal,
}

#[derive(Deserialize)]
pub struct CreateAuction {
    pub vehicle_id: i32,
    pub starting_price: f64,
    pub end_time: String,
}

#[derive(Deserialize)]
pub struct PlaceBid {
    pub auction_id: i32,
    pub user_id: i32,
    pub bid_amount: f64,
}
