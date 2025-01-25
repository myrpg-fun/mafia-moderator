use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum UserFlag {
    Mafia(MafiaUserFlag),
    Werewolf(WerewolfUserFlag),
}

impl UserFlag {
    pub fn get_icon(&self) -> &'static str {
        match self {
            UserFlag::Werewolf(flag) => match flag {
                WerewolfUserFlag::DireWolfLove => "💙",
                WerewolfUserFlag::ToughGuyLive => "💛",
                WerewolfUserFlag::Mayor => "🎖️",
                WerewolfUserFlag::VillageIdiot => "🤪",
                WerewolfUserFlag::Pacifist => "🕊️",
                WerewolfUserFlag::VampireBite => "🩸",
                WerewolfUserFlag::PriestProtection => "🙏",
                WerewolfUserFlag::Silence => "🤐",
            },
            UserFlag::Mafia(_) => "",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum MafiaUserFlag {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WerewolfUserFlag {
    DireWolfLove,
    ToughGuyLive,
    Mayor,
    VillageIdiot,
    Pacifist,
    VampireBite,
    PriestProtection,
    Silence, //BodyguardProtection,
}
