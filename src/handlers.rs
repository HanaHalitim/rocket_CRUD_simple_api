use crate::models::{ApiResponse, Record};
use rocket::{get, post, put, delete, State};
use crate::db::Db;  
use rocket::serde::json::Json;

// A simple health check endpoint.
#[get("/health")]
pub fn health() -> ApiResponse<&'static str> {
    ApiResponse { status: "ok".into(), data: "Healthy" }
}

// Other CRUD operations like get_records, add_record, etc., should follow similar patterns:
#[get("/records")]
pub fn get_records(db: &State<Db>) -> ApiResponse<Vec<Record>> {
    let db = db.records.lock().unwrap();
    ApiResponse { status: "ok".into(), data: db.clone() }
}

#[get("/favicon.ico")]
pub fn favicon() -> rocket::response::status::NoContent {
    rocket::response::status::NoContent
}

#[post("/records", format = "json", data = "<record>")]
pub fn add_record(record: Json<Record>, db: &State<Db>) -> ApiResponse<Record> {
    let mut db = db.records.lock().unwrap();
    let mut record = record.into_inner();
    if db.iter().any(|r| r.id == record.id) {
        record.id = db.len() + 1;  // Ensure unique ID
    }
    db.push(record.clone());
    ApiResponse { status: "ok".into(), data: record }
}

#[put("/records/<id>", format = "json", data = "<new_record>")]
pub fn edit_record(id: usize, new_record: Json<Record>, db: &State<Db>) -> Option<ApiResponse<Record>> {
    let mut db = db.records.lock().unwrap();
    if let Some(pos) = db.iter().position(|r| r.id == id) {
        db[pos] = new_record.into_inner();
        Some(ApiResponse { status: "ok".into(), data: db[pos].clone() })
    } else {
        None
    }
}

#[delete("/records/<id>")]
pub fn delete_record(id: usize, db: &State<Db>) -> ApiResponse<&'static str> {
    let mut db = db.records.lock().unwrap();
    if db.iter().any(|r| r.id == id) {
        db.retain(|r| r.id != id);
        ApiResponse { status: "ok".into(), data: "Record deleted" }
    } else {
        ApiResponse { status: "error".into(), data: "Record not found" }
    }
}
