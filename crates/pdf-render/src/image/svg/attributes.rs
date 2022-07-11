use crate::error::{DocumentGenerationError, UserInputError};

pub enum DominantBaseline {
    Auto,
    Central,
    Middle,
    Hanging,
}

impl TryFrom<&str> for DominantBaseline {
    type Error = DocumentGenerationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let baseline = match value.to_lowercase().as_ref() {
            "auto" => Self::Auto,
            "central" => Self::Central,
            "middle" => Self::Middle,
            "hanging" => Self::Hanging,
            baseline => return Err(UserInputError::SvgParseError {
                message: format!("Dominant Baseline Value of {baseline} is not valid or yet supported.")
            }.into()),
        };

        Ok(baseline)
    }
}
