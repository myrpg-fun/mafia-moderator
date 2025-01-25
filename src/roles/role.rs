use serde::Deserialize;
use serde::Serialize;

use crate::MafiaRole;
use crate::WerewolfRole;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Mafia(MafiaRole),
    Werewolf(WerewolfRole),
    None,
}
