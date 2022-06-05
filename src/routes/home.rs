use yew::prelude::*;

use crate::components::list_videos::VideosPage;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <VideosPage/ >
    }
}
