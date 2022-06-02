use simple_ssr::app::App;

fn main() {
    yew::Renderer::<App>::new().hydrate();
}
