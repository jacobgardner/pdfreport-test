
use printpdf::Mm as Ppmm;

use super::Mm;

impl From<Mm> for Ppmm {
    fn from(ours: Mm) -> Self {
      Ppmm(ours.0)
    }
}

