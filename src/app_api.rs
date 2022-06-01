use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };

            html! {
                <p onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[cfg(feature = "ssr")]
async fn fetched_videos() -> Vec<Video> {
    let resp = reqwest::get("https://yew.rs/tutorial/data.json")
        .await
        .unwrap();
    resp.json().await.unwrap()
}

#[function_component(VideosContainer)]
pub fn videos_container() -> HtmlResult {
    let videos =
        use_prepared_state!(async |_| -> Vec<Video> { fetched_videos().await }, ())?.unwrap();

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    Ok(html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    })
}

#[function_component(WasmApp)]
pub fn wasm_app() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <MainApp/>
            </main>
        </BrowserRouter>
    }
}

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
                <MainApp/>
            </main>
        </Router>
    }
}

#[function_component(MainApp)]
pub fn main_app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    // <Switch<Route> render={switch} />

    html! {
        <Suspense {fallback}>
            <VideosContainer />
        </Suspense>
    }
}
