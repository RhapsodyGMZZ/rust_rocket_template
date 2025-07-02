use std::{collections::HashMap, io::Cursor};

use rocket::{
    Response,
    http::{ContentType, Cookie, Header, Status},
    response::{Builder, Responder},
    serde,
};

/// Custom HTTP Response object to improve our dev experience / readablity of the code  
#[derive(Debug)]
pub struct CustomResponse {
    pub headers: HashMap<&'static str, &'static str>,
    pub status_code: Status,
    pub content_type: ContentType,
    pub body: serde::json::Value,
    pub cookie: Option<Cookie<'static>>,
}

impl Default for CustomResponse {
    fn default() -> Self {
        Self::new(
            None,
            Status::Ok,
            ContentType::JSON,
            serde::json::json!({}),
            None,
        )
    }
}

impl CustomResponse {
    /// Initialising the CustomResponse object
    pub fn new(
        headers_map: Option<HashMap<&'static str, &'static str>>,
        status_code: Status,
        content_type: ContentType,
        body: serde::json::Value,
        cookie: Option<Cookie<'static>>,
    ) -> Self {
        if let Some(headers) = headers_map {
            Self {
                headers,
                status_code,
                content_type,
                body,
                cookie,
            }
        } else {
            Self {
                headers: HashMap::new(),
                status_code,
                content_type,
                body,
                cookie,
            }
        }
    }

    pub fn created() -> CustomResponse {
        CustomResponse::new(
            None,
            Status::Created,
            ContentType::JSON,
            serde::json::json!({}),
            None,
        )
    }

    pub fn internal_error() -> CustomResponse {
        CustomResponse::new(
            None,
            Status::InternalServerError,
            ContentType::JSON,
            serde::json::json!({}),
            None,
        )
    }

    pub fn bad_request() -> CustomResponse {
        CustomResponse::new(
            None,
            Status::BadRequest,
            ContentType::JSON,
            serde::json::json!({}),
            None,
        )
    }

    pub fn forbidden() -> CustomResponse {
        CustomResponse::new(
            None,
            Status::Forbidden,
            ContentType::JSON,
            serde::json::json!({}),
            None,
        )
    }

    pub fn new_internal_error(
        content_type: ContentType,
        body: serde::json::Value,
    ) -> CustomResponse {
        CustomResponse::new(None, Status::InternalServerError, content_type, body, None)
    }

    pub fn new_conflict(content_type: ContentType, body: serde::json::Value) -> CustomResponse {
        CustomResponse::new(None, Status::Conflict, content_type, body, None)
    }
}

/// In Rocket's logic, an object can be returned as a HTTP Response only if it has
/// this `Responder` trait implemented
impl<'r, 'a> Responder<'r, 'static> for CustomResponse {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        // Building the response
        let mut build: Builder = Response::build();
        //For each entry in the Headers HashMap, it'll add it to the HTTP response
        self.headers.iter().for_each(|(&key, &val)| {
            build.header(Header::new(key, val));
        });
        if self
            .cookie
            .clone()
            .is_some_and(|c| request.cookies().get_private(c.name()).is_none()) // Checking if it's a nex cookie to set or just an existing cookie
        {
            request
                .cookies()
                .add_private(self.cookie.expect("No cookie to set")); // adding encrypted cookie to the front, only the server has access to the decrypted value
        }
        build
            .header(self.content_type) // adding "Content-Type" Header
            .sized_body(
                self.body.to_string().len(),
                Cursor::new(self.body.to_string()),
            )
            .status(self.status_code)
            .ok()
    }
}
