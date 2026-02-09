use wasm_bindgen::prelude::*;

use crate::lua_engine::LAST_SCRIPT_ERROR;

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

/// Returns the last Lua script error message, or null if no error.
/// The error is cleared after being read.
#[wasm_bindgen]
pub fn get_last_script_error() -> Option<String> {
    LAST_SCRIPT_ERROR.with(|cell| cell.borrow_mut().take())
}
