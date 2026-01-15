mod note;
mod user;

pub use note::*;
pub use user::*;
use validator::Validate;

pub fn is_valid<T: Validate>(dto: T) -> Result<T, Vec<String>> {
    match dto.validate() {
        Ok(_) => Ok(dto),
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
