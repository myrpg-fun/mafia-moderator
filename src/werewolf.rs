use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::hash::Hash;

use crate::roles::*;
use crate::user::*;
use crate::GameContext;
use crate::GameContextHistory;
use crate::GameState;
use leptos::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WerewolfRole {
    None,
    Villager,
    Werewolf,
    Minion,
    DireWolf,
    // LoneWolf,
    // WolfCub,
    Cursed,
    Bodyguard,
    Priest,
    WitchHeal,
    WitchPoison,
    Seer,
    Huntress,
    Spellcaster,
    ToughGuy,
    Lycan,
    Mayor,
    Hunter,
    //    ParanormalInvestigator,
    Prince,
    Diseased,
    Mason,
    Lovers,
    // Doppelganger,
    // AlphaWolf,
    // MadBomber,
    //*** TODO:
    Revealer,
    // ApprenticeSeer,
    // AuraSeer,
}

const _WEREWOLF_COLORS: [&str; 10] = [
    "text-red-950",
    "text-blue-950",
    "text-gray-950",
    "text-green-950",
    "text-purple-950",
    "bg-red-950/50",
    "bg-blue-950/50",
    "bg-gray-950/50",
    "bg-green-950/50",
    "bg-purple-950/50",
];

pub const WEREWOLF_ROLES: [RoleInfo; 20] = [
    RoleInfo::Icon(IconRoleInfo {
        role: Role::Werewolf(WerewolfRole::Villager),
        role_icon: "✋",
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Bodyguard),
        check_role: None,
        role_name: "Bodyguard",
        role_name_color: "green-950",
        role_icon: "🛡️",
        prepare_description: "Выберите игрока Bodyguard",
        night_description: "Кого защитит Bodyguard?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Priest),
        check_role: None,
        role_icon: "🙏",
        role_name: "Priest",
        role_name_color: "green-950",
        prepare_description: "Выберите игрока Priest",
        night_description: "Кого освятит Priest?",
        targeting_rules: NightTargetingRules::OnlyOne,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Werewolf),
        check_role: None,
        role_name: "Werewolf",
        role_name_color: "red-950",
        role_icon: "🐺",
        prepare_description: "Выберите игроков Werewolf",
        night_description: "Кого убьют Werewolf?",
        targeting_rules: NightTargetingRules::Anyone,
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Minion),
        role_name: "Minion",
        role_icon: "",
        additional_role: None,
        role_name_color: "red-950",
        prepare_description: "Выберите игрока Minion",
    }),
    RoleInfo::Additional(AdditionalRoleInfo {
        role: Role::Werewolf(WerewolfRole::DireWolf),
        role_name: "Dire Wolf",
        role_name_color: "red-950",
        role_icon: "💙",
        prepare_description: "Поставте сердечки игроку Dire Wolf и в кого он влюблен",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Cursed),
        role_name: "Cursed",
        role_icon: "",
        additional_role: None,
        role_name_color: "purple-950",
        prepare_description: "Выберите игрока Cursed",
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::WitchHeal),
        check_role: None,
        role_icon: "🌿",
        role_name: "Witch",
        role_name_color: "green-950",
        prepare_description: "Выберите игрока Witch",
        night_description: "Кого вылечит Witch?",
        targeting_rules: NightTargetingRules::OnlyOne,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::WitchPoison),
        check_role: Some(Role::Werewolf(WerewolfRole::WitchHeal)),
        role_icon: "☠️",
        role_name: "Witch",
        role_name_color: "green-950",
        prepare_description: "",
        night_description: "Кого отравит Witch?",
        targeting_rules: NightTargetingRules::OnlyOne,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Seer),
        check_role: None,
        role_name: "Seer",
        role_name_color: "blue-950",
        role_icon: "🔍",
        prepare_description: "Выберите игрока Seer",
        night_description: "Кого проверит Seer?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Spellcaster),
        check_role: None,
        role_name: "Spellcaster",
        role_name_color: "blue-950",
        role_icon: "🤐",
        prepare_description: "Выберите игрока Spellcaster",
        night_description: "Кого заглушил Spellcaster?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Huntress),
        check_role: None,
        role_name: "Huntress",
        role_name_color: "blue-950",
        role_icon: "🏹",
        prepare_description: "Выберите игрока Huntress",
        night_description: "Кого убъет Huntress?",
        targeting_rules: NightTargetingRules::OnlyOne,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Revealer),
        check_role: None,
        role_name: "Revealer",
        role_name_color: "blue-950",
        role_icon: "🔦",
        prepare_description: "Выберите игрока Revealer",
        night_description: "Кого убъет Revealer?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Mason),
        role_name: "Mason",
        role_icon: "",
        additional_role: None,
        role_name_color: "purple-950",
        prepare_description: "Выберите игроков Mason",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Lovers),
        role_name: "Lovers",
        role_icon: "❤️",
        additional_role: None,
        role_name_color: "purple-950",
        prepare_description: "Выберите игроков Lovers",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::ToughGuy),
        role_name: "ToughGuy",
        role_icon: "💛",
        additional_role: Some(Role::Werewolf(WerewolfRole::ToughGuy)),
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока ToughGuy",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Mayor),
        role_name: "Mayor",
        role_icon: "🎖️",
        additional_role: Some(Role::Werewolf(WerewolfRole::Mayor)),
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Mayor",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Lycan),
        role_name: "Lycan",
        role_icon: "",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Lycan",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Prince),
        role_name: "Prince",
        role_icon: "",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Prince",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Diseased),
        role_name: "Diseased",
        role_icon: "",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Diseased",
    }),
];

#[derive(Clone, Copy, Debug)]
pub enum WerewolfGameState<'a> {
    SelectActiveRoles,
    SetupRoles(&'a RoleInfo),
    Day,
    Night(&'a RoleInfo),
    End,
}

fn get_next_prepare_role(active_roles: HashSet<Role>, role: Role) -> Option<&'static RoleInfo> {
    let role_index = WEREWOLF_ROLES
        .iter()
        .position(|r| r.get_role() == role)
        .unwrap();

    let next_role = WEREWOLF_ROLES
        .iter()
        .skip(role_index.wrapping_add(1))
        .find(|r| active_roles.contains(&r.get_role()) && !r.get_prepare_description().is_empty());

    next_role
}

fn get_next_night_role(role: &RoleInfo) -> Option<&'static RoleInfo> {
    let role_index = WEREWOLF_ROLES.iter().position(|r| r == role).unwrap();

    // Find next night role
    let next_role = WEREWOLF_ROLES
        .iter()
        .skip(role_index.wrapping_add(1))
        .find(|r| matches!(r, RoleInfo::Night(_)));

    next_role
}

const ROLES_STORAGE_KEY: &str = "werewolf_active_roles";

#[derive(Clone, Debug)]
struct WerewolfActiveRoles {
    roles: HashSet<Role>,
}

impl Default for WerewolfActiveRoles {
    fn default() -> Self {
        let starting_roles = window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| {
                storage
                    .get_item(ROLES_STORAGE_KEY)
                    .ok()
                    .flatten()
                    .and_then(|value| serde_json::from_str::<HashSet<Role>>(&value).ok())
            })
            .unwrap_or_else(|| {
                let mut roles = HashSet::new();
                WEREWOLF_ROLES.iter().for_each(|r| match r {
                    RoleInfo::Icon(_) => {}
                    _ => {
                        roles.insert(r.get_role());
                    }
                });
                roles
            });

        Self {
            roles: starting_roles,
        }
    }
}

#[component]
pub fn WerewolfGameView() -> impl IntoView {
    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    let (active_werewolf_roles, set_active_werewolf_roles) =
        create_signal::<WerewolfActiveRoles>(WerewolfActiveRoles::default());

    provide_context(active_werewolf_roles);

    let game_state_view = move || match mafia_context.game_state.get() {
        GameState::Werewolf(game_state) => match game_state {
            WerewolfGameState::SelectActiveRoles => view! {
                <SelectActiveRoles active_werewolf_roles set_active_werewolf_roles />
            }
            .into_view(),

            WerewolfGameState::SetupRoles(role) => view! {
                <SetupRolesView role={role} />
            }
            .into_view(),

            WerewolfGameState::Day => view! {
                <DayVote />
            }
            .into_view(),
            WerewolfGameState::Night(role) => view! {
                <NightTurn role_info={role} />
            }
            .into_view(),
            WerewolfGameState::End => view! {
                <div>"Конец игры"</div>
            }
            .into_view(),
        },
        _ => view! {
            <div>"Ошибка"</div>
        }
        .into_view(),
    };

    view! {
        <div class="relative flex flex-col gap-4 w-full h-full">
            <h1 class="text-lg relative w-full text-left">
                "Werewolf"
                <button
                    class="absolute text-sm right-0 top-0 px-2 py-1 bg-gray-200 rounded-full"
                    on:click=move |_| if window().confirm_with_message("Вернуться в главное меню?").expect("REASON") {
                        mafia_context.game_state.set(GameState::SetupNames);
                    }>
                    "Главное меню"
                </button>
            </h1>
            {game_state_view}
        </div>
    }
}

#[component]
fn SelectActiveRoles(
    active_werewolf_roles: ReadSignal<WerewolfActiveRoles>,
    set_active_werewolf_roles: WriteSignal<WerewolfActiveRoles>,
) -> impl IntoView {
    let roles = WEREWOLF_ROLES.iter().filter(|r| match r {
        RoleInfo::Night(_) => r.get_check_role() == r.get_role(),
        RoleInfo::Passive(_) => true,
        RoleInfo::Additional(_) => true,
        _ => false,
    });

    let is_selected = move |role: &Role| active_werewolf_roles.get().roles.contains(role);

    create_effect(move |_| {
        if let Ok(Some(storage)) = window().local_storage() {
            let user_names = &active_werewolf_roles.get().roles;
            let json = serde_json::to_string(user_names).expect("couldn't serialize Users");
            if storage.set_item(ROLES_STORAGE_KEY, &json).is_err() {
                //log::error!("error while trying to set item in localStorage");
            }
        }
    });

    let onclick_next_role = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>()
            .expect("MafiaContext history not found");

        set_context_history.update(|history| history.push(game_ctx.get_history()));

        // find first selected role from list
        let role_info = WEREWOLF_ROLES
            .iter()
            .find(|role_info| is_selected(&role_info.get_check_role()));

        if let Some(role_info) = role_info {
            game_ctx
                .game_state
                .set(GameState::Werewolf(WerewolfGameState::SetupRoles(
                    role_info,
                )));
        }
    };

    let onclick_prev_role = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>()
            .expect("MafiaContext history not found");

        set_context_history.update(|history| {
            if let Some(prev_ctx) = history.pop() {
                game_ctx.set_history(prev_ctx);
            }
        });
    };

    view! {
        <div class="flex-1 flex flex-col gap-1 relative overflow-auto px-4 -mx-4">
            {roles.map(|role| {
                let role_clone = role.get_role().clone();
                view!{
                    <button class=move ||
                        format!("rounded-full px-3 py-1 text-sm {}", if is_selected(
                            &role.get_role()
                        ) {
                            format!("text-white {}", role.get_role_bg_color())
                        } else {
                            "bg-gray-200".to_string()
                        })
                        on:click=move|_|{
                            set_active_werewolf_roles.update(|active_werewolf_roles|{
                                if active_werewolf_roles.roles.contains(&role_clone) {
                                    active_werewolf_roles.roles.remove(&role_clone);
                                }else{
                                    active_werewolf_roles.roles.insert(role_clone);
                                }
                            });
                        }
                    >
                        {role.get_role_name()}
                    </button>
                }
            }).collect::<Vec<_>>().into_view()}
        </div>
        <div class="flex gap-2 w-full items-center">
            <button
                class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click=onclick_prev_role
            >
                "Назад"
            </button>
            <button
                class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click=onclick_next_role
            >
                "Далее"
            </button>
        </div>
    }
}

#[component]
fn UserRow(user: Player) -> impl IntoView {
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");

    view! {
        <div class="flex gap-2">
            <div class="flex-1 px-3 py-1 text-base bg-gray-200 rounded-full">{user.name.clone()}</div>
            <button class="text-lg"
                on:click=move |_| {
                    game_ctx.users.update(|users| users.retain(|u| *u != user));
                }
            >
                "✕"
            </button>
        </div>
    }
}

#[component]
fn SetupRolesView<'a>(role: &'a RoleInfo) -> impl IntoView {
    view! {
        <SetupRolesHeader role=role />
        <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
            <div class="flex-1"></div>
            <div class="flex flex-col gap-1 w-full">
                <SelectUserForRole role_info=role />
            </div>
        </div>
        <TurnButtons role_info=role />
    }
}

#[component]
fn SetupRolesHeader<'a>(role: &'a RoleInfo) -> impl IntoView {
    let active_roles =
        use_context::<ReadSignal<WerewolfActiveRoles>>().expect("WerewolfActiveRoles exists");

    let roles_iter = move || {
        WEREWOLF_ROLES.iter().filter(move |r| {
            active_roles.get().roles.contains(&r.get_role())
                && !r.get_prepare_description().is_empty()
        })
    };

    let len = move || roles_iter().count();
    let role_clone = role.clone();
    let index = move || roles_iter().position(|r| r == &role_clone).unwrap_or(0) + 1;

    view! {
        <div class="flex flex-col gap-2">
            <h2 class="text-base">
            <span class="bg-black/5 mr-2 rounded-md px-1.5 text-xs py-0.5">{index} <span class="text-black/30">" / " {len}</span></span>{role.get_prepare_description()}
            </h2>
        </div>
    }
}

#[component]
fn SelectUserForRole<'a>(role_info: &'a RoleInfo) -> impl IntoView {
    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    let users = move || mafia_context.users.get();
    let role = role_info.get_role();
    let role_info = role_info.clone();

    view! {
        <div class="grid grid-cols-3 gap-1">
            <For
                each=users
                key=|user| format!("{}-{}-{}-{}", user.name, user.role.len(), user.additional_role.len(), user.choosed_by.len())
                children=move |user| {
                    let user_clone = user.clone();

                    view!{
                        <UserSelectRole
                            user=user_clone
                            disabled=false
                            highlighted=false
                            killed=false
                            is_selected=move |u| {
                                match role_info{
                                    RoleInfo::Additional(_) => u.additional_role.contains(&role),
                                    _ => u.role.contains(&role),
                                }
                            }
                            on:click=move |_| {
                                mafia_context.users.update(|users| {
                                    if let Some(user) = users.iter_mut().find(|u| **u == user) {
                                        match role_info{
                                            RoleInfo::Additional(_) => {
                                                if user.additional_role.contains(&role){
                                                    user.additional_role.remove(&role);
                                                }else{
                                                    user.additional_role.insert(role);
                                                }
                                            }
                                            RoleInfo::Night(_) => {
                                                if user.role.contains(&role){
                                                    user.role.remove(&role);
                                                }else{
                                                    user.role.insert(role);
                                                }
                                            }
                                            RoleInfo::Passive(passive_role_info) => {
                                                if user.role.contains(&role){
                                                    user.role.remove(&role);
                                                    if let Some(additional_role) = passive_role_info.additional_role {
                                                        user.additional_role.remove(&additional_role);
                                                    }
                                                }else{
                                                    user.role.insert(role);
                                                    if let Some(additional_role) = passive_role_info.additional_role {
                                                        user.additional_role.insert(additional_role);
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                });
                            }
                        />
                    }
                }
            />
        </div>
    }
}

#[component]
fn UserSelectRole(
    user: Player,
    is_selected: impl Fn(&Player) -> bool + 'static,
    disabled: bool,
    killed: bool,
    highlighted: bool,
) -> impl IntoView {
    let history = user.history_by.clone();
    let choosed = user.choosed_by.clone();
    let user_clone = user.clone();

    view! {
        <button
            disabled=disabled
            class=move || {
                let mut main_class = "relative flex-1 px-3 py-1 text-sm rounded-2xl flex flex-col items-center justify-start".to_string();
                main_class.push_str(if highlighted {
                    " ring-1 ring-red-600/50"
                } else {
                    ""
                });
                main_class.push_str(if killed {
                    " opacity-20 bg-white hover:opacity-90"
                } else if disabled {
                    " opacity-60 bg-gray-100 hover:opacity-90"
                } else if is_selected(&user_clone) {
                    " bg-blue-300"
                } else {
                    " bg-gray-200"
                });
                main_class
            }
        >
            {move || if user.was_killed {
                view! {
                    <div class="absolute text-[0.5rem] right-2 top-[0.15rem]">"❌"</div>
                }.into_view()
            }else{
                "".into_view()
            }}
            <div class="flex items-baseline justify-center flex-wrap">{user.name} <UserAdditionalRoles roles=user.additional_role /></div>
            <UserRoleNames role=user.role />
            <UserHistory hystory=history current=choosed />
        </button>
    }
}

#[component]
fn UserRoleNames(role: HashSet<Role>) -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-x-1 items-start justify-center">
            {move || {
                if role.is_empty() {
                    view!{
                        <UserRoleName role=Role::Werewolf(WerewolfRole::None) />
                    }.into_view()
                }else{
                    role.iter().map(|role| {
                        let role = *role;

                        view!{
                            <UserRoleName role=role />
                        }
                    }).intersperse_with(|| {
                        // Add separator between the roles
                        view! { <Separator /> } // Assuming there's a Separator component; alternatively, use raw HTML
                    })
                    .collect::<Vec<_>>().into_view()
                }
            }}
        </div>
    }
}

#[component]
fn Separator() -> impl IntoView {
    view! {
        <div class="text-xs opacity-50">"•"</div>
    }
}

#[component]
fn UserHistory(hystory: Vec<(usize, HashSet<Role>)>, current: HashSet<Role>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-0.5 flex-wrap min-h-4">
            {
                hystory.iter().map(|(round, roles)| {
                    view!{
                        <div class="flex items-center">
                            <div class="text-xs opacity-60 mr-0.5">{round.into_view()}"."</div>
                            {
                                roles.iter().map(|role| {
                                    let role = *role;

                                    view!{
                                        <UserRoleIcon role=role is_history=UserRoleIconType::History />
                                    }.into_view()
                                }).collect::<Vec<_>>().into_view()
                            }
                        </div>
                    }
                }).collect::<Vec<_>>().into_view()
            }
            <div class="flex items-center gap-0.5">
            {
                current.iter().map(|role| {
                    let role = *role;

                    view!{
                        <UserRoleIcon role=role is_history=UserRoleIconType::Current />
                    }
                }).collect::<Vec<_>>().into_view()
            }
            </div>
        </div>
    }
}

enum UserRoleIconType {
    History,
    Current,
    Additional,
}

#[component]
fn UserRoleIcon(role: Role, is_history: UserRoleIconType) -> impl IntoView {
    WEREWOLF_ROLES
        .iter()
        .find(|r| r.get_role() == role)
        .map(|role_info| {
            view! {
                <div
                    class=move || match is_history {
                        UserRoleIconType::History => "text-xs opacity-80 w-4 h-4",
                        UserRoleIconType::Current => "text-xs rounded-md bg-white w-4 h-4",
                        UserRoleIconType::Additional => "text-xs w-4 h-4",
                    }
                >
                    {role_info.get_role_icon()}
                </div>
            }
        })
}

#[component]
fn UserRoleName(role: Role) -> impl IntoView {
    WEREWOLF_ROLES
        .iter()
        .find(|r| r.get_role() == role)
        .map_or_else(
            move || {
                view! {
                    <div
                        class="text-xs opacity-20"
                    >
                        "Не выбрано"
                    </div>
                }
            },
            move |role_info| {
                view! {
                    <div
                        class=move || format!("text-xs opacity-50 {}", role_info.get_role_name_color())
                    >
                        {role_info.get_role_name()}
                    </div>
                }
            },
        )
        .into_view()
}

#[component]
fn UserAdditionalRoles(roles: HashSet<Role>) -> impl IntoView {
    view! {
        <div class="flex">
            {
                roles.iter().map(|role| {
                    view!{
                        <UserRoleIcon role=*role is_history=UserRoleIconType::Additional />
                    }
                }).collect::<Vec<_>>().into_view()
            }
        </div>
    }
}

#[component]
fn TurnButtons<'a>(role_info: &'a RoleInfo) -> impl IntoView {
    let active_werewolf_roles =
        use_context::<ReadSignal<WerewolfActiveRoles>>().expect("WerewolfActiveRoles not found");

    let role = role_info.get_role();
    let onclick_next_role = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>()
            .expect("MafiaContext history not found");

        set_context_history.update(|history| history.push(game_ctx.get_history()));

        match get_next_prepare_role(active_werewolf_roles.get().roles, role) {
            Some(role_info) => {
                game_ctx
                    .game_state
                    .set(GameState::Werewolf(WerewolfGameState::SetupRoles(
                        role_info,
                    )))
            }
            None => {
                game_ctx.users.update(|users| {
                    initialize_user_roles(users);
                });
                game_ctx.round.set(0);
                game_ctx
                    .game_state
                    .set(GameState::Werewolf(WerewolfGameState::Day));
            }
        }
    };

    let onclick_prev_role = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>()
            .expect("MafiaContext history not found");

        set_context_history.update(|history| {
            if let Some(prev_ctx) = history.pop() {
                game_ctx.set_history(prev_ctx);
            }
        });
    };

    view! {
        <div class="flex gap-2 w-full items-center">
            <button
                class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click=onclick_prev_role
            >
                "Назад"
            </button>
            <button
                class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click=onclick_next_role
            >
                "Далее"
            </button>
        </div>
    }
}

#[component]
fn SelectUsersForVote(
    selected_users: ReadSignal<HashSet<String>>,
    set_selected_users: WriteSignal<HashSet<String>>,
    is_killed: impl Fn(&Player) -> bool + 'static,
    is_disabled: impl Fn(&Player) -> bool + 'static,
    is_highlighted: impl Fn(&Player) -> bool + 'static,
) -> impl IntoView {
    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    let users = move || mafia_context.users.get();
    let users_alive_len = move || users().iter().filter(|u| u.is_alive).count();
    let is_selected = move |user: &Player| selected_users.get().contains(&user.name);

    view! {
        <div class="text-sm">"Осталось игроков: "{users_alive_len()}</div>
        <div class="grid grid-cols-3 gap-1">
            <For
                each=users
                key=|user| user.name.clone()
                children=move |user| {
                    let disabled = is_disabled(&user);
                    let highlighted = is_highlighted(&user);
                    let killed = is_killed(&user);

                    view!{
                        <UserSelectRole
                            user=user.clone()
                            disabled=disabled
                            highlighted=highlighted
                            killed=killed
                            is_selected=is_selected
                            on:click=move |_| {
                                set_selected_users.update(|selected_users| {
                                    if selected_users.contains(&user.name) {
                                        selected_users.remove(&user.name);
                                    } else {
                                        selected_users.insert(user.name.clone());
                                    }
                                });
                            }
                        />
                    }
                }
            />
        </div>
    }
}

#[component]
fn DayVote() -> impl IntoView {
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());

    let onclick_next_role = move || {
        let selected_users = selected_users.get();
        let round = game_ctx.round.get();

        game_ctx.users.update(|users| {
            clear_choosed_by(users, round);
            clear_was_killed(users);

            fn kill_user(user: &mut Player, round: usize) {
                if !user.is_alive {
                    return;
                }

                if user
                    .additional_role
                    .contains(&Role::Werewolf(WerewolfRole::ToughGuy))
                {
                    user.additional_role
                        .remove(&Role::Werewolf(WerewolfRole::ToughGuy));
                    return;
                }

                user.choosed_by
                    .insert(Role::Werewolf(WerewolfRole::Villager));
                user.is_alive = false;
                user.was_killed = true;
            }

            users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.name) {
                    kill_user(u, round);
                }
            });

            calculate_after_kills(users);
            clear_choosed_by(users, round + 1);
        });

        let users = game_ctx.users.get();
        let mut next_role = Some(WEREWOLF_ROLES.first().unwrap());
        loop {
            if let Some(check_role) = next_role {
                if is_role_alive(check_role.get_role(), &users) {
                    game_ctx
                        .game_state
                        .set(GameState::Werewolf(WerewolfGameState::Night(check_role)));
                    break;
                }
                next_role = get_next_night_role(check_role);
            } else {
                game_ctx.game_state.set(GameState::SetupNames);
                break;
            }
        }
    };

    view! {
        <h2>"Кого мирные жители убъют этим Днем?"</h2>
        <p class="opacity-50 text-sm">"Если убили Hunter или Mad Bomber, выберите несколько целей"</p>
        <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
            <div class="flex-1"></div>
            <div class="flex flex-col gap-1 w-full">
                <SelectUsersForVote selected_users set_selected_users
                    is_killed=move |user: &Player| !user.is_alive && !user.was_killed
                    is_disabled=move |user| !user.is_alive
                    is_highlighted=move |_| false
                />
            </div>
        </div>
        <NextTurnButtons onclick_next_role />
    }
}

#[component]
fn NextTurnButtons<F>(onclick_next_role: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let onclick_prev_role = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>()
            .expect("MafiaContext history not found");

        set_context_history.update(|history| {
            if let Some(prev_ctx) = history.pop() {
                game_ctx.set_history(prev_ctx);
            }
        });
    };

    let onclick_next = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>()
            .expect("MafiaContext history not found");

        set_context_history.update(|history| history.push(game_ctx.get_history()));

        onclick_next_role();
    };

    view! {
        <div class="flex gap-2 w-full items-center">
            <button
                class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click=onclick_prev_role
            >
                "Назад"
            </button>
            <button
                class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click=onclick_next
            >
                "Далее"
            </button>
        </div>
    }
}

fn is_role_alive(role: Role, users: &[Player]) -> bool {
    users.iter().any(|u| u.role.contains(&role) && u.is_alive)
}

fn get_next_night_alive_role(role_info: &RoleInfo, users: &[Player]) -> Option<&'static RoleInfo> {
    let mut next_role = role_info;
    loop {
        let check_role = get_next_night_role(next_role);
        if let Some(check_role) = check_role {
            if is_role_alive(check_role.get_check_role(), users) {
                return Some(check_role);
            }
            next_role = check_role;
        } else {
            return None;
        }
    }
}

fn clear_choosed_by(users: &mut [Player], round: usize) {
    for user in users.iter_mut() {
        if !user.choosed_by.is_empty() {
            user.history_by.push((round, user.choosed_by.clone()));
        }
        user.choosed_by.clear();
    }

    for user in users.iter_mut() {
        // clear spellcaster silence
        user.additional_role
            .remove(&Role::Werewolf(WerewolfRole::Spellcaster));
    }
}

fn clear_was_killed(users: &mut [Player]) {
    for user in users.iter_mut() {
        user.was_killed = false;
    }
}

fn calculate_night_kills(users: &mut [Player]) {
    clear_was_killed(users);
    // Mafia killed choosed user if he is not protected by doctor or prostitute
    let mut alive_users = users.iter_mut().filter(|u| u.is_alive).collect::<Vec<_>>();

    fn is_user_protected(user: &Player, check_protection: &[Role]) -> bool {
        for role in check_protection {
            if user.choosed_by.contains(role) {
                return true;
            }
        }
        return false;
    }

    fn kill_user(user: &mut Player, check_protection: &[Role]) {
        if !user.is_alive {
            return;
        }

        for role in check_protection {
            if user.additional_role.contains(role) {
                user.additional_role.remove(role);
                return;
            }
        }

        user.is_alive = false;
        user.was_killed = true;
    }

    let mut kill_roles = HashSet::<Role>::new();

    for user in alive_users.iter_mut() {
        // Priest check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Priest))
        {
            if user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf)) {
                if !is_user_protected(user, &[Role::Werewolf(WerewolfRole::Bodyguard)]) {
                    kill_user(user, &[Role::Werewolf(WerewolfRole::ToughGuy)]);
                }
            } else {
                user.additional_role
                    .insert(Role::Werewolf(WerewolfRole::Priest));
            }
        }

        // Werewolf check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Werewolf))
        {
            if !is_user_protected(
                user,
                &[
                    Role::Werewolf(WerewolfRole::Bodyguard),
                    Role::Werewolf(WerewolfRole::WitchHeal),
                ],
            ) {
                if user.role.contains(&Role::Werewolf(WerewolfRole::Cursed)) {
                    user.role.insert(Role::Werewolf(WerewolfRole::Werewolf));
                } else {
                    kill_user(
                        user,
                        &[
                            Role::Werewolf(WerewolfRole::Priest),
                            Role::Werewolf(WerewolfRole::ToughGuy),
                        ],
                    );
                }
            }
        }

        // Witch check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::WitchPoison))
        {
            if !is_user_protected(user, &[]) {
                kill_user(
                    user,
                    &[
                        Role::Werewolf(WerewolfRole::ToughGuy),
                        Role::Werewolf(WerewolfRole::Priest),
                    ],
                );
            }
        }

        // Huntress check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Huntress))
        {
            if !is_user_protected(user, &[Role::Werewolf(WerewolfRole::Bodyguard)]) {
                kill_user(
                    user,
                    &[
                        Role::Werewolf(WerewolfRole::ToughGuy),
                        Role::Werewolf(WerewolfRole::Priest),
                    ],
                );
            }
        }

        // Revealer check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Revealer))
        {
            if user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf)) {
                if !is_user_protected(user, &[Role::Werewolf(WerewolfRole::Bodyguard)]) {
                    kill_user(
                        user,
                        &[
                            Role::Werewolf(WerewolfRole::ToughGuy),
                            Role::Werewolf(WerewolfRole::Priest),
                        ],
                    );
                }
            } else {
                kill_roles.insert(Role::Werewolf(WerewolfRole::Revealer));
            }
        }

        // Spellcaster check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Spellcaster))
        {
            user.additional_role
                .insert(Role::Werewolf(WerewolfRole::Spellcaster));
        }
    }

    // kill rest roles
    for kill_role in kill_roles {
        for user in alive_users.iter_mut() {
            if user.role.contains(&kill_role) {
                kill_user(user, &[]);
            }
        }
    }
}

fn initialize_user_roles(users: &mut [Player]) {
    // init additional roles
}

fn calculate_after_kills(users: &mut [Player]) {
    let mut kill_indices: Vec<usize> = Vec::new();

    for user in users.iter() {
        if user.is_alive {
            continue;
        }

        if user.role.contains(&Role::Werewolf(WerewolfRole::Lovers)) {
            users.iter().enumerate().for_each(|(index, u)| {
                if u.role.contains(&Role::Werewolf(WerewolfRole::Lovers)) {
                    kill_indices.push(index);
                }
            });
        }

        if user
            .additional_role
            .contains(&Role::Werewolf(WerewolfRole::DireWolf))
            && !user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
        {
            users.iter().enumerate().for_each(|(index, u)| {
                if u.additional_role
                    .contains(&Role::Werewolf(WerewolfRole::DireWolf))
                    && u.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                {
                    kill_indices.push(index);
                }
            });
        }
    }

    // Set is_alive to false for all Lovers
    for index in kill_indices {
        if let Some(u) = users.get_mut(index) {
            if u.is_alive {
                u.is_alive = false;
                u.was_killed = true;
                u.choosed_by.insert(Role::Werewolf(WerewolfRole::Villager));
            }
        }
    }
}

// check is user history contains role
fn check_user_history_for_role(user: &Player, role: &Role) -> bool {
    user.history_by
        .iter()
        .any(|(_, roles)| roles.contains(role))
}

#[component]
fn NightTurn(role_info: &'static RoleInfo) -> impl IntoView {
    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
    let night_description = role_info.get_night_description();

    let users = move || mafia_context.users.get();

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());
    let role = role_info.get_role();

    let onclick_next_role = move || {
        let selected_users = selected_users.get();
        game_ctx.users.update(|users| {
            users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.name) {
                    u.choosed_by.insert(role);
                }
            })
        });

        let users = game_ctx.users.get();
        let next_role = get_next_night_alive_role(role_info, &users);

        match next_role {
            Some(next_role) => {
                game_ctx
                    .game_state
                    .set(GameState::Werewolf(WerewolfGameState::Night(next_role)));
            }
            None => {
                game_ctx.users.update(|users| {
                    calculate_night_kills(users);
                    calculate_after_kills(users);
                });
                game_ctx.round.set(game_ctx.round.get() + 1);
                game_ctx
                    .game_state
                    .set(GameState::Werewolf(WerewolfGameState::Day));
            }
        }
    };

    let role_targeting_rules = role_info.get_targeting_rules();
    let is_fully_disabled = match role_targeting_rules {
        NightTargetingRules::OnlyOne => users()
            .iter()
            .any(|u| check_user_history_for_role(u, &role)),
        _ => false,
    };
    let is_disabled = move |user: &Player| {
        is_fully_disabled
            || !user.is_alive
            || match role_targeting_rules {
                NightTargetingRules::NotTheSame => check_user_history_for_role(&user, &role),
                _ => false,
            }
    };

    let role_info = role_info.clone();
    let is_highlighted = move |user: &Player| {
        if role_info.get_role() == Role::Werewolf(WerewolfRole::Seer) {
            user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                || user.role.contains(&Role::Werewolf(WerewolfRole::Lycan))
        } else {
            false
        }
    };

    view! {
        <h2>
            {night_description}
        </h2>
        <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
            <div class="flex-1"></div>
            <div class="flex flex-col gap-1 w-full">
            <SelectUsersForVote
                is_killed=move |user: &Player| !user.is_alive && !user.was_killed
                selected_users set_selected_users is_disabled is_highlighted />
            </div>
        </div>
        <NextTurnButtons onclick_next_role />
    }
}
