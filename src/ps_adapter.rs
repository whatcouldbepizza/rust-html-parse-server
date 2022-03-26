use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use reqwest;
use serde_json::{json};
use std::env;

use super::models::*;
use super::schema;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn query_url(search_id: i32) -> Url {
    use schema::urls::dsl::*;

    let conn = establish_connection();
    let fetched = urls.filter(id.eq(search_id))
        .load::<Url>(&conn)
        .expect("Error fetching urls");

    if fetched.len() != 1 {
        panic!("No such url found");
    }

    Url{
        id: fetched[0].id,
        url: fetched[0].url.clone(),
        result: fetched[0].result.clone(),
        status: fetched[0].status.clone()
    }
}

pub fn insert_new_url(new_url: String) -> Url {
    use schema::urls;

    let conn = establish_connection();
    let new_object = NewUrl {
        url: new_url.as_str(),
        status: "new",
    };

    diesel::insert_into(urls::table)
        .values(&new_object)
        .get_result(&conn)
        .expect("Error creating new entry")
}

pub async fn update_statuses() {
    use schema::urls::dsl::*;

    let conn = establish_connection();
    
    loop {
        println!("update statuses started");
        let extracted = urls.filter(status.eq("new"))
            .load::<Url>(&conn)
            .expect("Error extracting urls");

        println!("1");

        for current_url in extracted.iter() {
            println!("2");
            let res = reqwest::get(current_url.url.clone()).await
                .unwrap();
            println!("3");

            match res.status() {
                reqwest::StatusCode::OK => {
                    let text = res.text().await.unwrap();
                    let analyze_res = json!({
                        "success": true,
                        "response": text
                    });
                    diesel::update(urls.filter(id.eq(current_url.id)))
                        .set((result.eq(serde_json::to_string(&analyze_res).unwrap()), status.eq("analyzed")))
                        .get_result::<Url>(&conn)
                        .expect("Failed to update row");
                }
                _ => {
                    let analyze_res = json!({
                        "succcess": false
                    });
                    diesel::update(urls.filter(id.eq(current_url.id)))
                        .set((result.eq(serde_json::to_string(&analyze_res).unwrap()), status.eq("analyzed")))
                        .get_result::<Url>(&conn)
                        .expect("Failed to update row");
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}
