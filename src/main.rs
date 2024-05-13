use rocket::launch;  

mod models;
mod db;
mod handlers;

use crate::db::Db;
use rocket::routes;
use crate::handlers::*; 

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Db::new()) 
        .mount("/", routes![health, get_records, add_record, delete_record, edit_record, favicon])
}
