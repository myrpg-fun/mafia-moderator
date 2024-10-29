use crate::MafiaRole;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Role {
    Mafia(MafiaRole),
    None,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RoleInfo {
    pub role: Role,
    pub role_icon: &'static str,
    pub role_name: &'static str,
    pub role_name_color: &'static str,
    pub prepare_description: &'static str,
    pub night_description: &'static str,
    pub can_choose_twice: bool,
    pub is_night_role: bool,
}
