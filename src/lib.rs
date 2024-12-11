use wasm_bindgen::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use js_sys::{Promise, Function, Array};
use std::str;

#[wasm_bindgen(js_name = "hashSync")]
pub fn hash_password_sync(data: &str, salt_or_rounds: Option<u32>) -> Result<String, String> {
    let salt_or_rounds = salt_or_rounds.unwrap_or(DEFAULT_COST);

    hash(data, salt_or_rounds).map_err(|_| "Error hashing password".to_string())
}

#[wasm_bindgen(js_name = "compareSync")]
pub fn compare_password_sync(data: &str, encrypted: &str) -> bool {
    verify(data, encrypted).unwrap_or_else(|_| false)
}

#[wasm_bindgen(js_name = "hash")]
pub fn hash_password(data: String, salt_or_rounds: Option<u32>) -> Promise {
    let salt_or_rounds = salt_or_rounds.unwrap_or(DEFAULT_COST);

    // Tạo Promise
    Promise::new(&mut |resolve, reject| {
        match hash(&data, salt_or_rounds) {
            Ok(hashed) => {
                // Tạo một js_sys::Array chứa giá trị `hashed`
                let result = Array::new();
                result.push(&JsValue::from_str(&hashed));

                // Truyền mảng `result` vào `resolve` như một slice chứa `JsValue`
                Function::apply(&resolve, &JsValue::NULL, &result).unwrap();
            }
            Err(_) => {
                // Tạo một mảng lỗi
                let error = Array::new();
                error.push(&JsValue::from_str("Error hashing password"));

                // Truyền mảng `error` vào `reject` như một slice chứa `JsValue`
                Function::apply(&reject, &JsValue::NULL, &error).unwrap();
            }
        }
    })
}

#[wasm_bindgen(js_name = "compare")]
pub fn compare_password(data: String, encrypted: String) -> Promise {
    // Tạo Promise
    Promise::new(&mut |resolve, reject| {
        match verify(&data, &encrypted) {
            Ok(is_valid) => {
                // Trả về true hoặc false dưới dạng một mảng chứa một giá trị boolean
                let result = Array::new();
                result.push(&JsValue::from_bool(is_valid));

                // Resolve promise với kết quả
                Function::apply(&resolve, &JsValue::NULL, &result).unwrap();
            }
            Err(_) => {
                // Nếu có lỗi, trả về thông báo lỗi trong một mảng
                let error = Array::new();
                error.push(&JsValue::from_str("Error verifying password"));

                // Reject promise với lỗi
                Function::apply(&reject, &JsValue::NULL, &error).unwrap();
            }
        }
    })
}