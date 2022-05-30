use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
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

async fn fetched_videos() -> Vec<Video> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let resp = reqwest::Client::new()
            .get("https://yew.rs/tutorial/data.json")
            .fetch_mode_no_cors()
            .send()
            .await
            .unwrap();
        resp.json().await.unwrap()
    }

    #[cfg(target_arch = "wasm32")]
    {
        reqwasm::http::Request::get("/tutorial/data.json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    // vec![
    //     Video {
    //         id: 0,
    //         title: "aaa".into(),
    //         speaker: "bbb".into(),
    //         url: "ccc".into(),
    //     },
    //     Video {
    //         id: 1,
    //         title: "aaa".into(),
    //         speaker: "bbb".into(),
    //         url: "ccc".into(),
    //     },
    // ]
}

#[function_component(AppApi)]
pub fn app_api() -> Html {
    let videos = use_state(Vec::new);

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

    #[cfg(target_arch = "wasm32")]
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos = fetched_videos().await;
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                tokio::task::spawn_local(async move {
                    let fetched_videos = fetched_videos().await;
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    // ...

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

// fn main() {
//     yew::Renderer::<AppApi>::new().render();
// }
