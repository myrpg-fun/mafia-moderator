use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use web_sys::js_sys::*;

use crate::roles::*;
use leptos::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub comment: String,
    pub is_guest: bool,
    pub role: HashSet<Role>,
    pub additional_role: HashSet<Role>,
    pub choosed_by: HashSet<Role>,
    pub history_by: Vec<(usize, HashSet<Role>)>,
    pub is_alive: bool,
    pub was_killed: bool,
}

impl Player {
    pub fn new(id: String, name: String, comment: String, is_guest: bool) -> Self {
        Self {
            id,
            name,
            comment,
            is_guest,
            role: HashSet::new(),
            additional_role: HashSet::new(),
            choosed_by: HashSet::new(),
            history_by: Vec::new(),
            is_alive: true,
            was_killed: false,
        }
    }

    pub fn new_guest(id: String, name: String) -> Self {
        Self::new(id, name, "".to_string(), true)
    }

    pub fn new_player(id: String, name: String, comment: String) -> Self {
        Self::new(id, name, comment, false)
    }

    pub fn get_key(&self) -> String {
        self.id.clone()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.role == other.role
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
    comment: String,
    is_guest: bool,
    score: i32,
}

impl UserSheetInfo {
    pub fn new(id: String, name: String, comment: String, is_guest: bool) -> Self {
        Self {
            id,
            name,
            comment,
            is_guest,
            score: 0,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn is_guest(&self) -> bool {
        self.is_guest
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn comment(&self) -> String {
        self.comment.clone()
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
        // name index:1 ^name ... (... comment ...)
        let name = user_info.get(1).as_string().unwrap();
        // comment in brackets
        let comment = name
            .split("(")
            .nth(1)
            .map(|s| s.split(")").next().unwrap_or(""))
            .unwrap_or("")
            .to_string();
        let name = name.split("(").next().unwrap_or("").trim().to_string();

        UserSheetInfo {
            id: user_info.get(0).as_string().unwrap(),
            name,
            comment,
            is_guest: false,
            score: user_info
                .get(3)
                .as_string()
                .unwrap_or("0".to_string())
                .parse()
                .unwrap_or(0),
        }
    }
}
