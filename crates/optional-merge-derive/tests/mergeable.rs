use optional_merge_derive::mergeable;
use serde::Deserialize;

#[mergeable]
#[derive(Debug, Clone)]
struct NestedStruct {}

#[mergeable(rename = "poop")]
#[derive(Debug, Clone)]
struct TestStruct {
    bool_test: bool,
    #[mergeable(nested)]
    nested_struct: NestedStruct,
}


#[test]
fn test_unmerged() {
    let original = TestStruct {
        bool_test: true,
        nested_struct: NestedStruct {},
    };

    let merged = MergeableTestStruct::default();
}
