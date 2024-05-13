use serde::{Serialize, Deserialize};

// Define the Record struct which represents a single record in the application.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: usize,
    pub name: String,
    pub value: String,
}

// Define the ApiResponse struct which is a generic structure for API responses.
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

// Implement the Responder trait for ApiResponse<T> so that Rocket can convert it into an HTTP response.
use rocket::response::{self, Responder, Response};
use rocket::{Request, http::Status};

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResponse<T> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // Serialize ApiResponse using Serde to JSON
        let json_string = serde_json::to_string(&self)
            .map_err(|_| Status::InternalServerError)?;
        
        // Create a new Response and set its body to the serialized JSON
        Response::build()
            .header(rocket::http::ContentType::JSON)
            .sized_body(json_string.len(), std::io::Cursor::new(json_string))
            .ok()
    }
}
