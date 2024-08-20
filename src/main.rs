use io::download_all_events;

mod events;
mod io;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://raw.githubusercontent.com/statsbomb/open-data/master/data/events/3775620.json"
            .to_string(),
        "https://raw.githubusercontent.com/statsbomb/open-data/master/data/events/3775619.json"
            .to_string(),
    ];

    let events = download_all_events(urls).await.ok().unwrap();
    println!("{}", events.len());
    let events = events.filter_by_predicate(|event| event.block.is_some());
    println!("{}", events.len());
}
