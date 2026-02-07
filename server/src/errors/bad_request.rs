use actix_web::{Responder, error};

pub fn bad_req_header(err: error::JsonPayloadError, req: &actix_web::HttpRequest) -> error::Error {
    let code = "INVALID_JSON".to_string();
    let status_code = 400;

    let response = http::Response::error(status_code, code, err.to_string(), ()).respond_to(req);

    error::InternalError::from_response(err, response).into()
}
