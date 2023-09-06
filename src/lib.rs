mod utils;

use wasm_bindgen::prelude::*;
use ws_stream_wasm::{*};
use futures_util::io::AsyncWriteExt;

extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub async fn greet() {
    console_error_panic_hook::set_once();

    let ws_server_url = "ws://localhost:12345";
    let (mut ws_meta, mut ws_stream) = WsMeta::connect(ws_server_url, None ).await
        .expect_throw( "assume the client ws connection succeeds" );
    let mut io_stream = ws_stream.into_io();
    let message         = b"Hello from browser".to_vec();
	io_stream
        .write(&message).await
        .expect_throw( "Failed to write to websocket" );
    console_log!("Successfully sent message to server");
}
