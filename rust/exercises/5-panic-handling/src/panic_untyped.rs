use once_cell::sync::OnceCell;

use js_sys::{Function as JsFunction, JsString};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

thread_local! {
    static WASM_PANIC_HANDLER: OnceCell<WasmPanicHandler> = OnceCell::new();
}

struct WasmPanicHandler(JsFunction);

impl WasmPanicHandler {
    fn on_panic(&self, info: &std::panic::PanicInfo) {
        let panic_info: JsString = info.to_string().into();

        // `JsFunction::call1` yields a `Result`, but we ignore it here
        // since we're panicking anyway.
        let cx = JsValue::null();
        let _ = self.0.call1(&cx, &panic_info);
    }
}

#[wasm_bindgen]
pub fn register_panic_handler_untyped(on_panic: JsFunction) {
    WASM_PANIC_HANDLER.with(|handler| {
        let _ = handler.set(WasmPanicHandler(on_panic));
    });

    // The panic hook is invoked when a thread panics, but before the panic runtime is invoked.
    std::panic::set_hook(Box::new(|info| {
        WASM_PANIC_HANDLER.with(|handler| {
            if let Some(handler) = handler.get() {
                handler.on_panic(info);
            }
        });
    }));
}
