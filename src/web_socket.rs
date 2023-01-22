
use std::io::{Cursor, Seek, SeekFrom, Write};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use crate::binary_readers::read_hash;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Taken from https://rustwasm.github.io/wasm-bindgen/examples/websockets.html
#[wasm_bindgen]
pub fn start_websocket(uri: &str) -> Result<(), JsValue> {
    // Connect to an echo server
    let ws = WebSocket::new(uri)?;
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            console_log!("message event, received arraybuffer: {:?}", abuf);
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            let _array = array.to_vec();
            // here you can for example use Serde Deserialize decode the message
            // for demo purposes we switch back to Blob-type and send off another binary message
            let size_buf : [u8; 4]= _array[0..4].try_into().unwrap();
            let karabo_len = u32::from_le_bytes(size_buf);
            let mut c = Cursor::new(Vec::new());
            console_log!("Hash of {} bytes was {}", karabo_len, len);
            // Write into the "file" and seek to the beginning
            c.write_all(&_array).unwrap();
            c.seek(SeekFrom::Start(4)).unwrap();
            let hash = read_hash(&mut c).unwrap();
            console_log!("Hash keys: {:?}", hash.keys());
            // here you can for example use the received image/png data
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            console_log!("message event, received blob: {:?}", blob);
            // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
            let fr = web_sys::FileReader::new().unwrap();
            let fr_c = fr.clone();
            // create onLoadEnd callback
            let onloadend_cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                let _array = array.to_vec();
                let size_buf : [u8; 4]= _array[0..4].try_into().unwrap();
                let karabo_len = u32::from_le_bytes(size_buf);
                let mut c = Cursor::new(Vec::new());
                console_log!("Hash of {}bytes", karabo_len);
                // Write into the "file" and seek to the beginning
                c.write_all(&_array).unwrap();
                c.seek(SeekFrom::Start(4)).unwrap();
                let hash = read_hash(&mut c).unwrap();
                console_log!("Hash keys: {:?}", hash.keys());
                // here you can for example use the received image/png data
            });
            fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            fr.read_as_array_buffer(&blob).expect("blob not readable");
            onloadend_cb.forget();
        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            console_log!("message event, received Text: {:?}", txt);
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    });
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened to {:?}", cloned_ws.url());

        match cloned_ws.send_with_str("{\"host\":\"exflqr18337\",\"port\":44444}") {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
        // send off binary message
        // match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
        //     Ok(_) => console_log!("binary message successfully sent"),
        //     Err(err) => console_log!("error sending message: {:?}", err),
        // }
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}