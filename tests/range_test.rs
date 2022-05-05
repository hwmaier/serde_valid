use serde_json::json;
use serde_valid::Validate;

#[test]
fn range_integer_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 2000)]
        val: i32,
    }

    let s = TestStruct { val: 1234 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_float_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0.0)]
        #[validate(maximum = 2000.0)]
        val: f32,
    }

    let s = TestStruct { val: 1234.5678 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 0)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_minimum_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_minimum_is_err_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_err());
}

#[test]
fn range_exclusive_minimum_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 1 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_minimum_is_err_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_err());
}

#[test]
fn range_maximum_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 10 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_maximum_is_err_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 11 };
    assert!(s.validate().is_err());
}

#[test]
fn range_exclusive_maximum_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_maximum = 10)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 9 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_maximum_is_err_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_maximum = 10)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 10 };
    assert!(s.validate().is_err());
}

#[test]
fn range_vec_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 20)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![12, 16] };
    assert!(s.validate().is_ok());
}

#[test]
fn range_nested_vec_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 20)]
        val: Vec<Vec<i32>>,
    }

    let s = TestStruct {
        val: vec![vec![4, 8], vec![12, 16]],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn range_option_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: Option<i32>,
    }

    let s = TestStruct { val: Some(5) };
    assert!(s.validate().is_ok());
}

#[test]
fn range_nested_option_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: Option<Option<i32>>,
    }

    let s = TestStruct { val: Some(Some(5)) };
    assert!(s.validate().is_ok());
}

#[test]
fn range_vec_optional_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: Vec<Option<i32>>,
    }

    let s = TestStruct {
        val: vec![Some(4), Some(8), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn range_array_optional_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: [Option<i32>; 3],
    }

    let s = TestStruct {
        val: [Some(4), Some(8), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn range_inclusive_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "the number must be `>= 1`."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn range_exclusive_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 1)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "the number must be `> 1`."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn range_custom_err_message_fn_test() {
    fn custom_min_error_message(_params: &serde_valid::MinimumParams) -> String {
        "this is min custom message.".to_string()
    }

    fn custom_max_error_message(_params: &serde_valid::MaximumParams) -> String {
        "this is max custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 5, message_fn(custom_min_error_message))]
        #[validate(maximum = 3, message_fn(custom_max_error_message))]
        val: i32,
    }

    let s = TestStruct { val: 4 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is min custom message.",
                "this is max custom message."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn range_custom_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 5, message = "this is min custom message.")]
        #[validate(maximum = 3, message = "this is max custom message.")]
        val: i32,
    }

    let s = TestStruct { val: 4 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is min custom message.",
                "this is max custom message."
            ]
        }))
        .unwrap()
    );
}
