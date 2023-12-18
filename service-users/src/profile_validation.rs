use tonic::Status;

use crate::proto::Profile;

#[derive(serde::Serialize)]
pub struct Validator<'a> {
    pub field: &'a str,
    pub tag: &'a str,
}

pub trait Validation {
    fn validate(&self) -> Result<(), Status>;
}

impl Validation for Profile {
    fn validate(&self) -> Result<(), Status> {
        let mut validators = Vec::new();

        // name is required and max length is 1000
        if self.name.is_empty() {
            validators.push(Validator {
                field: "name",
                tag: "required",
            });
        } else if self.name.len() > 1000 {
            validators.push(Validator {
                field: "name",
                tag: "max",
            });
        }
        // about is required and max length is 1000
        if self.about.is_empty() {
            validators.push(Validator {
                field: "about",
                tag: "required",
            });
        } else if self.about.len() > 1000 {
            validators.push(Validator {
                field: "about",
                tag: "max",
            });
        }

        if validators.is_empty() {
            Ok(())
        } else {
            let json = serde_json::to_string(&validators);
            match json {
                Ok(json) => Err(Status::invalid_argument(json)),
                Err(e) => {
                    tracing::error!("Failed to serialize validators: {:?}", e);
                    Err(Status::internal("Failed to serialize validators"))
                }
            }
        }
    }
}
