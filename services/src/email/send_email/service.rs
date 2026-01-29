use lettre::{
    Message, SmtpTransport, Transport, message::Mailbox,
    transport::smtp::authentication::Credentials,
};

use super::{Cred, From, SendEmailError, To};
use crate::Service;

pub struct SendEmailService {
    pub from: From,
    pub cred: Cred,
    pub smtp: String,
}

#[async_trait::async_trait]
impl Service for SendEmailService {
    type Args = (To, String, String, String);
    type Out = ();

    async fn run(&self, args: Self::Args) -> Result<Self::Out, SendEmailError> {
        let (to, subject, body_text, body_html) = args;

        let email = Message::builder()
            .from(Mailbox::new(
                Some(self.from.name.clone()),
                self.from.email.parse().unwrap(),
            ))
            .to(Mailbox::new(Some(to.name), to.email.parse().unwrap()))
            .subject(subject)
            .multipart(
                lettre::message::MultiPart::alternative()
                    .singlepart(lettre::message::SinglePart::plain(String::from(&body_text)))
                    .singlepart(lettre::message::SinglePart::html(String::from(&body_html))),
            )
            .unwrap();

        let creds = Credentials::new(self.from.email.clone(), self.cred.app_key.clone());

        let mailer = SmtpTransport::relay(&self.smtp)
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => Err(SendEmailError::SendEmailFail(e.to_string())),
        }
    }
}
