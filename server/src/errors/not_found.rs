use crate::app::App;
use actix_web::{HttpRequest, web};

pub async fn not_found_header(app: web::Data<App>, req: HttpRequest) -> http::Response {
    let path = req.uri().path();
    let method = req.method().as_str();
    let code = "NOT_FOUND_ROUTE".to_string();
    let status_code = app.error_code(code.clone());

    http::Response::error(
        status_code,
        code,
        format!("endpoint not found: {} | {}", method, path),
        (),
    )
}
