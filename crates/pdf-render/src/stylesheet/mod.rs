use std::collections::HashMap;

use serde::Deserialize;

mod style;

use style::MergeableStyle;

#[derive(Deserialize, Debug)]
pub struct Stylesheet(HashMap<String, MergeableStyle>);