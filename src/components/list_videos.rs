use serde::{Deserialize, Serialize};
use yew::prelude::*;

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
            <h3>{ &video.title }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[cfg(feature = "ssr")]
async fn fetched_videos() -> Vec<Video> {
    let resp = match reqwest::get("https://yew.rs/tutorial/data.json").await {
        Ok(r) => r,
        Err(_) => {
            return vec![Video {
                id: 1,
                title: "xxx".into(),
                speaker: "xxx".into(),
                url: "xxx".into(),
            }]
        }
    };
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

#[function_component(VideosPage)]
pub fn videos_page() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    // <Switch<Route> render={switch} />

    html! {
        <div>
            <Suspense {fallback}>
                <VideosContainer />
            </Suspense>
        </div>
    }
}
