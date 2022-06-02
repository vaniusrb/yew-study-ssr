use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { "home" },
    }
}
