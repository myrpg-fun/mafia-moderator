use std::collections::HashSet;

use web_sys::js_sys::*;

use crate::roles::*;

#[derive(Clone, Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub is_guest: bool,
    pub role: HashSet<Role>,
    pub additional_role: HashSet<Role>,
    pub choosed_by: HashSet<Role>,
    pub history_by: Vec<(usize, HashSet<Role>)>,
    pub is_alive: bool,
    pub was_killed: bool,
}

impl Player {
    pub fn new(id: Option<String>, name: String) -> Self {
        let is_guest = id.is_none();
        let id = id.unwrap_or_else(|| format!("_{}", name.clone()));

        Self {
            id,
            name,
            is_guest,
            role: HashSet::new(),
            additional_role: HashSet::new(),
            choosed_by: HashSet::new(),
            history_by: Vec::new(),
            is_alive: true,
            was_killed: false,
        }
    }

    pub fn new_guest(name: String) -> Self {
        Self::new(None, name)
    }

    pub fn new_player(id: String, name: String) -> Self {
        Self::new(Some(id), name)
    }

    pub fn get_key(&self) -> String {
        self.id.clone()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.role == other.role
    }
}

pub fn reset_user_roles(users: &mut Vec<Player>) {
    for user in users.iter_mut() {
        user.role.clear();
        user.is_alive = true;
        user.was_killed = false;
        user.additional_role.clear();
        user.choosed_by.clear();
        user.history_by.clear();
    }
}

#[derive(Clone, Debug)]
pub struct UserSheetInfo {
    id: String,
    name: String,
    score: i32,
    mafia: UserMafiaSheetInfo,
    werewolf: UserWerewolfSheetInfo,
}

impl UserSheetInfo {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            score: 0,
            mafia: UserMafiaSheetInfo {
                score: 0,
                games: 0,
                wins: 0,
                win_citizen: 0,
                win_mafia: 0,
                win_maniac: 0,
                win_commissar: 0,
                win_prostitute: 0,
                win_doctor: 0,
                best_player: 0,
            },
            werewolf: UserWerewolfSheetInfo {
                score: 0,
                games: 0,
                wins: 0,
                win_villager: 0,
                win_werewolf: 0,
                best_player: 0,
            },
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl PartialEq<Player> for UserSheetInfo {
    fn eq(&self, other: &Player) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug)]
struct UserMafiaSheetInfo {
    score: i32,
    games: i32,
    wins: i32,
    win_citizen: i32,
    win_mafia: i32,
    win_maniac: i32,
    win_commissar: i32,
    win_prostitute: i32,
    win_doctor: i32,
    best_player: i32,
}

#[derive(Clone, Debug)]
struct UserWerewolfSheetInfo {
    score: i32,
    games: i32,
    wins: i32,
    win_villager: i32,
    win_werewolf: i32,
    best_player: i32,
}

impl From<Array> for UserSheetInfo {
    fn from(user_info: Array) -> Self {
        UserSheetInfo {
            id: user_info.get(0).as_string().unwrap(),
            name: user_info.get(1).as_string().unwrap(),
            score: user_info.get(2).as_string().unwrap().parse().unwrap(),
            mafia: UserMafiaSheetInfo {
                score: 0,
                games: 0,
                wins: 0,
                win_citizen: 0,
                win_mafia: 0,
                win_maniac: 0,
                win_commissar: 0,
                win_prostitute: 0,
                win_doctor: 0,
                best_player: 0,
            },
            werewolf: UserWerewolfSheetInfo {
                score: 0,
                games: 0,
                wins: 0,
                win_villager: 0,
                win_werewolf: 0,
                best_player: 0,
            },
        }
    }
}
