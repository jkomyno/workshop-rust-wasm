use js_sys::{Function as JsFunction, JsString};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

struct WasmPanicHandler(JsFunction);

impl WasmPanicHandler {
    fn on_panic(&self, info: &std::panic::PanicInfo) {
        let panic_info: JsString = info.to_string().into();

        // `JsFunction::call1` yields a `Result`, but we ignore it here
        // since we're panicking anyway.
        let this = JsValue::null();
        let _ = self.0.call1(&this, &panic_info);
    }
}

#[wasm_bindgen]
pub fn register_panic_handler_untyped(on_panic: JsFunction) {
    let handler = WasmPanicHandler(on_panic);

    // The panic hook is invoked when a thread panics, but before the panic runtime is invoked.
    std::panic::set_hook(Box::new(|info| {
        handler.on_panic(info);
    }));
}
