use crate::events::events::{
    Card, EventSubType, EventType, FiftyFifty, FiftyFiftyOutcome, PlayPattern,
};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

pub(super) fn type_from_nested<'de, D>(deserializer: D) -> Result<EventType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: EventSubType = Deserialize::deserialize(deserializer)?;
    Ok(EventType::from_str(&s.name).unwrap())
}

pub(super) fn playpattern_nested<'de, D>(deserializer: D) -> Result<PlayPattern, D::Error>
where
    D: Deserializer<'de>,
{
    let s: EventSubType = Deserialize::deserialize(deserializer)?;
    let parsed: PlayPattern = PlayPattern::from_str(&s.name).unwrap();

    Ok(parsed)
}

pub(super) fn fifty_fifty_parser<'de, D>(deserializer: D) -> Result<Option<FiftyFifty>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Intermediate {
        outcome: EventSubType,
        counterpress: Option<bool>,
    }
    let s: Intermediate = Deserialize::deserialize(deserializer)?;
    let outcome: FiftyFiftyOutcome = match s.outcome.name.as_str() {
        "Won" => FiftyFiftyOutcome::Won,
        "Lost" => FiftyFiftyOutcome::Lost,
        "Success To Team" => FiftyFiftyOutcome::SuccessToTeam,
        "Success To Opposition" => FiftyFiftyOutcome::SuccessToOpposition,
        other => FiftyFiftyOutcome::Unknown(other.to_string()),
    };

    let parsed: FiftyFifty = FiftyFifty {
        outcome,
        counterpress: s.counterpress,
    };

    Ok(Some(parsed))
}

pub(super) fn bad_behaviour_parser<'de, D>(deserializer: D) -> Result<Option<Card>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Intermediate {
        card: EventSubType,
    }
    let s: Intermediate = Deserialize::deserialize(deserializer)?;

    let card: Card = match s.card.name.as_str() {
        "Yellow Card" => Card::YellowCard,
        "Second Yellow" => Card::SecondYellowCard,
        "Red Card" => Card::RedCard,
        other => Card::Unknown(other.to_string()),
    };

    Ok(Some(card))
}
