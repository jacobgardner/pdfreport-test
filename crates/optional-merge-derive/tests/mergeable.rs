// use optional_merge_derive::mergeable;

// #[mergeable]
// #[derive(Debug, Clone)]
// struct NestedStruct {}

// #[mergeable]
// #[derive(Debug, Clone)]
// struct TestStruct {
//     bool_test: bool,
//     #[mergeable(nested)]
//     nested_struct: NestedStruct,
// }

// #[test]
// fn test_unmerged() {
//     let _original = TestStruct::Unmergeable {
//         bool_test: true,
//         nested_struct: NestedStruct::Unmergeable {},
//     };

//     let _merged = TestStruct::Mergeable::default();
// }
