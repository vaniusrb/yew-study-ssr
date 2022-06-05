use crate::routes::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html!(
        <div>
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div id="navbarBasicExample" class="navbar-menu">
                    <div class="navbar-start">
                        <a class="navbar-item">
                            <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                                { "Home" }
                            </Link<AppRoute>>
                        </a>
                        <a class="navbar-item">
                            <Link<AppRoute> to={AppRoute::Login} classes="nav-link">
                                { "Login" }
                            </Link<AppRoute>>
                        </a>
                        <a class="navbar-item">
                            <Link<AppRoute> to={AppRoute::Register} classes="nav-link">
                                { "Register" }
                            </Link<AppRoute>>
                        </a>
                    </div>
                </div>
            </nav>
        </div>
    )
}
