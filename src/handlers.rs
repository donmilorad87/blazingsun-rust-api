use super::models::{NewUser, User, Item, NewProduct, Product, Items};
use super::schema::users::dsl::*;
use super::schema::products::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;


use crate::diesel::PgTextExpressionMethods;
use crate::diesel::BoolExpressionMethods;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputProduct {
    pub productname: String,
    pub productdescription: String,
    pub shortdescription: String,
    pub category: String,
    pub price: String,
    pub active: String
}

pub async fn get_products(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_products(db))
        .await
        .map(|product| HttpResponse::Ok().json(product))  
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_products(pool: web::Data<Pool>) -> Result<Items<>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    
    let items = products
        .load::<Product>(&conn)?;
    let koki = items.len();
    let origin = Items { products: items, count: koki }; 

    Ok(origin)
}

pub async fn get_products_by_page(db: web::Data<Pool>, page_id: web::Path<i32>,) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_products_by_page_id(db, page_id.into_inner()))
        .await
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_products_by_page_id(pool: web::Data<Pool>, page_id: i32) -> Result<Vec<Product>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    
    let items = products
        .offset(i64::from(page_id) * 8)
        .limit(5)
        .load::<Product>(&conn)?;
        
    Ok(items)
}

pub async fn get_product_by_id(
    db: web::Data<Pool>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_product_by_id(db, product_id.into_inner()))
            .await
            .map(|product| HttpResponse::Ok().json(product))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /users
pub async fn add_product(
    db: web::Data<Pool>,
    item: web::Json<InputProduct>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_product(db, item))
        .await
        .map(|product| HttpResponse::Created().json(product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /users/{id}
pub async fn delete_product(
    db: web::Data<Pool>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_product(db, product_id.into_inner()))
            .await
            .map(|product| HttpResponse::Ok().json(product))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_product_by_id(pool: web::Data<Pool>, product_id: i32) -> Result<Product, diesel::result::Error> {
    let conn = pool.get().unwrap();
    products.find(product_id).get_result::<Product>(&conn)
}

fn add_single_product(
    db: web::Data<Pool>,
    item: web::Json<InputProduct>,
) -> Result<Product, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_product = NewProduct {
        productname: &item.productname,
        productdescription: &item.productdescription,
        shortdescription: &item.shortdescription,
        category: &item.category,
        price: &item.price,
        active: &item.active,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(products).values(&new_product).get_result(&conn)?;
    Ok(res)
}

fn delete_single_product(db: web::Data<Pool>, product_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(products.find(product_id)).execute(&conn)?;
    Ok(count)
}

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))  
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Item<>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    
    let items = users
        .load::<User>(&conn)?;
    let koki = items.len();
    let origin = Item { users: items, count: koki }; 

    Ok(origin)
}

pub async fn get_users_by_page(db: web::Data<Pool>, page_id: web::Path<i32>,) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_users_by_page_id(db, page_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_users_by_page_id(pool: web::Data<Pool>, page_id: i32) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    
    let items = users
        .offset(i64::from(page_id) * 8)
        .limit(5)
        .load::<User>(&conn)?;
        
    Ok(items)
}

pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}