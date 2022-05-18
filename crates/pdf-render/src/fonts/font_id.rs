use std::sync::atomic::{AtomicUsize, Ordering};

/// Guaranteed Unique Id 
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct FontId(usize);

static CURRENT_ID: AtomicUsize = AtomicUsize::new(0);

impl FontId {
    pub(crate) fn new() -> Self {
        // NOTE: I know nothing about atomic ordering other than you use
        //  Sequentially Consistent when you're a dumb like me.
        //  
        //  The nomicon itself says this about ordering:
        //  "Trying to fully explain the model in this book is fairly hopeless. 
        //  It's defined in terms of madness-inducing causality graphs that 
        //  require a full book to properly understand in a practical way."
        Self(CURRENT_ID.fetch_add(1, Ordering::SeqCst))
    }
}
