use crate::events::parsers::{
    bad_behaviour_parser, fifty_fifty_parser, playpattern_nested, type_from_nested,
};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::ops::Index;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct Events {
    pub events: Vec<Event>,
}

pub struct EventsIter {
    events: Events,
    index: usize,
}
pub struct RefEventsIter<'a> {
    events: &'a Events,
    index: usize,
}

impl Iterator for EventsIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.events.events.len() {
            return None;
        }
        let result = Some(self.events[self.index].clone());
        self.index += 1;
        result
    }
}

impl<'a> Iterator for RefEventsIter<'a> {
    type Item = &'a Event;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.events.events.len() {
            return None;
        }
        let result = Some(&self.events[self.index]);
        self.index += 1;
        result
    }
}

impl<'a> IntoIterator for &'a Events {
    type Item = &'a Event;
    type IntoIter = RefEventsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            events: &self,
            index: 0,
        }
    }
}

impl IntoIterator for Events {
    type Item = Event;
    type IntoIter = EventsIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            events: self,
            index: 0,
        }
    }
}

impl Index<usize> for Events {
    type Output = Event;

    fn index(&self, index: usize) -> &Self::Output {
        &self.events[index]
    }
}

impl Events {
    pub fn retain(&mut self, event_type: EventType) -> () {
        self.events.retain(|event| event.event_type == event_type)
    }

    pub fn filter_by_event_type(&self, event_type: EventType) -> Self {
        Self {
            events: self
                .events
                .iter()
                .filter(|event| event.event_type == event_type)
                .cloned()
                .collect(),
        }
    }

    pub fn filter_by_team(&self, team: &str) -> Self {
        Self {
            events: self
                .events
                .iter()
                .filter(|event| event.team.name == team)
                .cloned()
                .collect(),
        }
    }

    pub fn filter_by_player(&self, player: &str) -> Self {
        Self {
            events: self
                .events
                .iter()
                .filter(|event| {
                    if event.player.is_none() {
                        return false;
                    } else {
                        return event.player.as_ref().unwrap().name == player;
                    }
                })
                .cloned()
                .collect(),
        }
    }

    pub fn filter_by_predicate<F>(&self, predicate: F) -> Self
    where
        F: Fn(&Event) -> bool,
    {
        Self {
            events: self
                .events
                .iter()
                .filter(|event| predicate(event))
                .cloned()
                .collect(),
        }
    }

    pub fn extend(&mut self, other: Self) -> () {
        self.events.extend(other)
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    pub id: String,
    pub index: u16,
    pub period: Period,
    pub timestamp: String,
    pub minute: u8,
    pub second: u8,
    #[serde(deserialize_with = "type_from_nested")]
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub possesion: Option<u16>,
    pub possesion_team: Option<EventSubType>,
    #[serde(deserialize_with = "playpattern_nested")]
    pub play_pattern: PlayPattern,
    pub team: EventSubType,
    pub player: Option<EventSubType>,
    pub position: Option<EventSubType>,
    pub location: Option<(f32, f32)>,
    pub duration: Option<f32>,
    pub tactics: Option<Box<Tactics>>,
    pub under_pressure: Option<bool>,
    pub out: Option<bool>,
    pub off_camera: Option<bool>,
    pub related_events: Option<Vec<String>>,
    #[serde(deserialize_with = "fifty_fifty_parser")]
    #[serde(rename = "50_50")]
    #[serde(default)]
    pub fifty_fifty: Option<FiftyFifty>,
    #[serde(deserialize_with = "bad_behaviour_parser")]
    #[serde(default)]
    pub bad_behaviour: Option<Card>,
    pub ball_receipt: Option<BallReceipt>,
    pub ball_recovery: Option<BallRecovery>,
    pub block: Option<Block>,
    pub clearance: Option<Clearance>,
    pub pass: Option<Pass>,
    pub carry: Option<Carry>,
    pub duel: Option<Duel>,
    pub dribble: Option<Dribble>,
    pub dribbled_past: Option<DribbledPast>,
    pub foul_comitted: Option<FoulCommitted>,
    pub foul_won: Option<FoulWon>,
    pub goalkeeper: Option<GoalKeeper>,
    pub half_end: Option<HalfEnd>,
    pub half_start: Option<HalfStart>,
    pub injury_stoppage: Option<InjuryStoppage>,
    pub player_off: Option<PlayerOff>,
    pub pressure: Option<Pressure>,
    pub shot: Option<Shot>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum EventType {
    BallReceipt,
    BallRecovery,
    Dispossesed,
    Duel,
    CameraOn,
    Block,
    Offside,
    Clearance,
    Interception,
    Dribble,
    Shot,
    Pressure,
    HalfStart,
    Substitution,
    OwnGoalAgainst,
    FoulWon,
    FoulCommitted,
    GoalKeeper,
    BadBehaviour,
    OwnGoalFor,
    PlayerOn,
    PlayerOff,
    Shield,
    Pass,
    FiftyFifty,
    HalfEnd,
    StartingEleven,
    TacticalShift,
    Error,
    Miscontrol,
    DribbledPast,
    InjuryStoppage,
    RefereeBallDrop,
    Carry,
    Unknown(String),
}

impl FromStr for EventType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Ball Receipt*" => Ok(Self::BallReceipt),
            "Ball Recovery" => Ok(Self::BallRecovery),
            "Dispossessed" => Ok(Self::Dispossesed),
            "Duel" => Ok(Self::Duel),
            "Camera On" => Ok(Self::CameraOn),
            "Block" => Ok(Self::Block),
            "Offside" => Ok(Self::Offside),
            "Clearance" => Ok(Self::Clearance),
            "Interception" => Ok(Self::Interception),
            "Dribble" => Ok(Self::Dribble),
            "Shot" => Ok(Self::Shot),
            "Pressure" => Ok(Self::Pressure),
            "Half Start" => Ok(Self::HalfStart),
            "Substitution" => Ok(Self::Substitution),
            "Own Goal Against" => Ok(Self::OwnGoalAgainst),
            "Foul Won" => Ok(Self::FoulWon),
            "Foul Committed" => Ok(Self::FoulCommitted),
            "Goal Keeper" => Ok(Self::GoalKeeper),
            "Bad Behaviour" => Ok(Self::BadBehaviour),
            "Own Goal For" => Ok(Self::OwnGoalFor),
            "Player On" => Ok(Self::PlayerOn),
            "Player Off" => Ok(Self::PlayerOff),
            "Shield" => Ok(Self::Shield),
            "Pass" => Ok(Self::Pass),
            "50/50" => Ok(Self::FiftyFifty),
            "Half End" => Ok(Self::HalfEnd),
            "Starting XI" => Ok(Self::StartingEleven),
            "Tactical Shift" => Ok(Self::TacticalShift),
            "Error" => Ok(Self::Error),
            "Miscontrol" => Ok(Self::Miscontrol),
            "Dribbled Past" => Ok(Self::DribbledPast),
            "Injury Stoppage" => Ok(Self::InjuryStoppage),
            "Referee Ball-Drop" => Ok(Self::RefereeBallDrop),
            "Carry" => Ok(Self::Carry),
            other => Ok(Self::Unknown(other.to_string())),
        }
    }
}

impl FromStr for PlayPattern {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Regular Play" => Ok(Self::RegularPlay),
            "FromCorner" => Ok(Self::FromCorner),
            "From Free Kick" => Ok(Self::FromFreeKick),
            "From Throw In" => Ok(Self::FromThrowIn),
            "Other" => Ok(Self::Other),
            "From Counter" => Ok(Self::FromCounter),
            "From Goal Kick" => Ok(Self::FromGoalKick),
            "From Keeper" => Ok(Self::FromKeeper),
            "From KickOff" => Ok(Self::FromKickOff),
            _ => Ok(Self::Other),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Card {
    YellowCard,
    SecondYellowCard,
    RedCard,
    Unknown(String),
}

#[derive(Deserialize, Debug, Clone)]
pub enum PlayPattern {
    RegularPlay,
    FromCorner,
    FromFreeKick,
    FromThrowIn,
    Other,
    FromCounter,
    FromGoalKick,
    FromKeeper,
    FromKickOff,
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum Period {
    FirstHalf = 1,
    SecondHalf = 2,
    ThirdPeriod = 3,
    FourthPeriod = 4,
    PenaltyShootout = 5,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Duel {
    #[serde(default = "bool::default")]
    pub counterpress: bool,
    pub r#type: EventSubType,
    pub outcome: Option<EventSubType>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DribbledPast {
    #[serde(default = "bool::default")]
    pub counterpress: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FoulCommitted {
    #[serde(default = "bool::default")]
    pub counterpress: bool,
    #[serde(default = "bool::default")]
    pub offensive: bool,
    pub r#type: EventSubType,
    #[serde(default = "bool::default")]
    pub advantage: bool,
    #[serde(default = "bool::default")]
    pub penalty: bool,
    pub card: Option<Card>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FoulWon {
    #[serde(default = "bool::default")]
    pub defensive: bool,
    #[serde(default = "bool::default")]
    pub advantage: bool,
    #[serde(default = "bool::default")]
    pub penalty: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoalKeeper {
    pub position: Option<EventSubType>,
    pub technique: Option<EventSubType>,
    pub body_part: Option<EventSubType>,
    pub r#type: EventSubType,
    pub outcome: Option<EventSubType>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HalfEnd {
    #[serde(default = "bool::default")]
    #[serde(rename = "Early Video End")]
    pub early_video_end: bool,
    #[serde(default = "bool::default")]
    #[serde(rename = "Match Suspended")]
    pub match_suspended: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HalfStart {
    #[serde(default = "bool::default")]
    #[serde(rename = "Late Video Start")]
    pub late_video_start: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InjuryStoppage {
    #[serde(default = "bool::default")]
    pub in_chain: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Dribble {
    #[serde(default = "bool::default")]
    #[serde(rename = "Overrun")]
    pub overrun: bool,
    #[serde(default = "bool::default")]
    #[serde(rename = "Nutmeg")]
    pub nutmeg: bool,
    pub outcome: EventSubType,
    #[serde(default = "bool::default")]
    #[serde(rename = "No Touch")]
    pub no_touch: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Clearance {
    #[serde(default = "bool::default")]
    pub aerial_won: bool,
    pub body_part: EventSubType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Interception {
    pub outcome: EventSubType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Miscontrol {
    #[serde(default = "bool::default")]
    #[serde(rename = "Nutmeg")]
    pub aerial_won: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pass {
    #[serde(default = "bool::default")]
    pub backheel: bool,
    #[serde(default = "bool::default")]
    pub deflected: bool,
    #[serde(default = "bool::default")]
    pub miscommunication: bool,
    #[serde(default = "bool::default")]
    pub cross: bool,
    #[serde(default = "bool::default")]
    #[serde(rename = "cut-back")]
    pub cut_back: bool,
    #[serde(default = "bool::default")]
    pub switch: bool,
    #[serde(default = "bool::default")]
    #[serde(rename = "shot-assist")]
    pub shot_assist: bool,
    #[serde(default = "bool::default")]
    #[serde(rename = "goal-assist")]
    pub goal_assist: bool,
    pub body_part: Option<EventSubType>,
    pub r#type: Option<EventSubType>,
    pub outcome: Option<EventSubType>,
    #[serde(rename = "Technique")]
    pub technique: Option<EventSubType>,
    pub recipient: Option<EventSubType>,
    pub length: f32,
    pub angle: f32,
    pub height: EventSubType,
    pub end_location: (f32, f32),
    pub assisted_shot_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Shot {
    #[serde(default = "bool::default")]
    pub aerial_won: bool,
    #[serde(default = "bool::default")]
    pub follows_dribble: bool,
    #[serde(default = "bool::default")]
    pub first_time: bool,
    #[serde(default = "bool::default")]
    pub open_goal: bool,
    #[serde(default = "bool::default")]
    #[serde(rename = "cut-back")]
    pub deflected: bool,
    pub statsbomb_xg: f32,
    pub body_part: Option<EventSubType>,
    pub r#type: Option<EventSubType>,
    pub outcome: Option<EventSubType>,
    #[serde(rename = "Technique")]
    pub technique: Option<EventSubType>,
    pub freeze_frame: Option<Vec<FreezeFrame>>,
    pub end_location: Vec<f32>,
    pub key_pass_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Block {
    #[serde(default = "bool::default")]
    pub deflection: bool,
    #[serde(default = "bool::default")]
    pub offensive: bool,
    #[serde(default = "bool::default")]
    pub save_block: bool,
    #[serde(default = "bool::default")]
    pub counterpress: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerOff {
    #[serde(default = "bool::default")]
    #[serde(rename = "Permenant")]
    pub permenant: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pressure {
    #[serde(default = "bool::default")]
    pub counterpress: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FiftyFifty {
    pub outcome: FiftyFiftyOutcome,
    pub counterpress: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum FiftyFiftyOutcome {
    Won,
    Lost,
    SuccessToTeam,
    SuccessToOpposition,
    Unknown(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct BallReceipt {
    pub outcome: EventSubType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BallRecovery {
    pub recovery_failure: Option<bool>,
    pub offensive: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Carry {
    pub end_location: (f32, f32),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tactics {
    pub formation: u16,
    pub lineup: [LineupPlayer; 11],
}

#[derive(Deserialize, Debug, Clone)]
pub struct FreezeFrame {
    pub location: (f32, f32),
    pub player: EventSubType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LineupPlayer {
    pub player: EventSubType,
    pub position: EventSubType,
    pub jersey_number: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventSubType {
    pub id: u32,
    pub name: String,
}
