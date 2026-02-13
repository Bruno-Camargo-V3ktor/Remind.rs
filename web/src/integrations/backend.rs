use domain::models::Property;
use dtos::{CreatePropertyDTO, CreateUserDTO, InfoUserDTO, LoginUserDTO, UpdateUserDTO};
use http::error::ErrorInfos;

const BASE_URL: &str = "http://localhost:3000/api";

#[derive(Clone, Debug)]
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

    pub async fn login_user(&self, email: String, password: String) -> Result<Token, ErrorInfos> {
        let dto = LoginUserDTO { email, password };

        let response = self
            .client
            .post(format!("{BASE_URL}/auth/login"))
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

    pub async fn auth_user(&self, token: Token) -> Result<InfoUserDTO, ErrorInfos> {
        let response = self
            .client
            .get(format!("{BASE_URL}/auth/user"))
            .header("Authorization", token.0)
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
            let value = http_response.data.as_ref().unwrap().clone();
            let infos: InfoUserDTO = serde_json::from_value(value).unwrap();

            Ok(infos)
        } else {
            Err(http_response.error.unwrap())
        }
    }

    pub async fn delete_user(&self, token: Token) -> Result<(), ErrorInfos> {
        let response = self
            .client
            .delete(format!("{BASE_URL}/users/"))
            .header("Authorization", token.0)
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
            Ok(())
        } else {
            Err(http_response.error.unwrap())
        }
    }

    pub async fn update_user(
        &self,
        token: Token,
        update: UpdateUserDTO,
    ) -> Result<InfoUserDTO, ErrorInfos> {
        let response = self
            .client
            .put(format!("{BASE_URL}/users/"))
            .header("Authorization", token.0)
            .json(&update)
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
            let value = http_response.data.as_ref().unwrap().clone();
            let infos: InfoUserDTO = serde_json::from_value(value).unwrap();

            Ok(infos)
        } else {
            Err(http_response.error.unwrap())
        }
    }

    pub async fn request_new_password(&self, email: String, url: String) -> Result<(), ErrorInfos> {
        let response = self
            .client
            .post(format!(
                "{BASE_URL}/auth/reset-password?email={email}&url={url}"
            ))
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
            Ok(())
        } else {
            Err(http_response.error.unwrap())
        }
    }

    pub async fn list_propertys(&self, token: Token) -> Result<Vec<Property>, ErrorInfos> {
        let response = self
            .client
            .get(format!("{BASE_URL}/propertys/"))
            .header("Authorization", token.0)
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
            let value = http_response.data.as_ref().unwrap().clone();
            let propertys: Vec<Property> = serde_json::from_value(value).unwrap();

            Ok(propertys)
        } else {
            Err(http_response.error.unwrap())
        }
    }

    pub async fn create_property(
        &self,
        token: Token,
        created: CreatePropertyDTO,
    ) -> Result<Property, ErrorInfos> {
        let response = self
            .client
            .post(format!("{BASE_URL}/propertys/"))
            .header("Authorization", token.0)
            .json(&created)
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
            let value = http_response.data.as_ref().unwrap().clone();
            let property: Property = serde_json::from_value(value).unwrap();

            Ok(property)
        } else {
            Err(http_response.error.unwrap())
        }
    }
}
