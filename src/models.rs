use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Item {
    pub users: Vec<User>,
    pub count: usize
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub productname: String,
    pub productdescription: String,
    pub shortdescription: String,
    pub category: String,
    pub price: String,
    pub active: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Items {
    pub products: Vec<Product>,
    pub count: usize
}

#[derive(Insertable, Debug)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub productname: &'a str,
    pub productdescription: &'a str,
    pub shortdescription: &'a str,
    pub category: &'a str,
    pub price: &'a str,
    pub active: &'a str,
    pub created_at: chrono::NaiveDateTime
}