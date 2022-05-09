use serde::Deserialize;
use serde_json::json;
use serde_valid::json::FromJson;
use serde_valid::Validate;

#[test]
fn from_json_value_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 2000)]
        val: i32,
    }

    let s = TestStruct::from_json_value(json!({ "val": 1234 }));
    assert!(s.is_ok())
}

#[test]
fn from_json_str_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 2000)]
        val: i32,
    }
    let s = TestStruct::from_json_str(&serde_json::to_string(&json!({ "val": 1234 })).unwrap());

    assert!(s.is_ok())
}

#[test]
fn from_json_slice_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 2000)]
        val: i32,
    }

    TestStruct::from_json_slice(b"{ \"val\": 1234 }").unwrap();
}

#[test]
fn deserialize_validation_err_to_string() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 1000)]
        val: i32,
    }

    let err = TestStruct::from_json_value(json!({ "val": 1234 })).unwrap_err();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&err.to_string()).unwrap(),
        json!({"val": ["the number must be `<= 1000`."]})
    );
}

#[test]
fn deserialize_validation_err_to_json_value() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 1000)]
        val: i32,
    }

    let err = TestStruct::from_json_value(json!({ "val": 1234 })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({"val": ["the number must be `<= 1000`."]})
    );
}
