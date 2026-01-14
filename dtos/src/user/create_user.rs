use domain::models::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateUserDTO {
    #[validate(length(min = 1, max = 20))]
    pub first_name: String,

    #[validate(length(min = 1, max = 20))]
    pub last_name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 20))]
    pub password: String,
}

impl CreateUserDTO {
    pub fn to_user(&self) -> Result<User, Vec<String>> {
        match self.validate() {
            Ok(_) => Ok(User::new(
                format!("{} {}", self.first_name, self.last_name),
                self.email.clone(),
                self.password.clone(),
            )),

            Err(err) => {
                let mut mensagens = vec![];

                for (field, errors) in err.field_errors() {
                    if let Some(error) = errors.first() {
                        let msg = format!(
                            "{}: {}",
                            field,
                            error.message.clone().unwrap_or(error.code.clone())
                        );
                        mensagens.push(msg);
                    }
                }

                Err(mensagens)
            }
        }
    }
}
