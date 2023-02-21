use reqwest::Client;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub async fn stream_audio_file(url: &str) -> Result<(), JsValue> {
    // Load the audio file into a buffer
    let buffer = load_audio_file(url).await?;

    // Set up a Reqwest client
    let client = Client::new();

    // Send the buffer to the server using a POST request
    let response = client.post("http://example.com/audio")
        .body(buffer)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Reqwest error: {}", e)))?;

    // Check the response status
    if !response.status().is_success() {
        return Err(JsValue::from_str("Server returned non-200 status code"));
    }

    Ok(())
}

async fn load_audio_file(url: &str) -> Result<Vec<u8>, JsValue> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| JsValue::from_str(&format!("Reqwest error: {}", e)))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| JsValue::from_str(&format!("Reqwest error: {}", e)))?;

    Ok(bytes.to_vec())
}