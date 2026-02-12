use dtos::CreateUserDTO;
use http::error::ErrorInfos;

const BASE_URL: &str = "http://localhost:3000/api";

pub struct Token(pub String);

#[derive(Default, Debug)]
pub struct Backend {
    pub client: reqwest::Client,
}

impl Backend {
    pub async fn register_user(
        &self,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    ) -> Result<Token, ErrorInfos> {
        let dto = CreateUserDTO {
            first_name,
            last_name,
            email,
            password,
        };

        let response = self
            .client
            .post(format!("{BASE_URL}/auth/register"))
            .json(&dto)
            .send()
            .await
            .map_err(|e| {
                ErrorInfos::new(
                    "REQWEST_ERROR".into(),
                    "Failed to send request".into(),
                    e.to_string(),
                )
            })?;

        let http_response: http::Response = response.json().await.map_err(|e| {
            ErrorInfos::new(
                "SERIALIZATION_ERROR".into(),
                "Failed to parse response".into(),
                e.to_string(),
            )
        })?;

        if http_response.success {
            let token_str = http_response
                .data
                .as_ref()
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            Ok(Token(token_str))
        } else {
            Err(http_response.error.unwrap())
        }
    }
}
