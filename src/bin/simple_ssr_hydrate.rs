use simple_ssr::app_api::AppApi;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<AppApi>::new().hydrate();
}
