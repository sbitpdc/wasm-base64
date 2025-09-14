use wasm_bindgen::prelude::*;
use base64::{engine::general_purpose, Engine as _};
use js_sys::{Uint8Array, ArrayBuffer};

// 使用 #[wasm_bindgen] 宏来导出函数到 JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}


// 将 ArrayBuffer 转换为 base64 字符串
#[wasm_bindgen]
pub fn array_buffer_to_base64(buffer: &[u8]) -> String {
    general_purpose::STANDARD.encode(buffer)
}

// 将 base64 字符串转换为 ArrayBuffer
#[wasm_bindgen]
pub fn base64_to_array_buffer(base64_str: &str) -> Result<Vec<u8>, JsValue> {
    let bytes = general_purpose::STANDARD
        .decode(base64_str)
        .map_err(|e| JsValue::from_str(&format!("Base64 decode error: {}", e)))?;
    Ok(bytes)
}

// 辅助函数：从 JS 的 Uint8Array 转换为 Rust 字节数组（用于接收 ArrayBuffer）
#[wasm_bindgen]
pub fn uint8_array_to_base64(uint8_array: &Uint8Array) -> String {
    let buffer = uint8_array.to_vec();
    array_buffer_to_base64(&buffer)
}

// 辅助函数：将 Rust 字节数组转换为 JS 的 Uint8Array（用于返回 ArrayBuffer）
#[wasm_bindgen]
pub fn base64_to_uint8_array(base64_str: &str) -> Result<Uint8Array, JsValue> {
    let bytes = base64_to_array_buffer(base64_str)?;
    Ok(Uint8Array::from(&bytes[..]))
}

#[wasm_bindgen]
pub fn array_buffer_to_base64_direct(js_array_buffer: &ArrayBuffer) -> String {
    // 1. 直接从 ArrayBuffer 创建 Uint8Array 视图（零拷贝）
    let uint8_array = Uint8Array::new(js_array_buffer);

    // 2. 获取数据长度
    let len = uint8_array.length() as usize;

    // 3. 将数据复制到 WASM 内存（必须步骤）
    let mut buffer = vec![0u8; len];
    uint8_array.copy_to(&mut buffer);

    // 4. 进行 Base64 编码
    general_purpose::STANDARD.encode(&buffer)
}