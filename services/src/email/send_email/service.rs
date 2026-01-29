use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
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
    type Args = (To, String, String);
    type Out = ();

    async fn run(&self, args: Self::Args) -> Result<Self::Out, SendEmailError> {
        let (to, subject, body) = args;

        let email = Message::builder()
            .from(Mailbox::new(
                Some(self.from.name.clone()),
                self.from.email.parse().unwrap(),
            ))
            .to(Mailbox::new(Some(to.name), to.email.parse().unwrap()))
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)
            .unwrap();

        let creds = Credentials::new(self.cred.username.clone(), self.cred.password.clone());

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
