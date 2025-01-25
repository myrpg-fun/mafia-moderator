use crate::user::user_flag::UserFlag;

use super::{role::Role, target_flag::TargetFlag};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum RoleInfo {
    Night(NightRoleInfo),
    Passive(PassiveRoleInfo),
    Additional(AdditionalRoleInfo),
    Icon(IconRoleInfo),
}

impl PartialEq<Role> for RoleInfo {
    fn eq(&self, other: &Role) -> bool {
        match self {
            RoleInfo::Night(night) => *other == night.role,
            RoleInfo::Passive(passive) => *other == passive.role,
            RoleInfo::Additional(additional) => *other == additional.role,
            RoleInfo::Icon(icon) => *other == icon.role,
        }
    }
}

impl RoleInfo {
    // getters role
    pub fn get_role(&self) -> Role {
        match self {
            RoleInfo::Night(night) => night.role,
            RoleInfo::Passive(passive) => passive.role,
            RoleInfo::Additional(additional) => additional.role,
            RoleInfo::Icon(icon) => icon.role,
        }
    }

    pub fn get_target_flag(&self) -> Option<TargetFlag> {
        match self {
            RoleInfo::Night(night) => Some(night.target_flag),
            _ => None,
        }
    }

    pub fn get_check_role(&self) -> Role {
        match self {
            RoleInfo::Night(night) => night.check_role.unwrap_or(night.role),
            RoleInfo::Passive(passive) => passive.role,
            RoleInfo::Additional(additional) => additional.role,
            RoleInfo::Icon(icon) => icon.role,
        }
    }

    // getters role_name
    pub fn get_role_name(&self) -> &'static str {
        match self {
            RoleInfo::Night(night) => night.role_name,
            RoleInfo::Passive(passive) => passive.role_name,
            RoleInfo::Additional(passive) => passive.role_name,
            RoleInfo::Icon(icon) => icon.role_name,
        }
    }

    // getters role_name_color
    pub fn get_role_name_color(&self) -> String {
        match self {
            RoleInfo::Night(night) => format!("{}{}", "text-", night.role_name_color),
            RoleInfo::Passive(passive) => format!("{}{}", "text-", passive.role_name_color),
            RoleInfo::Additional(passive) => format!("{}{}", "text-", passive.role_name_color),
            RoleInfo::Icon(icon) => format!("{}{}", "text-", icon.role_name_color),
        }
    }

    pub fn get_role_bg_color(&self) -> String {
        match self {
            RoleInfo::Night(night) => format!("bg-{}/50", night.role_name_color),
            RoleInfo::Passive(passive) => format!("bg-{}/50", passive.role_name_color),
            RoleInfo::Additional(passive) => format!("bg-{}/50", passive.role_name_color),
            RoleInfo::Icon(icon) => format!("bg-{}/50", icon.role_name_color),
        }
    }

    // getters prepare_description
    pub fn get_prepare_description(&self) -> &'static str {
        match self {
            RoleInfo::Night(night) => night.prepare_description,
            RoleInfo::Passive(passive) => passive.prepare_description,
            RoleInfo::Additional(additional) => additional.prepare_description,
            _ => "",
        }
    }

    // getters night_description
    pub fn get_night_description(&self) -> &'static str {
        match self {
            RoleInfo::Night(night) => night.night_description,
            _ => "",
        }
    }

    // getters can_choose_twice
    pub fn get_targeting_rules(&self) -> NightTargetingRules {
        match self {
            RoleInfo::Night(night) => night.targeting_rules,
            _ => NightTargetingRules::No,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NightTargetingRules {
    No,
    OnlyOne,
    NotTheSame,
    Anyone,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NightRoleInfo {
    pub role: Role,
    pub check_role: Option<Role>,
    pub role_name: &'static str,
    pub role_name_color: &'static str,
    pub prepare_description: &'static str,
    pub night_description: &'static str,
    pub targeting_rules: NightTargetingRules,
    pub target_flag: TargetFlag,
}

impl PartialEq<Role> for NightRoleInfo {
    fn eq(&self, other: &Role) -> bool {
        *other == self.role
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PassiveRoleInfo {
    pub role: Role,
    pub role_name: &'static str,
    pub role_name_color: &'static str,
    pub user_flag: Option<UserFlag>,
    pub prepare_description: &'static str,
}

impl PartialEq<Role> for PassiveRoleInfo {
    fn eq(&self, other: &Role) -> bool {
        *other == self.role
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AdditionalRoleInfo {
    pub role: Role,
    pub user_flag: UserFlag,
    pub role_name: &'static str,
    pub role_name_color: &'static str,
    pub prepare_description: &'static str,
}

impl PartialEq<Role> for AdditionalRoleInfo {
    fn eq(&self, other: &Role) -> bool {
        *other == self.role
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IconRoleInfo {
    pub role: Role,
    pub role_name: &'static str,
    pub role_name_color: &'static str,
}

impl PartialEq<Role> for IconRoleInfo {
    fn eq(&self, other: &Role) -> bool {
        *other == self.role
    }
}
