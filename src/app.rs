use crate::components::list_videos::VideosPage;
use crate::routes::{switch, AppRoute};
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();
    html! {
        <Router history={history}>
            <main>
                <Switch<AppRoute> render={switch} />
            </main>
        </Router>
    }
}

#[function_component(WasmApp)]
pub fn wasm_app() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Switch<AppRoute> render={switch} />
            </main>
        </BrowserRouter>
    }
}

#[function_component(MainApp)]
fn main_app() -> Html {
    html! {
        <div>
            <RouteApp/ >
        </div>
    }
}

#[function_component(VideosApp)]
fn videos_app() -> Html {
    html! {

            <main>
                <VideosPage/ >
            </main>

    }
}

#[function_component(RouteApp)]
fn route_app() -> Html {
    html! {

            // <Header/ >
            <main>
                <Switch<AppRoute> render={switch} />
            </main>

    }
}
