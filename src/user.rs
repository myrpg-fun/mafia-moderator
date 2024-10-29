use leptos::*;
use std::collections::HashSet;

use crate::roles::*;

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub role: Role,
    pub additional_role: HashSet<Role>,
    pub choosed_by: HashSet<Role>,
    pub hystory_by: HashSet<Role>,
    pub is_alive: bool,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            role: Role::None,
            additional_role: HashSet::new(),
            choosed_by: HashSet::new(),
            hystory_by: HashSet::new(),
            is_alive: true,
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
        user.role = Role::None;
        user.is_alive = true;
        user.additional_role.clear();
        user.choosed_by.clear();
        user.hystory_by.clear();
    }
}
