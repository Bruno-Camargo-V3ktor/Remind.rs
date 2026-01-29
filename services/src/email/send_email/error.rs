use crate::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[serde(untagged)]
pub enum SendEmailError {
    #[error("could not send email: {0}")]
    SendEmailFail(String),
}

impl ServiceError for SendEmailError {
    fn code(&self) -> String {
        match self {
            Self::SendEmailFail(_) => "SEND_EMAIL_FAIL".into(),
        }
    }

    fn content(&self) -> &impl serde::Serialize {
        self
    }

    fn description(&self) -> String {
        self.to_string().trim().to_string()
    }
}
