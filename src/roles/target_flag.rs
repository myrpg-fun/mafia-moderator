use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum TargetFlag {
    Mafia(MafiaTargetFlag),
    Werewolf(WerewolfTargetFlag),
    WasKilled,
}

impl TargetFlag {
    pub fn get_icon(&self) -> &'static str {
        match self {
            TargetFlag::WasKilled => "❌",
            TargetFlag::Mafia(flag) => match flag {
                MafiaTargetFlag::Mafia => "🔫",
                MafiaTargetFlag::Detective => "🔍",
                MafiaTargetFlag::Doctor => "🚑",
                MafiaTargetFlag::Prostitute => "💋",
                MafiaTargetFlag::Maniac => "🔪",
                MafiaTargetFlag::Priest => "🙏",
                MafiaTargetFlag::Voted => "✋",
            },
            TargetFlag::Werewolf(flag) => match flag {
                WerewolfTargetFlag::Werewolf => "🐺",
                WerewolfTargetFlag::CursedConverted => "🧛",
                WerewolfTargetFlag::Seer => "🔍",
                WerewolfTargetFlag::Bodyguard => "🛡️",
                WerewolfTargetFlag::Priest => "🙏",
                WerewolfTargetFlag::WitchPoison => "☠️",
                WerewolfTargetFlag::WitchHeal => "🌿",
                WerewolfTargetFlag::Voted => "✋",
                WerewolfTargetFlag::Vampire => "🩸",
                WerewolfTargetFlag::VampireVoted => "🧛",
                WerewolfTargetFlag::Mentalist => "👁️",
                WerewolfTargetFlag::ParanormalInvestigator => "📸",
                WerewolfTargetFlag::Spellcaster => "🤐",
                WerewolfTargetFlag::Huntress => "🏹",
                WerewolfTargetFlag::Revealer => "🔦",
                WerewolfTargetFlag::DireWolfLove => "💙",
                WerewolfTargetFlag::Lovers => "💞",
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum MafiaTargetFlag {
    Mafia,
    Detective,
    Doctor,
    Prostitute,
    Maniac,
    Priest,
    Voted,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WerewolfTargetFlag {
    Werewolf,
    CursedConverted,
    Seer,
    Bodyguard,
    Priest,
    WitchPoison,
    WitchHeal,
    Voted,
    Vampire,
    VampireVoted,
    Mentalist,
    ParanormalInvestigator,
    Spellcaster,
    Huntress,
    Revealer,
    DireWolfLove,
    Lovers,
}
