use std::{cell::RefCell, collections::HashMap, rc::Rc};

use printpdf::IndirectFontRef;

use crate::fonts::FontId;

pub(super) struct FontLookup(RefCell<HashMap<FontId, Rc<IndirectFontRef>>>);

impl FontLookup {
    pub(super) fn new() -> Self {
        Self(RefCell::new(HashMap::new()))
    }

    pub(super) fn get(&self, font_id: FontId) -> Option<Rc<IndirectFontRef>> {
        self.0.borrow().get(&font_id).cloned()
    }

    pub(super) fn insert(&self, font_id: FontId, font_ref: IndirectFontRef) {
        self.0.borrow_mut().insert(font_id, Rc::new(font_ref));
    }

    pub(super) fn insert_and_get(&self, font_id: FontId, font_ref: IndirectFontRef) -> Rc<IndirectFontRef> {
        self.insert(font_id, font_ref);

        self.get(font_id)
            .expect("We just inserted it. It has to exist")
    }
}
