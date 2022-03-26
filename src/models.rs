use diesel::{Queryable, Insertable};
use serde::{Deserialize};
use super::schema::urls;

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub url: String,
}

#[derive(Deserialize)]
pub struct UrlRequest {
    pub id: i32,
}

#[derive(Queryable)]
pub struct Url {
    pub id: i32,
    pub url: String,
    pub result: Option<String>,
    pub status: String,
}

#[derive(Insertable)]
#[table_name="urls"]
pub struct NewUrl<'a> {
    pub url: &'a str,
    pub status: &'a str,
}
