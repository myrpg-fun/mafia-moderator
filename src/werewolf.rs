use itertools::Itertools;
use leptos_use::utils::*;
use leptos_use::*;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use crate::roles::*;
use crate::rust_create_new_game_log;
use crate::user::*;
use crate::GameContext;
use crate::GameContextHistory;
use crate::GameState;
use crate::UserLogs;
use leptos::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WerewolfRole {
    None,
    Villager,
    Drunk,
    Werewolf,
    Minion,
    DireWolf,
    // LoneWolf,
    WolfCub,
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
    Ghost,
    ParanormalInvestigator,
    Prince,
    Diseased,
    Mason,
    Lovers,
    Doppelganger,
    Mentalist,
    // AlphaWolf,
    MadBomber,
    //*** TODO:
    Revealer,
    VillageIdiot,
    Pacifist,
    // ApprenticeSeer,
    // AuraSeer,
    OldHag,
    TroubleMaker,
    Tanner,
    Vampire,
}

const _WEREWOLF_COLORS: [&str; 13] = [
    "text-red-950",
    "text-blue-950",
    "text-gray-950",
    "text-green-950",
    "text-purple-950",
    "bg-rose-950/50",
    "bg-red-950/50",
    "bg-blue-950/50",
    "bg-gray-950/50",
    "bg-green-950/50",
    "bg-purple-950/50",
    "ring-red-600/50",
    "ring-blue-600/50",
];

pub const WEREWOLF_ROLES: [RoleInfo; 32] = [
    RoleInfo::Icon(IconRoleInfo {
        role: Role::WasKilled,
        role_name: "Killed",
        role_name_color: "red-950",
        role_icon: "❌",
    }),
    RoleInfo::Icon(IconRoleInfo {
        role: Role::Werewolf(WerewolfRole::Villager),
        role_name: "Villagers",
        role_name_color: "blue-950",
        role_icon: "✋",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Drunk),
        role_name: "Drunk",
        role_icon: "👺",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Drunk и посмотрите его роль",
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
        role: Role::Werewolf(WerewolfRole::Vampire),
        check_role: None,
        role_icon: "🧛",
        role_name: "Vampire",
        role_name_color: "purple-950",
        prepare_description: "Выберите игрока Vampire",
        night_description: "Кого укусят Vampire?",
        targeting_rules: NightTargetingRules::Anyone,
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
        role_icon: "👺",
        additional_role: None,
        role_name_color: "red-950",
        prepare_description: "Выберите игрока Minion",
    }),
    RoleInfo::Additional(AdditionalRoleInfo {
        role: Role::Werewolf(WerewolfRole::DireWolf),
        role_name: "Lone Wolf",
        role_name_color: "red-950",
        role_icon: "💙",
        prepare_description: "Поставте сердечки игроку Lone Wolf и в кого он влюблен",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Cursed),
        role_name: "Cursed",
        role_icon: "😈",
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
        role_name_color: "green-950",
        role_icon: "🔍",
        prepare_description: "Выберите игрока Seer",
        night_description: "Кого проверит Seer?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Mentalist),
        check_role: None,
        role_name: "Mentalist",
        role_name_color: "green-950",
        role_icon: "👁️",
        prepare_description: "Выберите игрока Mentalist",
        night_description: "Выберите двух игроков, кого проверил Mentalist?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::ParanormalInvestigator),
        check_role: None,
        role_name: "P.I.",
        role_name_color: "green-950",
        role_icon: "📸",
        prepare_description: "Выберите игрока Paranormal Investigator",
        night_description: "Выберите 3х соседних игроков, кого проверил Paranormal Investigator?",
        targeting_rules: NightTargetingRules::OnlyOne,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Spellcaster),
        check_role: None,
        role_name: "Spellcaster",
        role_name_color: "green-950",
        role_icon: "🤐",
        prepare_description: "Выберите игрока Spellcaster",
        night_description: "Кого заглушил Spellcaster?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Huntress),
        check_role: None,
        role_name: "Huntress",
        role_name_color: "green-950",
        role_icon: "🏹",
        prepare_description: "Выберите игрока Huntress",
        night_description: "Кого убъет Huntress?",
        targeting_rules: NightTargetingRules::OnlyOne,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Revealer),
        check_role: None,
        role_name: "Revealer",
        role_name_color: "green-950",
        role_icon: "🔦",
        prepare_description: "Выберите игрока Revealer",
        night_description: "Кого проверит Revealer?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Lovers),
        role_name: "Mason",
        role_icon: "❤️",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игроков Mason (Lovers)",
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
        role_icon: "🌓",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Lycan",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Ghost),
        role_name: "Ghost",
        role_icon: "👻",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Ghost",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Prince),
        role_name: "Prince",
        role_icon: "👑",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Prince",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Diseased),
        role_name: "Diseased",
        role_icon: "🦠",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Diseased",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::VillageIdiot),
        role_name: "Village Idiot",
        role_icon: "🤪",
        additional_role: Some(Role::Werewolf(WerewolfRole::VillageIdiot)),
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Village Idiot",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Pacifist),
        role_name: "Pacifist",
        role_icon: "🕊️",
        additional_role: Some(Role::Werewolf(WerewolfRole::Pacifist)),
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Pacifist",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Hunter),
        role_name: "Hunter",
        role_icon: "🎯",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Hunter",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::TroubleMaker),
        role_name: "TroubleMaker",
        role_icon: "🔥",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока TroubleMaker",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::OldHag),
        role_name: "Old Hag",
        role_icon: "👵",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Old Hag",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::MadBomber),
        role_name: "Mad Bomber",
        role_icon: "💣",
        additional_role: None,
        role_name_color: "blue-950",
        prepare_description: "Выберите игрока Mad Bomber",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Tanner),
        role_name: "Tanner",
        role_icon: "🧵",
        additional_role: None,
        role_name_color: "gray-950",
        prepare_description: "Выберите игрока Tanner",
    }),
];

#[derive(Clone, Debug, PartialEq)]
enum WerewolfHint {
    Mayor(Player),
    Cursed(Player),
    LostHeart(Player),
    Spellcaster(Player),
    Killed(Player, HashSet<Role>),
    Seer(Vec<Player>),
    //Vampire(Vec<Player>),
}

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

#[derive(Debug, Clone)]
struct OpenFinishGameDialogue(bool);

#[component]
pub fn WerewolfGameView() -> impl IntoView {
    let open_finish_game_dialogue = create_rw_signal(OpenFinishGameDialogue(false));
    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    let (active_werewolf_roles, set_active_werewolf_roles) =
        create_signal::<WerewolfActiveRoles>(WerewolfActiveRoles::default());

    provide_context(active_werewolf_roles);
    provide_context(open_finish_game_dialogue);

    let game_state_view = move || {
        if open_finish_game_dialogue.get().0 {
            return view! {
                <SelectWinners
                    on_close=move || open_finish_game_dialogue.set(OpenFinishGameDialogue(false))
                    on_finish=move || mafia_context.game_state.set(GameState::SetupNames)
                />
            }
            .into_view();
        }

        match mafia_context.game_state.get() {
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
        }
    };

    view! {
        <div class="relative flex flex-col gap-4 w-full h-full">
            <h1 class="text-lg relative w-full text-left">
                "Werewolf"
                {move || if open_finish_game_dialogue.get().0 {
                    view!{
                        <button
                            class="absolute text-sm right-0 top-0 px-2 py-1 bg-gray-200 rounded-full"
                            on:click=move |_| {
                                open_finish_game_dialogue.set(OpenFinishGameDialogue(false));
                            }>
                            "Отмена"
                        </button>
                        }
                    }else{
                        view!{
                        <button
                            class="absolute text-sm right-0 top-0 px-2 py-1 bg-gray-200 rounded-full"
                            on:click=move |_| {
                                open_finish_game_dialogue.set(OpenFinishGameDialogue(true));
                            }>
                            "Завершить игру"
                        </button>
                        }
                    }
                }
            </h1>
            {game_state_view}
        </div>
    }
}

fn calculate_user_logs(
    users: Vec<Player>,
    best_players: HashSet<String>,
    selected_winners: HashSet<Role>,
) -> Vec<UserLogs> {
    let mut logs = Vec::new();
    let mut last_round = 0;

    // Calculate last round safely
    for user in users.iter() {
        for (index, _) in &user.history_by {
            last_round = last_round.max(*index);
        }
    }

    // Prepare user history with fallback for missing data
    let mut user_history = HashMap::new();
    for user in users.iter() {
        let mut rounds = user.history_by.clone();
        rounds.push((last_round + 1, user.choosed_by.clone()));

        user_history.insert(user.id.clone(), rounds);
    }

    // Process each user with error resilience
    let empty_history = vec![(0, HashSet::new())];
    for user in users.iter() {
        let mut rounds = vec!["".to_string(); last_round + 1];

        let current_user_history = user_history.get(&user.id).unwrap_or_else(|| &empty_history);

        for (index, roles) in current_user_history {
            // Safe index calculation
            let adjusted_index = index.saturating_sub(1).clamp(0, rounds.len() - 1);

            // Safe role processing with default icon
            let role_icons = roles
                .iter()
                //sort WasKilled role
                .sorted_by(|a, b| {
                    if **a == Role::WasKilled {
                        Ordering::Greater
                    } else if **b == Role::WasKilled {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                })
                .map(|role| {
                    WEREWOLF_ROLES
                        .iter()
                        .find(|r| r.get_role() == *role)
                        .map(|r| r.get_role_icon())
                        .unwrap_or("❓") // Fallback icon
                })
                .collect::<Vec<_>>()
                .join(" ");

            rounds[adjusted_index] = role_icons;
        }

        // Winner calculation without implicit assumptions
        let winner = user.role.iter().any(|role| selected_winners.contains(role))
            || (selected_winners.contains(&Role::Werewolf(WerewolfRole::Villager))
                && user.role.is_empty());

        // Safe role processing with fallbacks
        let role_str = if user.role.is_empty() {
            "Мирный".to_string()
        } else {
            user.role
                .iter()
                .filter_map(|role| {
                    WEREWOLF_ROLES
                        .iter()
                        .find(|r| r.get_role() == *role)
                        .map(|r| r.get_role_name())
                })
                .collect::<Vec<_>>()
                .join(" • ")
        };

        let role_index = if user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf)) {
            "Werewolf"
        } else if user.role.contains(&Role::Werewolf(WerewolfRole::Minion)) {
            "Werewolf"
        } else if user.role.contains(&Role::Werewolf(WerewolfRole::Tanner)) {
            "Tanner"
        } else if user.role.contains(&Role::Werewolf(WerewolfRole::Vampire)) {
            "Vampire"
        } else {
            "Villager"
        }
        .to_string();

        let role_score = if winner { 1 } else { 0 };

        let best_player = best_players.contains(&user.id);

        logs.push(UserLogs {
            id: user.id.clone(),
            name: user.name.clone(),
            is_guest: user.is_guest,
            role: role_str,
            role_index,
            role_score,
            best_player,
            score: 0,
            winner,
            rounds,
        });
    }

    logs
}

#[component]
fn SelectWinners(
    on_close: impl Fn() -> () + Clone + 'static,
    on_finish: impl Fn() -> () + Clone + 'static,
) -> impl IntoView {
    let selected_winners = create_rw_signal(HashSet::<Role>::new());
    let selected_users = create_rw_signal(HashSet::<String>::new());

    let roles = [
        RoleInfo::Icon(IconRoleInfo {
            role: Role::Werewolf(WerewolfRole::Tanner),
            role_name: "Таннер",
            role_name_color: "gray-950",
            role_icon: "🧵",
        }),
        RoleInfo::Icon(IconRoleInfo {
            role: Role::Werewolf(WerewolfRole::Mason),
            role_name: "Масоны (Любовники)",
            role_name_color: "blue-950",
            role_icon: "❤️",
        }),
        RoleInfo::Icon(IconRoleInfo {
            role: Role::Werewolf(WerewolfRole::Vampire),
            role_name: "Вампиры",
            role_name_color: "purple-950",
            role_icon: "🩸",
        }),
        RoleInfo::Icon(IconRoleInfo {
            role: Role::Werewolf(WerewolfRole::Werewolf),
            role_name: "Оборотни",
            role_name_color: "red-950",
            role_icon: "🔫",
        }),
        RoleInfo::Icon(IconRoleInfo {
            role: Role::Werewolf(WerewolfRole::Villager),
            role_name: "Жители деревни",
            role_name_color: "green-950",
            role_icon: "✋",
        }),
    ];

    let is_selected = move |role: &Role| selected_winners.get().contains(role);

    let calculate_user_logs = move || {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let users = game_ctx.users.get();
        let best_players = selected_users.get();
        let selected_winners = selected_winners.get();

        calculate_user_logs(users, best_players, selected_winners)
    };

    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    let users = move || mafia_context.users.get();

    view! {
        <div class="flex-1 flex flex-col gap-1 relative overflow-auto px-4 -mx-4">
            <h2>"⭐ Выберите лучших игроков"</h2>
            <div class="grid grid-cols-3 gap-1">
                <For
                    each=users
                    key=|user| format!("{}_{}", user.id.clone(), user.role.len())
                    children=move |user| {
                        let user_clone = user.clone();

                        view!{
                            <UserSelectRole
                                user=user_clone
                                disabled=false
                                is_selected=move |u| selected_users.get().contains(&u.id)
                                highlighted=false
                                killed=false
                                on:click=move |_| {
                                    selected_users.update(|users| {
                                        if users.contains(&user.id) {
                                            users.remove(&user.id);
                                        } else {
                                            users.insert(user.id.clone());
                                        }
                                    });
                                }
                            />
                        }
                    }
                />
            </div>
        </div>
        <div class="flex flex-col gap-1 relative px-4 -mx-4">
            <h2>"🏆 Выберите кто победил"</h2>
            <button class=move ||
                format!("rounded-xl px-3 py-2 text-sm {}", if selected_winners.get().is_empty() {
                    "text-white bg-red-800/80".to_string()
                } else {
                    "bg-gray-200".to_string()
                })
                on:click=move|_|{
                    selected_winners.update(|selected_winners|{
                        selected_winners.clear();
                    });
                }
            >
                "Не сохранять результаты"
            </button>
            <div class="grid grid-cols-3 gap-1 justify-stretch w-full">
            <div></div>
            {roles.map(|role| {
                let role_clone = role.get_role().clone();
                view!{
                    <button class=move ||
                        format!("flex-1 rounded-xl px-3 py-5 text-sm {}", if is_selected(
                            &role.get_role()
                        ) {
                            format!("text-white {}", role.get_role_bg_color())
                        } else {
                            "bg-gray-200".to_string()
                        })
                        on:click=move|_|{
                            selected_winners.update(|selected_winners|{
                                if selected_winners.contains(&role_clone) {
                                    selected_winners.remove(&role_clone);
                                }else{
                                    selected_winners.insert(role_clone);
                                }
                            });
                        }
                    >
                        {role.get_role_name()}
                    </button>
                }
            })}
            </div>
        </div>
        <div class="flex gap-2 w-full items-center">
            <button
                class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click={
                    move |_| on_close()
                }
            >
                "Назад"
            </button>
            <button
                class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click={
                    move |_| {
                        if selected_winners.get().is_empty() {
                            if window().confirm_with_message("Вернуться в главное меню без победителей?").expect("REASON") {
                                on_finish();
                            }
                        }else{
                            rust_create_new_game_log(calculate_user_logs(), false);

                            on_finish();
                        }
                    }
                }
            >
                "Закончить игру"
            </button>
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
        <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
            <div class="flex-1"></div>
            <button class=move || format!("rounded-2xl px-3 py-2 text-sm bg-gray-200 mb-3")
                on:click=move|_|{
                    set_active_werewolf_roles.update(|active_werewolf_roles|{
                        active_werewolf_roles.roles.clear();
                    });
                }
            >
                "Очистить роли"
            </button>
            <div class="grid grid-cols-3 gap-1">
                {roles.map(|role| {
                    let role_clone = role.get_role().clone();
                    view!{
                        <button class=move ||
                            format!("text-left rounded-2xl px-3 py-2 text-sm {}", if is_selected(
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
        </div>
        <div class="flex gap-2 w-full items-center">
            <button
                class="w-9 px-2 py-2 text-sm bg-gray-200 rounded-full flex items-center justify-center"
                on:click=onclick_prev_role
            >
                "←"
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

    let role = role_info.get_role();
    let role_info = role_info.clone();
    let users_sorted = move || users_sorted(mafia_context.users.get());

    view! {
        <div class="grid grid-cols-2 gap-y-1 gap-x-3">
            <For
                each=users_sorted
                key=|user| format!("{}-{}-{}-{}", user.id, user.role.len(), user.additional_role.len(), user.choosed_by.len())
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

fn user_background_role_color(user: &Player) -> &str {
    if user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf)) {
        "bg-red-100"
    } else if user.role.contains(&Role::Werewolf(WerewolfRole::Cursed)) {
        "bg-fuchsia-100"
    } else if user.role.contains(&Role::Werewolf(WerewolfRole::Vampire)) {
        "bg-violet-100"
    } else if user.role.contains(&Role::Werewolf(WerewolfRole::Seer))
        || user.role.contains(&Role::Werewolf(WerewolfRole::Mentalist))
        || user
            .role
            .contains(&Role::Werewolf(WerewolfRole::ParanormalInvestigator))
    {
        "bg-cyan-100"
    } else if user.role.contains(&Role::Werewolf(WerewolfRole::Bodyguard))
        || user.role.contains(&Role::Werewolf(WerewolfRole::WitchHeal))
        || user.role.contains(&Role::Werewolf(WerewolfRole::Priest))
    {
        "bg-emerald-100"
    } else if user.role.contains(&Role::Werewolf(WerewolfRole::Tanner)) {
        "bg-orange-100"
    } else {
        "bg-gray-100"
    }
}

#[component]
fn UserSelectRole(
    user: Player,
    is_selected: impl Fn(&Player) -> bool + 'static,
    disabled: bool,
    killed: bool,
    highlighted: bool,
    #[prop(default = "ring-red-600/50".to_string())] highlight_color: String,
) -> impl IntoView {
    let history = user.history_by.clone();
    let choosed = user.choosed_by.clone();
    let user_c1 = user.clone();

    view! {
        <button
            disabled=disabled
            class=move || {
                let is_selected = is_selected(&user_c1);

                format!("relative overflow-hidden
                    flex-1 px-3 py-1 text-sm rounded-2xl 
                    flex gap-1.5 items-center justify-center 
                    min-h-8 {} {}", 
                if killed {
                    "opacity-20 bg-white hover:opacity-90"
                } else if disabled {
                    "opacity-60 bg-gray-100 hover:opacity-90"
                } else if is_selected {
                    "bg-blue-300"
                } else {
                    user_background_role_color(&user_c1)
                },
                if highlighted {
                    format!("ring-1 {}", highlight_color)
                } else {
                    "".to_string()
                })}
        >
            <div class="flex-grow">
                <div class="text-left">{user.name} <UserAdditionalRoles roles=user.additional_role /></div>
                <UserRoleNames role=user.role />
            </div>
            <UserHistory hystory=history current=choosed />
            {move || if user.was_killed && !user.is_alive {
                view! {
                    <div class="text-[0.5rem]">"❌"</div>
                }.into_view()
            }else if user.was_killed && user.is_alive {
                view! {
                    <div class="relative text-[0.5rem]">"💛"
                        <div class="absolute text-[0.3rem] left-[0.15rem] top-[0.04rem]">"❌"</div>
                    </div>
                }.into_view()
            }else{
                "".into_view()
            }}
        </button>
    }
}

#[component]
fn UserRoleNames(role: HashSet<Role>) -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-x-1 items-start justify-start">
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
fn UserKilledBy(killed_by: HashSet<Role>) -> impl IntoView {
    view! {
        {move || {
            let killed_by = killed_by.clone().into_iter().filter(|role| {
                // werewolf or witch or revealer or hunteress
                matches!(
                    role,
                    Role::Werewolf(WerewolfRole::Werewolf)
                        | Role::Werewolf(WerewolfRole::WitchPoison)
                        | Role::Werewolf(WerewolfRole::Revealer)
                        | Role::Werewolf(WerewolfRole::Huntress)
                )
            }).collect::<Vec<_>>();

            if killed_by.is_empty() {
                view!{
                    ""
                }.into_view()
            }else{
                view!{
                    " (by "
                    {move || killed_by.iter().map(|role| {
                        let role = *role;

                        view!{
                            <UserRoleName role=role />
                        }.into_view()
                    }).intersperse_with(|| {
                        // Add separator between the roles
                        view! { ", " }.into_view() // Assuming there's a Separator component; alternatively, use raw HTML
                    })
                    .collect::<Vec<_>>().into_view()}
                    ")"
                }.into_view()
            }
        }}
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
        <div class="flex gap-0.5 flex-wrap min-h-4">
            {
                current.iter().map(|role| {
                    let role = *role;

                    if role != Role::WasKilled{
                        view!{
                            <UserRoleIcon role=role is_history=UserRoleIconType::Current />
                        }
                    }else{
                        "".into_view()
                    }
                }).collect::<Vec<_>>().into_view()
            }
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
        .map(|role_info| match is_history {
            UserRoleIconType::Additional => view! {
                " "{role_info.get_role_icon()}
            }
            .into_view(),
            _ => view! {
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
            .into_view(),
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
        {
            roles.iter().map(|role| {
                view!{
                    <UserRoleIcon role=*role is_history=UserRoleIconType::Additional />
                }
            }).collect::<Vec<_>>().into_view()
        }
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
                class="w-9 px-2 py-2 text-sm bg-gray-200 rounded-full flex items-center justify-center"
                on:click=onclick_prev_role
            >
                "←"
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

fn users_sorted(users: Vec<Player>) -> Vec<Player> {
    // Clone and sort the users by a desired attribute if needed. Here, sorting by ID as an example.
    let len = users.len();
    let mut rearranged_users = Vec::with_capacity(len);

    let is_odd = len % 2 != 0;
    // If the length is odd, add the last middle element.
    if is_odd {
        rearranged_users.push(users[len / 2].clone());
    }

    for i in 0..len / 2 {
        let i = len / 2 - i - 1;
        // Add the i-th from the end and the i-th from the start in pairs.
        if is_odd {
            rearranged_users.push(users[len - i - 1].clone());
            rearranged_users.push(users[i].clone());
        } else {
            rearranged_users.push(users[i].clone());
            rearranged_users.push(users[len - i - 1].clone());
        }
    }

    rearranged_users
}

#[component]
fn SelectUsersForVote(
    selected_users: ReadSignal<HashSet<String>>,
    set_selected_users: WriteSignal<HashSet<String>>,
    is_killed: impl Fn(&Player) -> bool + 'static,
    is_disabled: impl Fn(&Player) -> bool + 'static,
    is_highlighted: impl Fn(&Player) -> bool + 'static,
    #[prop(default = "ring-red-600/50".to_string())] highlight_color: String,
) -> impl IntoView {
    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    let users = move || mafia_context.users.get();
    let users_alive_len = move || users().iter().filter(|u| u.is_alive).count();
    let werewolf_alive_len = move || {
        users()
            .iter()
            .filter(|u| u.is_alive && u.role.contains(&Role::Werewolf(WerewolfRole::Werewolf)))
            .count()
    };
    let vampire_alive_len = move || {
        users()
            .iter()
            .filter(|u| u.is_alive && u.role.contains(&Role::Werewolf(WerewolfRole::Vampire)))
            .count()
    };
    let is_selected = move |user: &Player| selected_users.get().contains(&user.id);
    let users_sorted = move || users_sorted(mafia_context.users.get());

    view! {
        <div class="text-sm">"Осталось игроков: "{users_alive_len}", оборотней: "{werewolf_alive_len}", вампиров: "{vampire_alive_len}</div>
        <div class="grid grid-cols-2 gap-y-1 gap-x-3">
            <For
                each=users_sorted
                key=|user| user.id.clone()
                children=move |user| {
                    let disabled = is_disabled(&user);
                    let highlighted = is_highlighted(&user);
                    let killed = is_killed(&user);

                    view!{
                        <UserSelectRole
                            user=user.clone()
                            disabled=disabled
                            highlighted=highlighted
                            highlight_color=highlight_color.clone()
                            killed=killed
                            is_selected=is_selected
                            on:click=move |_| {
                                set_selected_users.update(|selected_users| {
                                    if selected_users.contains(&user.id) {
                                        selected_users.remove(&user.id);
                                    } else {
                                        selected_users.insert(user.id.clone());
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

fn day_kill_user(user: &mut Player, round: usize) {
    if !user.is_alive {
        return;
    }

    if user
        .additional_role
        .contains(&Role::Werewolf(WerewolfRole::Vampire))
    {
        user.choosed_by
            .insert(Role::Werewolf(WerewolfRole::Vampire));
    } else {
        user.choosed_by
            .insert(Role::Werewolf(WerewolfRole::Villager));
    }
    user.was_killed = true;

    if user
        .additional_role
        .contains(&Role::Werewolf(WerewolfRole::ToughGuy))
    {
        user.additional_role
            .remove(&Role::Werewolf(WerewolfRole::ToughGuy));
        return;
    }

    user.choosed_by.insert(Role::WasKilled);
    user.is_alive = false;
}

#[component]
fn DayVote() -> impl IntoView {
    let clock_choose = create_rw_signal(true);
    let kill_player_choose = create_rw_signal(false);
    let start_player_choose = create_rw_signal(false);

    let (highlighted_player, set_highlighted_player) =
        create_signal::<HashSet<String>>(HashSet::new());

    let open_dialogue =
        use_context::<RwSignal<OpenFinishGameDialogue>>().expect("MafiaContext not found");
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());

    let onclick_next_role = move || {
        let selected_users = selected_users.get();
        let round = game_ctx.round.get();

        game_ctx.users.update(|users| {
            clear_choosed_by(users, round);
            clear_was_killed(users);

            users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.id) {
                    day_kill_user(u, round);
                }
            });

            calculate_after_kills(users);
            clear_choosed_by(users, round + 1);
        });
        game_ctx.round.set(round + 2);

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
                open_dialogue.set(OpenFinishGameDialogue(true));
                break;
            }
        }
    };

    let onclick_kill_users = move || {
        let selected_users = selected_users.get();
        let round = game_ctx.round.get();

        game_ctx.users.update(|users| {
            users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.id) {
                    day_kill_user(u, round);
                }
            });

            calculate_after_kills(users);
        });

        kill_player_choose.set(false);
        clock_choose.set(true);
    };

    create_effect(move |_| {
        kill_player_choose.get();
        set_selected_users.update(|selected_users| {
            selected_users.clear();
        });
    });

    let onclick_start_player = move || {
        if !start_player_choose.get() {
            set_highlighted_player.update(|highlighted_player| {
                highlighted_player.clear();
            });
            start_player_choose.set(true);
        } else {
            start_player_choose.set(false);
        }
    };

    create_effect(move |_| {
        if start_player_choose.get() && !highlighted_player.get().is_empty() {
            start_player_choose.set(false);
            clock_choose.set(true);
        }
    });

    let game_log = create_memo(move |_| {
        let mut log = Vec::<WerewolfHint>::new();

        let users = game_ctx.users.get();

        users.iter().for_each(|user| {
            if user.was_killed && !user.is_alive {
                log.push(WerewolfHint::Killed(user.clone(), user.choosed_by.clone()));
            }
        });

        users.iter().for_each(|user| {
            if user.role.contains(&Role::Werewolf(WerewolfRole::ToughGuy))
                && user.is_alive
                && user.was_killed
            {
                log.push(WerewolfHint::LostHeart(user.clone()));
            }
        });

        users.iter().for_each(|user| {
            if user
                .choosed_by
                .contains(&Role::Werewolf(WerewolfRole::Spellcaster))
                && user.is_alive
            {
                log.push(WerewolfHint::Spellcaster(user.clone()));
            }
        });

        users.iter().for_each(|user| {
            if user
                .additional_role
                .contains(&Role::Werewolf(WerewolfRole::Mayor))
                && user.is_alive
            {
                log.push(WerewolfHint::Mayor(user.clone()));
            }
        });

        // let v_users = users
        //     .iter()
        //     .filter(|user| {
        //         return user
        //             .additional_role
        //             .contains(&Role::Werewolf(WerewolfRole::Vampire))
        //             && user.is_alive;
        //     })
        //     .cloned()
        //     .collect::<Vec<_>>();

        // if !v_users.is_empty() {
        //     log.push(WerewolfHint::Vampire(v_users));
        // }

        log
    });

    let is_highlighted = move |user: &Player| highlighted_player.get().contains(&user.id);

    view! {
        {move ||
            if start_player_choose.get() {
                view!{
                    <h2>"Кто начал этот раунд?"</h2>
                    <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
                        <div class="flex-1"></div>
                        <div class="flex flex-col gap-1 w-full pb-0.5">
                            <SelectUsersForVote
                                selected_users=highlighted_player
                                set_selected_users=set_highlighted_player
                                is_killed=move |user: &Player| !user.is_alive && !user.was_killed
                                is_disabled=move |user| !user.is_alive
                                is_highlighted
                                highlight_color="ring-blue-600/80".to_string()
                            />
                        </div>
                    </div>
                    <DayTurnButtons onclick_next_role clock_choose start_player_choose kill_player_choose onclick_start_player />
                }.into_view()
            }else if kill_player_choose.get() {
                view!{
                    <h2>"Выберите убитых в начале дня:"</h2>
                    <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
                        <div class="flex-1"></div>
                        <div class="flex flex-col gap-1 w-full pb-0.5">
                            <SelectUsersForVote
                                selected_users
                                set_selected_users
                                is_killed=move |user: &Player| !user.is_alive && !user.was_killed
                                is_disabled=move |user| !user.is_alive
                                is_highlighted
                                highlight_color="ring-blue-600/80".to_string()
                            />
                        </div>
                    </div>
                    <DayTurnButtons onclick_next_role=onclick_kill_users clock_choose start_player_choose kill_player_choose onclick_start_player />
                }.into_view()
            }else{
                view! {
                    <h2>"Кого мирные жители убъют этим Днем?"</h2>
                    <DisplayLogs logs=game_log />
                    <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
                        <div class="flex-1"></div>
                        <div class="flex flex-col gap-1 w-full pb-0.5">
                            <SelectUsersForVote selected_users set_selected_users
                                is_killed=move |user: &Player| !user.is_alive && !user.was_killed
                                is_disabled=move |user| !user.is_alive
                                is_highlighted
                                highlight_color="ring-blue-600/80".to_string()
                            />
                        </div>
                    </div>
                    <Show when=move || clock_choose.get()>
                        <Timer />
                    </Show>
                    <DayTurnButtons onclick_next_role clock_choose start_player_choose kill_player_choose onclick_start_player />
                }.into_view()
            }
        }
    }
}

#[component]
fn DisplayLogs(logs: Memo<Vec<WerewolfHint>>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1 relative text-xs">
            {move || logs.get().iter().map(
                |log| {
                    match log {
                        WerewolfHint::Killed(user, killed_by) => {
                            let user = user.clone();
                            view!{
                                <div class="w-full flex items-center justify-start gap-1 text-gray-500">
                                    "❌"<span class="bg-gray-100 text-gray-900 px-1 rounded-md">{user.name}</span><UserRoleNames role=user.role />" убит"<UserKilledBy killed_by=killed_by.clone() />"."
                                </div>
                            }.into_view()
                        },
                        WerewolfHint::Mayor(user) => {
                            let user = user.clone();
                            view!{
                                <div class="w-full flex items-center justify-start gap-1 text-gray-500">
                                    "🎖️"<span class="bg-gray-100 text-gray-900 px-1 rounded-md">{user.name}</span>"мэр."
                                </div>
                            }.into_view()
                        },
                        WerewolfHint::Spellcaster(user) => {
                            let user = user.clone();
                            view!{
                                <div class="w-full flex items-center justify-start gap-1 text-gray-500">
                                    "🤐"<span class="bg-gray-100 text-gray-900 px-1 rounded-md">{user.name}</span>"не может говорить."
                                </div>
                            }.into_view()
                        },
                        WerewolfHint::Cursed(user) => {
                            let user = user.clone();
                            view!{
                                <div class="w-full flex items-center justify-start gap-1 text-gray-500">
                                    "😈"<span class="bg-gray-100 text-gray-900 px-1 rounded-md">{user.name}</span>"станет оборотнем если его съедят."
                                </div>
                            }.into_view()
                        },
                        WerewolfHint::LostHeart(user) => {
                            let user = user.clone();
                            view!{
                                <div class="w-full flex items-center justify-start gap-1 text-gray-500">
                                    "💔"<span class="bg-gray-100 text-gray-900 px-1 rounded-md">{user.name}</span>"потерял жизнь."
                                </div>
                            }.into_view()
                        }
                        WerewolfHint::Seer(users) => {
                            view!{
                                <div class="w-full flex-wrap flex items-center justify-start gap-1.5 text-gray-500">
                                    "🔍"{users.iter().map(|user| {
                                        let user = user.clone();
                                        view!{
                                            <span class="bg-gray-100 text-gray-900 px-1 rounded-md whitespace-nowrap">{user.name}</span>
                                        }.into_view()
                                    }).collect::<Vec<_>>().into_view()}"оборотни."
                                </div>
                            }.into_view()
                        }
                        // WerewolfHint::Vampire(users) => {
                        //     view!{
                        //         <div class="w-full flex-wrap flex items-center justify-start gap-1.5 text-gray-500">
                        //             "🩸"{users.iter().map(|user| {
                        //                 let user = user.clone();
                        //                 view!{
                        //                     <span class="bg-gray-100 text-gray-900 px-1 rounded-md whitespace-nowrap">{user.name}</span>
                        //                 }.into_view()
                        //             }).collect::<Vec<_>>().into_view()}"укушены вампирами."
                        //         </div>
                        //     }.into_view()
                        // }
                    }
                }
            ).collect::<Vec<_>>().into_view()}
        </div>
    }
}

#[component]
fn Timer() -> impl IntoView {
    let (time, set_time) = create_signal(0);

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            // do something
            set_time.update(|time| {
                if *time > 0 {
                    *time -= 1;
                }
            });
        },
        1000,
    );

    let get_time = move || {
        let time = time.get();
        let minutes = time / 60;
        let seconds = time % 60;

        format!("{:01}:{:02}", minutes, seconds)
    };

    pause();

    create_effect(move |_| {
        if is_active.get() && time.get() == 0 {
            pause();

            // play alarm sound
            let audio = web_sys::HtmlAudioElement::new_with_src("assets/alarm.mp3");
            if let Ok(audio) = audio {
                let _ = audio.play().unwrap();
            }
        }
    });

    let start_timer = move |start_time: i32| {
        set_time.update(|time| {
            *time = start_time;
        });
        resume();
    };

    view! {
        <div class="flex items-stretch justify-center gap-1">
            <div class="text-4xl">"⏰"</div>
            <div class="text-4xl">{get_time}</div>
            <button class="flex-1 px-1 py-1 text-sm bg-gray-200 rounded-full" on:click={
                let start_timer = start_timer.clone();

                move|_|{
                    start_timer(60);
                }
            }>"1 мин"</button>
            <button class="flex-1 px-1 py-1 text-sm bg-gray-200 rounded-full" on:click={
                let start_timer = start_timer.clone();

                move|_|{
                    start_timer(30);
                }
            }>"30 сек"</button>
            <button class="flex-1 px-1 py-1 text-sm bg-gray-200 rounded-full" on:click={
                let start_timer = start_timer.clone();

                move|_|{
                    start_timer(15);
                }
            }>"15 сек"</button>
        </div>
    }
}

#[component]
fn DayTurnButtons<F, F2>(
    onclick_next_role: F,
    onclick_start_player: F2,
    clock_choose: RwSignal<bool>,
    start_player_choose: RwSignal<bool>,
    kill_player_choose: RwSignal<bool>,
) -> impl IntoView
where
    F: Fn() + 'static,
    F2: Fn() + 'static,
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
            class="w-9 px-2 py-2 text-sm bg-gray-200 rounded-full flex items-center justify-center"
            on:click=onclick_prev_role
        >
            "←"
        </button>
        <button
            class=move || {
                format!("flex-1 px-2 py-2 text-sm rounded-full flex items-center justify-center {}", if kill_player_choose.get() {
                    "bg-blue-500"
                }else{
                    "bg-gray-200"
                })
            }
            on:click=move |_| {
                kill_player_choose.update(|kill_player_choose| {
                    *kill_player_choose = !*kill_player_choose;
                });
                clock_choose.set(false);
                start_player_choose.set(false);
            }
        >
            "❌"
        </button>
        <button
            class=move || {
                format!("flex-1 px-2 py-2 text-sm rounded-full flex items-center justify-center {}", if start_player_choose.get() {
                    "bg-blue-500"
                }else{
                    "bg-gray-200"
                })
            }
            on:click=move |_| {
                onclick_start_player();
                clock_choose.set(false);
                kill_player_choose.set(false);
            }
        >
            "🏁"
        </button>
        <button
            class=move || {
                format!("flex-1 px-2 py-2 text-sm rounded-full flex items-center justify-center {}", if clock_choose.get() {
                    "bg-blue-500"
                }else{
                    "bg-gray-200"
                })
            }
            on:click=move |_| {
                start_player_choose.set(false);
                clock_choose.update(|clock_choose| {
                    *clock_choose = !*clock_choose;
                });
                kill_player_choose.set(false);
            }
        >
            "⏰"
        </button>
        <button
                class="flex-grow-[2] px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click=onclick_next
            >
                "Далее"
            </button>
        </div>
    }
}

#[component]
fn KillDayButtons<F, F2>(onclick_kill_users: F, onclick_cancel: F2) -> impl IntoView
where
    F: Fn() + 'static,
    F2: Fn() + 'static,
{
    let onclick_prev_role = move |_| {
        onclick_cancel();
    };

    let onclick_next = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>()
            .expect("MafiaContext history not found");

        set_context_history.update(|history| history.push(game_ctx.get_history()));

        onclick_kill_users();
    };

    view! {
        <div class="flex gap-2 w-full items-center">
            <button
                class="flex-grow-[2] px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click=onclick_prev_role
            >
                "Отмена"
            </button>
            <button
                class="flex-grow-[2] px-4 py-2 text-sm bg-gray-200 rounded-full"
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
                user.was_killed = true;
                return;
            }
        }

        user.choosed_by.insert(Role::WasKilled);
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
            if user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                || user.role.contains(&Role::Werewolf(WerewolfRole::Vampire))
            {
                if !is_user_protected(user, &[Role::Werewolf(WerewolfRole::Bodyguard)]) {
                    kill_user(user, &[Role::Werewolf(WerewolfRole::ToughGuy)]);
                }
            } else {
                user.additional_role
                    .insert(Role::Werewolf(WerewolfRole::Priest));
            }
        }

        // Vampire check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Vampire))
        {
            if !is_user_protected(user, &[Role::Werewolf(WerewolfRole::Bodyguard)]) {
                if user
                    .additional_role
                    .contains(&Role::Werewolf(WerewolfRole::Priest))
                {
                    user.additional_role
                        .remove(&Role::Werewolf(WerewolfRole::Priest));
                } else if user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf)) {
                    if !is_user_protected(
                        user,
                        &[
                            Role::Werewolf(WerewolfRole::Bodyguard),
                            Role::Werewolf(WerewolfRole::WitchHeal),
                        ],
                    ) {
                        kill_user(user, &[Role::Werewolf(WerewolfRole::ToughGuy)]);
                    }
                } else {
                    user.role.insert(Role::Werewolf(WerewolfRole::Vampire));
                }
            }
        }

        // Werewolf check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Werewolf))
        {
            if user.role.contains(&Role::Werewolf(WerewolfRole::Cursed)) {
                if !is_user_protected(user, &[Role::Werewolf(WerewolfRole::Bodyguard)]) {
                    // Priest check
                    if user
                        .additional_role
                        .contains(&Role::Werewolf(WerewolfRole::Priest))
                    {
                        user.additional_role
                            .remove(&Role::Werewolf(WerewolfRole::Priest));
                    } else {
                        user.role.insert(Role::Werewolf(WerewolfRole::Werewolf));
                    }
                }
            } else {
                if !is_user_protected(
                    user,
                    &[
                        Role::Werewolf(WerewolfRole::Bodyguard),
                        Role::Werewolf(WerewolfRole::WitchHeal),
                    ],
                ) {
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

        // Huntress check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Huntress))
        {
            if !is_user_protected(
                user,
                &[
                    Role::Werewolf(WerewolfRole::Bodyguard),
                    Role::Werewolf(WerewolfRole::WitchHeal),
                ],
            ) {
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
            if user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                || user.role.contains(&Role::Werewolf(WerewolfRole::Vampire))
            {
                if !is_user_protected(
                    user,
                    &[
                        Role::Werewolf(WerewolfRole::Bodyguard),
                        Role::Werewolf(WerewolfRole::WitchHeal),
                    ],
                ) {
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

    // vampire
    users
        .iter_mut()
        .filter(|u| u.role.contains(&Role::Werewolf(WerewolfRole::Vampire)))
        .for_each(|u| {
            u.additional_role
                .insert(Role::Werewolf(WerewolfRole::Vampire));
        });
}

fn calculate_after_kills(users: &mut [Player]) {
    let mut kill_indices: Vec<(usize, Role)> = Vec::new();

    for user in users.iter() {
        if user.is_alive {
            continue;
        }

        if user.role.contains(&Role::Werewolf(WerewolfRole::Lovers)) {
            users.iter().enumerate().for_each(|(index, u)| {
                if u.role.contains(&Role::Werewolf(WerewolfRole::Lovers)) {
                    kill_indices.push((index, Role::Werewolf(WerewolfRole::Lovers)));
                }
            });
        }

        if user
            .additional_role
            .contains(&Role::Werewolf(WerewolfRole::Vampire))
            && user.role.contains(&Role::Werewolf(WerewolfRole::Vampire))
        {
            users.iter().enumerate().for_each(|(index, u)| {
                if u.role.contains(&Role::Werewolf(WerewolfRole::Vampire)) {
                    kill_indices.push((index, Role::Werewolf(WerewolfRole::Vampire)));
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
                    kill_indices.push((index, Role::Werewolf(WerewolfRole::DireWolf)));
                }
            });
        }
    }

    // Set is_alive to false for all Lovers
    for (index, role) in kill_indices {
        if let Some(u) = users.get_mut(index) {
            if u.is_alive {
                u.is_alive = false;
                u.was_killed = true;
                u.choosed_by.insert(Role::WasKilled);
                u.choosed_by.insert(role);
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
    let show_cursed_convert = create_rw_signal(false);

    let users = move || mafia_context.users.get();

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());
    let role = role_info.get_role();

    let onclick_next_role = move || {
        let selected_users = selected_users.get();
        game_ctx.users.update(|users| {
            users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.id) {
                    u.choosed_by.insert(role);
                }
            })
        });

        if show_cursed_convert.get() == false && role == Role::Werewolf(WerewolfRole::Werewolf) {
            let users = game_ctx.users.get();
            let is_selected = users.iter().any(|u| {
                u.role.contains(&Role::Werewolf(WerewolfRole::Cursed))
                    && !u.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                    && u.is_alive
                    && !u
                        .choosed_by
                        .contains(&Role::Werewolf(WerewolfRole::Bodyguard))
                    && !u.choosed_by.contains(&Role::Werewolf(WerewolfRole::Priest))
                    && !u
                        .additional_role
                        .contains(&Role::Werewolf(WerewolfRole::Priest))
                    && selected_users.contains(&u.id)
            });

            if is_selected {
                show_cursed_convert.set(true);
                return;
            }
        }

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
            return user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                || user.role.contains(&Role::Werewolf(WerewolfRole::Vampire))
                || user.role.contains(&Role::Werewolf(WerewolfRole::Lycan));
        }

        if role_info.get_role() == Role::Werewolf(WerewolfRole::Mentalist) {
            return user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                || user.role.contains(&Role::Werewolf(WerewolfRole::Vampire))
                || user.role.contains(&Role::Werewolf(WerewolfRole::Minion));
        }

        if role_info.get_role() == Role::Werewolf(WerewolfRole::ParanormalInvestigator) {
            return user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                || user.role.contains(&Role::Werewolf(WerewolfRole::Vampire))
                || user.role.contains(&Role::Werewolf(WerewolfRole::Lycan));
        }

        false
    };

    let game_log: Memo<Vec<WerewolfHint>> = create_memo(move |_| {
        let mut log = Vec::<WerewolfHint>::new();

        let users = game_ctx.users.get();

        if role_info.get_role() == Role::Werewolf(WerewolfRole::Werewolf) {
            users.iter().for_each(|user| {
                if user.role.contains(&Role::Werewolf(WerewolfRole::Cursed))
                    && !user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                    && user.is_alive
                    && !user
                        .choosed_by
                        .contains(&Role::Werewolf(WerewolfRole::Bodyguard))
                    && !user
                        .choosed_by
                        .contains(&Role::Werewolf(WerewolfRole::Priest))
                    && !user
                        .additional_role
                        .contains(&Role::Werewolf(WerewolfRole::Priest))
                {
                    log.push(WerewolfHint::Cursed(user.clone()));
                }
            });
        }

        if role_info.get_role() == Role::Werewolf(WerewolfRole::Seer) {
            let ww_users = users
                .iter()
                .filter(|user| {
                    return user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                        || user.role.contains(&Role::Werewolf(WerewolfRole::Vampire))
                        || user.role.contains(&Role::Werewolf(WerewolfRole::Lycan))
                            && user.is_alive;
                })
                .cloned()
                .collect::<Vec<_>>();

            if !ww_users.is_empty() {
                log.push(WerewolfHint::Seer(ww_users));
            }
        }

        log
    });

    view! {
        <h2>
            {night_description}
        </h2>
        <DisplayLogs logs=game_log />
        <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
            <div class="flex-1"></div>
            <div class="flex flex-col gap-1 w-full">
            <SelectUsersForVote
                is_killed=move |user: &Player| !user.is_alive && !user.was_killed
                selected_users set_selected_users is_disabled is_highlighted
                highlight_color="ring-red-600/50".to_string()
            />
            </div>
        </div>
        <NightTurnButtons onclick_next_role />
        {move || if show_cursed_convert.get() {
            view!{
                <div class="absolute -bottom-1 -left-1 -right-1 rounded-2xl p-3 bg-fuchsia-300">
                    <div class="mb-4 flex flex-col items-center justify-center">"Cursed был превращен в Werewolf"</div>
                    <div class="flex items-center justify-center">
                        <button
                            class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                            on:click=move |_| onclick_next_role()
                        >Далее</button>
                    </div>
                </div>
            }.into_view()
        } else {
            "".into_view()
        }}
    }
}

#[component]
fn NightTurnButtons<F>(onclick_next_role: F) -> impl IntoView
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
                class="w-9 px-2 py-2 text-sm bg-gray-200 rounded-full flex items-center justify-center"
                on:click=onclick_prev_role
            >
                "←"
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
