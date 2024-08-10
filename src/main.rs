mod events;

fn main() {
    let response = reqwest::blocking::get(
        "https://raw.githubusercontent.com/statsbomb/open-data/master/data/events/3775620.json",
    )
    .unwrap();
    let events: events::Events = events::Events {
        events: response.json().unwrap(),
    };

    let events = events.filter_by_predicate(|event| event.block.is_some());
    println!("{}", events.len());
}
