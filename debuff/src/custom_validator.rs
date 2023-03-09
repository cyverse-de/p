use uuid::Uuid;
use validator::ValidationError;

pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
    match Uuid::parse_str(uuid) {
        Ok(_) => Ok(()),
        Err(e) => Err(ValidationError::new("cannot parse uuid")),
    }
}
