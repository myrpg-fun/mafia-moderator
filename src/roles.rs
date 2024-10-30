use crate::MafiaRole;
// use crate::WerewolfRole;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Role {
    Mafia(MafiaRole),
    // Werewolf(WerewolfRole),
    None,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum RoleInfo {
    Night(NightRoleInfo),
    Passive(PassiveRoleInfo),
    Additional(AdditionalRoleInfo),
}

impl PartialEq<Role> for RoleInfo {
    fn eq(&self, other: &Role) -> bool {
        match self {
            RoleInfo::Night(night) => *other == night.role,
            RoleInfo::Passive(passive) => *other == passive.role,
            RoleInfo::Additional(additional) => *other == additional.role,
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
        }
    }

    pub fn get_check_role(&self) -> Role {
        match self {
            RoleInfo::Night(night) => night.check_role.unwrap_or(night.role),
            RoleInfo::Passive(passive) => passive.role,
            RoleInfo::Additional(additional) => additional.role,
        }
    }

    // getters role_icon
    pub fn get_role_icon(&self) -> &'static str {
        match self {
            RoleInfo::Night(night) => night.role_icon,
            RoleInfo::Additional(additional) => additional.role_icon,
            _ => "",
        }
    }

    // getters role_name
    pub fn get_role_name(&self) -> &'static str {
        match self {
            RoleInfo::Night(night) => night.role_name,
            RoleInfo::Passive(passive) => passive.role_name,
            _ => "",
        }
    }

    // getters role_name_color
    pub fn get_role_name_color(&self) -> &'static str {
        match self {
            RoleInfo::Night(night) => night.role_name_color,
            RoleInfo::Passive(passive) => passive.role_name_color,
            _ => "",
        }
    }

    // getters prepare_description
    pub fn get_prepare_description(&self) -> &'static str {
        match self {
            RoleInfo::Night(night) => night.prepare_description,
            RoleInfo::Passive(passive) => passive.prepare_description,
            RoleInfo::Additional(additional) => additional.prepare_description,
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
    pub role_icon: &'static str,
    pub role_name: &'static str,
    pub role_name_color: &'static str,
    pub prepare_description: &'static str,
    pub night_description: &'static str,
    pub targeting_rules: NightTargetingRules,
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
    pub role_icon: &'static str,
    pub prepare_description: &'static str,
}

impl PartialEq<Role> for AdditionalRoleInfo {
    fn eq(&self, other: &Role) -> bool {
        *other == self.role
    }
}
