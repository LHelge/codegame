use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// Thread-local channels for receiving data from JavaScript (WASM)
// ---------------------------------------------------------------------------

std::thread_local! {
    pub static PENDING_LUA_CODE: std::cell::RefCell<Option<String>> =
        const { std::cell::RefCell::new(None) };
    pub static PENDING_RESET: std::cell::RefCell<bool> =
        const { std::cell::RefCell::new(false) };
}

#[wasm_bindgen]
pub fn set_agent_code(code: &str) {
    PENDING_LUA_CODE.with(|cell| {
        *cell.borrow_mut() = Some(code.to_string());
    });
}

#[wasm_bindgen]
pub fn request_reset() {
    PENDING_RESET.with(|cell| {
        *cell.borrow_mut() = true;
    });
}
