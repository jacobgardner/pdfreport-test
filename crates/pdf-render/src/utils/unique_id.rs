macro_rules! create_id {
    ($id_type:ident) => {
        use std::sync::atomic::{AtomicUsize, Ordering};

        /// Guaranteed Unique Id
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct $id_type(usize);

        static CURRENT_ID: AtomicUsize = AtomicUsize::new(0);

        impl $id_type {
            pub fn new() -> Self {
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
        
        impl Default for $id_type {
          fn default() -> Self {
            Self::new()
          }
        }
    };
}

pub(crate) use create_id;
