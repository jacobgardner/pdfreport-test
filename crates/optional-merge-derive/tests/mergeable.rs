use optional_merge_derive::mergeable_fn;
use serde::Deserialize;
use ts_rs::TS;

mergeable_fn! {
    source => {
        struct BorderStyle {
            pub width: String,
            pub height: String,
            pub debug: bool,
        }
    },
    mergeable => {
        #[derive(TS, Clone, Debug, PartialEq, Deserialize)]
        #[ts(export, rename_all = "camelCase")]
        #[serde(rename_all = "camelCase", deny_unknown_fields, skip_serializing_if = "Option::is_none")]
        pub struct MergeableBorderStyle;
    },
    unmergeable => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct BorderStyle;
    },
}


mergeable_fn! {
    source => {
        struct Style {
            #[mergeable(nested)]
            pub border: BorderStyle,
            pub width: String,
            pub height: String,
            pub debug: bool,
        }
    },
    mergeable => {
        #[derive(TS, Clone, Debug, PartialEq, Deserialize)]
        #[ts(export, rename_all = "camelCase")]
        #[serde(rename_all = "camelCase", deny_unknown_fields, skip_serializing_if = "Option::is_none")]
        pub struct MergeableStyle;
    },
    unmergeable => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct Style;
    },
}
