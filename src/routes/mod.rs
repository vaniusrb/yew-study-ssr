pub mod home;
pub mod login;
pub mod register;
use self::{home::Home, login::Login, register::Register};
use yew::prelude::*;
use yew_router::prelude::*;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Login => html! { <Login /> },
        AppRoute::Register => html! { <Register /> },
        AppRoute::Home => html! { <Home /> },
    }
}
