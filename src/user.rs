use std::collections::HashSet;

use crate::roles::*;

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub role: HashSet<Role>,
    pub additional_role: HashSet<Role>,
    pub choosed_by: HashSet<Role>,
    pub history_by: HashSet<Role>,
    pub is_alive: bool,
    pub was_killed: bool,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            role: HashSet::new(),
            additional_role: HashSet::new(),
            choosed_by: HashSet::new(),
            history_by: HashSet::new(),
            is_alive: true,
            was_killed: false,
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.role == other.role
    }
}

pub fn reset_user_roles(users: &mut Vec<User>) {
    for user in users.iter_mut() {
        user.role.clear();
        user.is_alive = true;
        user.was_killed = false;
        user.additional_role.clear();
        user.choosed_by.clear();
        user.history_by.clear();
    }
}
