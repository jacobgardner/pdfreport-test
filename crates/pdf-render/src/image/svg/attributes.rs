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
            baseline => {
                return Err(UserInputError::SvgParseError {
                    message: format!(
                        "Dominant Baseline Value of {baseline} is not valid or yet supported."
                    ),
                }
                .into())
            }
        };

        Ok(baseline)
    }
}

pub trait LowerCaseAttribute {
    fn lc_has_attribute(&self, attribute: &str) -> bool;
    fn lc_attribute(&self, attribute: &str) -> Option<&str>;
}

impl<'a, 'b> LowerCaseAttribute for roxmltree::Node<'a, 'b> {
    fn lc_has_attribute(&self, attribute: &str) -> bool {
        debug_assert!(
            attribute == attribute.to_lowercase(),
            "Must provide attribute name in lowercase"
        );
        todo!()
    }

    fn lc_attribute(&self, attribute: &str) -> Option<&str> {
        debug_assert!(
            attribute == attribute.to_lowercase(),
            "Must provide attribute name in lowercase"
        );

        self.attributes()
            .iter()
            .find(|&attr| attr.name().to_lowercase() == attribute)
            .map(|attr| attr.value())
    }
}
