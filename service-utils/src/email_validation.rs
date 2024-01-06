#[derive(serde::Serialize)]
pub struct Validator<'a> {
    pub field: &'a str,
    pub tag: &'a str,
}

pub trait Validation {
    fn validate(&self) -> Result<(), tonic::Status>;
}

impl Validation for crate::proto::Email {
    fn validate(&self) -> Result<(), tonic::Status> {
        let mut validators = Vec::new();
        if self.email_to.is_empty() || self.email_to.len() < 3 || self.email_to.len() > 1000 {
            validators.push(Validator {
                field: "email_to",
                tag: "req",
            });
        }
        // check if to is valid email
        if !self.email_to.contains("@") {
            validators.push(Validator {
                field: "email_to",
                tag: "email",
            });
        }
        if self.email_from.is_empty() || self.email_from.len() < 3 || self.email_from.len() > 1000 {
            validators.push(Validator {
                field: "email_from",
                tag: "req",
            });
        }
        if (self.email_from_name.is_empty()
            || self.email_from_name.len() < 3
            || self.email_from_name.len() > 1000)
            && !self.email_from_name.contains("@")
        {
            validators.push(Validator {
                field: "email_from_name",
                tag: "req",
            });
        }
        if self.email_subject.is_empty()
            || self.email_subject.len() < 3
            || self.email_subject.len() > 1000
        {
            validators.push(Validator {
                field: "email_subject",
                tag: "req",
            });
        }

        if self.email_body.is_empty() || self.email_body.len() < 3 || self.email_body.len() > 1000 {
            validators.push(Validator {
                field: "email_body",
                tag: "req",
            });
        }

        if validators.is_empty() {
            Ok(())
        } else {
            let json = serde_json::to_string(&validators);
            match json {
                Ok(json) => Err(tonic::Status::invalid_argument(json)),
                Err(e) => {
                    tracing::error!("Failed to serialize validators: {:?}", e);
                    Err(tonic::Status::internal("Failed to serialize validators"))
                }
            }
        }
    }
}
