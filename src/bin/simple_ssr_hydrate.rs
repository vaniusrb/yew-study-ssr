use simple_ssr::app_api::WasmApp;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<WasmApp>::new().hydrate();
}
