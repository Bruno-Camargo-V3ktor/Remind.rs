use super::{Cred, From, SendEmailService};
use crate::ServiceBuilder;

pub struct SendEmailBuilder {
    from: Option<From>,
    cred: Option<Cred>,
    smtp: Option<String>,
}

impl SendEmailBuilder {
    pub fn from(mut self, name: String, email: String) -> Self {
        self.from = Some(From { name, email });
        self
    }

    pub fn cred(mut self, key: String) -> Self {
        self.cred = Some(Cred { app_key: key });
        self
    }

    pub fn smtp(mut self, smtp: String) -> Self {
        self.smtp = Some(smtp);
        self
    }
}

impl ServiceBuilder for SendEmailBuilder {
    type S = SendEmailService;

    fn new() -> Self {
        Self {
            cred: None,
            from: None,
            smtp: None,
        }
    }

    fn build(self) -> Self::S {
        Self::S {
            from: self.from.expect(""),
            cred: self.cred.expect(""),
            smtp: self.smtp.expect(""),
        }
    }
}
