use tonic::Status;

use crate::proto::Note;

#[derive(serde::Serialize)]
pub struct Validator<'a> {
    pub field: &'a str,
    pub tag: &'a str,
}

pub trait Validation {
    fn validate(&self) -> Result<(), Status>;
}

impl Validation for Note {
    fn validate(&self) -> Result<(), Status> {
        let mut validators = Vec::new();
        if self.title.is_empty() {
            validators.push(Validator {
                field: "title",
                tag: "required",
            });
        }
        if self.title.len() > 1000 {
            validators.push(Validator {
                field: "title",
                tag: "max",
            });
        }
        if self.content.is_empty() {
            validators.push(Validator {
                field: "content",
                tag: "required",
            });
        }
        if self.content.len() > 1000 {
            validators.push(Validator {
                field: "content",
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
