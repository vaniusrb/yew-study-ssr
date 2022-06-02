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

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
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

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Switch<AppRoute> render={switch} />
            </main>
        </BrowserRouter>
    }
}
