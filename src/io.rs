//Io Functions
use crate::events::{Event, Events};
use anyhow::Result;
use tokio::task::JoinSet;

// Async download all given event urls
pub async fn download_all_events(event_urls: Vec<String>) -> Result<Events> {
    let mut tasks = JoinSet::new();

    for url in event_urls {
        tasks.spawn(download_event(url));
    }

    let mut events: Events = Events { events: vec![] };

    while let Some(res) = tasks.join_next().await {
        let new_events = res??;
        events.extend(new_events);
    }

    Ok(events)
}

pub async fn download_event(url: String) -> Result<Events> {
    let response: Vec<Event> = reqwest::get(url).await?.json().await?;
    let events = Events { events: response };

    Ok(events)
}
