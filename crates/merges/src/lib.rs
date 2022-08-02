pub trait Merges: Sized + Clone {
    fn merge(&self, rhs: &Self) -> Self;

    fn merge_optional(&self, rhs: &Option<Self>) -> Option<Self> {
        if let Some(op) = rhs {
            Some(self.merge(op))
        } else {
            Some(self.clone())
        }
    }
}

pub fn nested_merge<T: Merges>(lhs: &Option<T>, rhs: &Option<T>) -> Option<T> {
    if let Some(mergeable_lhs) = lhs {
        if let Some(rhs) = rhs {
            Some(mergeable_lhs.merge(rhs))
        } else {
            lhs.clone()
        }
    } else {
        rhs.clone()
    }
}

pub fn primitive_merge<T: Clone>(lhs: &Option<T>, rhs: &Option<T>) -> Option<T> {
    rhs.as_ref().or(lhs.as_ref()).cloned()
}

pub trait HasMergeableVariant {
    type MergeableType;
}

pub trait HasUnmergeableVariant {
    type UnmergeableType;
}
