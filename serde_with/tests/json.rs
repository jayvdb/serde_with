//! Test Cases

mod utils;

use crate::utils::is_equal;
use expect_test::expect;
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::{json::JsonString, serde_as, DisplayFromStr};
use std::collections::BTreeMap;

#[test]
fn test_jsonstring() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde_as(as = "JsonString")]
        value: Nested,
    }

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Nested {
        #[serde_as(as = "DisplayFromStr")]
        value: u32,
    }

    is_equal(
        Struct {
            value: Nested { value: 444 },
        },
        expect![[r#"
            {
              "value": "{\"value\":\"444\"}"
            }"#]],
    );
}

#[test]
fn test_jsonstring_base64_encoded() {
    #[serde_with::serde_as]
    #[derive(Debug, Deserialize, PartialEq)]
    enum Wrapper {
        Base64Encoded(#[serde_as(deserialize_as = "Base64<JsonString>")] Struct),
    }

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde_as(as = "JsonString")]
        value: Nested,
    }

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Nested {
        #[serde_as(as = "DisplayFromStr")]
        value: u32,
    }

    /*
    is_equal(
        Wrapper::Base64Encoded(Struct {
            value: Nested { value: 444 },
        }),
        expect![[r#"
            {
              "value": "{\"value\":\"444\"}"
            }"#]],
    );
    */
}

#[test]
fn test_jsonstring_nested() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        #[serde_as(as = "JsonString<Vec<(JsonString, _)>>")]
        value: BTreeMap<[u8; 2], u32>,
    }

    is_equal(
        Struct {
            value: BTreeMap::from([([1, 2], 3), ([4, 5], 6)]),
        },
        expect![[r#"
            {
              "value": "[[\"[1,2]\",3],[\"[4,5]\",6]]"
            }"#]],
    );
}
