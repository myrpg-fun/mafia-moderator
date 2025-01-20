use itertools::Itertools;
use leptos_use::utils::*;
use leptos_use::*;
use std::collections::HashMap;
use std::collections::HashSet;

use leptos::*;
use serde::Deserialize;
use serde::Serialize;
use crate::rust_create_new_game_log;
use crate::user::*;
use crate::roles::*;
use crate::GameContextHistory;
use crate::GameState;
use crate::GameContext;
use crate::UserLogs;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum MafiaRole {
    None,
    Citizen,
    Mafia,
    Doctor,
    Detective,
    Maniac,
    Prostitute,
    Priest,
}

const _MAFIA_COLORS: [&str; 10] = [
    "text-red-950", "text-blue-950", "text-gray-950", "text-green-950", "text-purple-950",
    "bg-red-950", "bg-blue-950", "bg-gray-950", "bg-green-950", "bg-purple-950",
];

pub const MAFIA_ROLES: [RoleInfo; 8] = [
    RoleInfo::Icon(IconRoleInfo{
        role: Role::WasKilled,
        role_name: "Killed",
        role_name_color: "red-950",
        role_icon: "❌",
    }),
    RoleInfo::Icon(IconRoleInfo{
        role: Role::Mafia(MafiaRole::Citizen),
        role_name: "Мирные",
        role_name_color: "blue-950",
        role_icon: "✋",
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Mafia),
        check_role: None,
        role_name: "Мафия",
        role_name_color: "red-950",
        role_icon: "🔫",
        prepare_description: "Выберите игроков Мафии",
        night_description: "Кого убьет Мафия?",
        targeting_rules: NightTargetingRules::Anyone,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Detective),
        check_role: None,
        role_name: "Детектив",
        role_name_color: "blue-950",
        role_icon: "🔍",
        prepare_description: "Выберите игрока Детектива",
        night_description: "Кого проверит Детектив?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Maniac),
        check_role: None,
        role_name: "Маньяк",
        role_name_color: "gray-950",
        role_icon: "🔪",
        prepare_description: "Выберите игрока Маньяка",
        night_description: "Кого убьет Маньяк?",
        targeting_rules: NightTargetingRules::Anyone,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Doctor),
        check_role: None,
        role_name: "Доктор",
        role_name_color: "green-950",
        role_icon: "🚑",
        prepare_description: "Выберите игрока Доктора",
        night_description: "Кого спасет Доктор?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Prostitute),
        check_role: None,
        role_name: "Проститутка",
        role_name_color: "green-950",
        role_icon: "💋",
        prepare_description: "Выберите игрока Проститутку",
        night_description: "К кому зайдет Проститутка?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Priest),
        check_role: None,
        role_name: "Священник",
        role_name_color: "blue-950",
        role_icon: "🙏",
        prepare_description: "Выберите игрока Священника",
        night_description: "Кого проверит Священник?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
];

#[derive(Clone, Debug, PartialEq)]
enum MafiaHint {
    Killed(Player, HashSet<Role>),
    Prostitute(Player),
    Detective(Vec<Player>),
    Priest(Vec<Player>),
}

#[derive(Clone, Copy, Debug)]
pub enum MafiaGameState<'a> {
    SetupRoles(&'a RoleInfo),
    Day,
    Night(&'a RoleInfo),
}

fn get_next_prepare_role(role: Role) -> Option<&'static RoleInfo> {
    let role_index = MAFIA_ROLES.iter().position(|r| *r == role).unwrap();

    MAFIA_ROLES.get(role_index.wrapping_add(1))
}

fn get_next_night_role(role: Role) -> Option<&'static RoleInfo> {
    let role_index = MAFIA_ROLES.iter().position(|r| *r == role).unwrap();

    MAFIA_ROLES.get(role_index.wrapping_add(1))
}

#[derive(Debug, Clone)]
struct OpenFinishGameDialogue(bool);

#[component]
pub fn MafiaGameView() -> impl IntoView {
    let open_finish_game_dialogue = create_rw_signal(OpenFinishGameDialogue(false));
    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    provide_context(open_finish_game_dialogue);

    let game_state_view = move || {
        if open_finish_game_dialogue.get().0 {
            return view! {
                <SelectWinners 
                    on_close=move || open_finish_game_dialogue.set(OpenFinishGameDialogue(false))
                    on_finish=move || mafia_context.game_state.set(GameState::SetupNames)
                />
            }.into_view();
        }

        match mafia_context.game_state.get() {
            GameState::Mafia(game_state) => match game_state {
                MafiaGameState::SetupRoles(role) => view! {
                    <SetupRolesView role={role} />
                }
                .into_view(),

                MafiaGameState::Day => view! {
                    <DayVote />
                }
                .into_view(),
                
                MafiaGameState::Night(role) => view! {
                    <NightTurn role_info={role} />
                }
                .into_view(),
            },
            _ => view! {
                <div>"Ошибка"</div>
            }.into_view()
        }
    };

    view! {
        <div class="relative flex flex-col gap-4 w-full h-full">
            <h1 class="text-lg relative w-full text-left">
                "Мафия"
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

fn calculate_user_logs(users: Vec<Player>, best_players: HashSet<String>, selected_winners: HashSet<Role>) -> Vec::<UserLogs>{
    let mut logs = Vec::<UserLogs>::new();
    let mut last_round = 0;

    for user in users.iter() {
        for (index, _) in user.history_by.iter() {
            last_round = last_round.max(*index);
        }
    }

    let mut user_history = HashMap::<String, Vec::<(usize, HashSet<Role>)>>::new();
    for user in users.iter() {
        let mut rounds = user.history_by.clone();
        rounds.push((last_round + 1, user.choosed_by.clone()));

        user_history.insert(user.id.clone(), rounds);
    }

    for user in users.iter() {
        let mut rounds = Vec::<String>::new();
        rounds.resize(last_round + 1, "".to_string());

        let current_user_history = user_history.get(&user.id).expect("user_history not found");

        for (index, roles) in current_user_history.iter() {
            let role = roles.iter()
                //sort WasKilled role
                .sorted_by(|a, b| {
                    if **a == Role::WasKilled {
                        std::cmp::Ordering::Greater
                    } else if **b == Role::WasKilled {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .map(|role| {
                    MAFIA_ROLES.iter().find(|r| r.get_role() == *role).unwrap().get_role_icon()
                }).collect::<Vec<_>>().join(" ");
            
            // set role icons to rounds[index]
            // index might be empty, we should create "" for it
            let index = if *index <= 0 { 0 } else { *index - 1 };
            rounds[index] = role;
        }

        let winner = 
            (selected_winners.contains(&Role::Mafia(MafiaRole::Maniac)) 
                && user.role.contains(&Role::Mafia(MafiaRole::Maniac)))
            || (selected_winners.contains(&Role::Mafia(MafiaRole::Mafia)) 
                && user.role.contains(&Role::Mafia(MafiaRole::Mafia)))
            || (selected_winners.contains(&Role::Mafia(MafiaRole::Citizen)) 
                && !user.role.contains(&Role::Mafia(MafiaRole::Mafia)) 
                && !user.role.contains(&Role::Mafia(MafiaRole::Maniac)));

        let score = 0;

        let role = if user.role.is_empty() {
            "Мирный".to_string()
        }else{
            user.role.iter().map(|role| {
                MAFIA_ROLES.iter().find(|r| r.get_role() == *role).unwrap().get_role_name()
            }).collect::<Vec<_>>().join(" • ")
        };

        let role_index = if user.role.contains(&Role::Mafia(MafiaRole::Mafia)) {
            "Mafia"
        }else if user.role.contains(&Role::Mafia(MafiaRole::Maniac)) {
            "Maniac"
        }else if user.role.contains(&Role::Mafia(MafiaRole::Detective)) {
            "Detective"
        }else if user.role.contains(&Role::Mafia(MafiaRole::Prostitute)) {
            "Prostitute"
        }else if user.role.contains(&Role::Mafia(MafiaRole::Doctor)) {
            "Doctor"
        }else if user.role.contains(&Role::Mafia(MafiaRole::Priest)) {
            "Priest"
        }else{
            "Citizen"
        }.to_string();

        let role_score = if user.role.contains(&Role::Mafia(MafiaRole::Maniac)) {
            let mut score = 0;

            for checked_user in users.iter().filter(|u| u.role.contains(&Role::Mafia(MafiaRole::Mafia))) {
                let current_user_history = user_history.get(&checked_user.id).expect("user_history not found");
                current_user_history.iter().for_each(|(_, roles)| {
                    if roles.contains(&Role::Mafia(MafiaRole::Maniac)) && !roles.contains(&Role::Mafia(MafiaRole::Doctor)) && !roles.contains(&Role::Mafia(MafiaRole::Prostitute)) {
                        score += 1;
                    }
                });
            }

            if winner {
                score += 1;
            }

            score
        }else if user.role.contains(&Role::Mafia(MafiaRole::Detective)) {
            let mut score = 0;

            for checked_user in users.iter().filter(|u| u.role.contains(&Role::Mafia(MafiaRole::Mafia))) {
                let current_user_history = user_history.get(&checked_user.id).expect("user_history not found");
                current_user_history.iter().for_each(|(_, roles)| {
                    if roles.contains(&Role::Mafia(MafiaRole::Detective)) {
                        score += 1;
                    }
                });
            }

            if winner {
                score += 1;
            }

            score
        }else if user.role.contains(&Role::Mafia(MafiaRole::Priest)) {
            let mut score = 0;

            for checked_user in users.iter().filter(|u| u.role.contains(&Role::Mafia(MafiaRole::Maniac))) {
                let current_user_history = user_history.get(&checked_user.id).expect("user_history not found");
                current_user_history.iter().for_each(|(_, roles)| {
                    if roles.contains(&Role::Mafia(MafiaRole::Priest)) {
                        score += 1;
                    }
                });
            }

            if winner {
                score += 1;
            }

            score
        }else if user.role.contains(&Role::Mafia(MafiaRole::Doctor)) {
            let mut score = 0;

            for checked_user in users.iter() {
                let current_user_history = user_history.get(&checked_user.id).expect("user_history not found");
                current_user_history.iter().for_each(|(_, roles)| {
                    if roles.contains(&Role::Mafia(MafiaRole::Doctor)) && (roles.contains(&Role::Mafia(MafiaRole::Mafia)) || roles.contains(&Role::Mafia(MafiaRole::Maniac))) {
                        score += 1;
                    }
                });
            }

            if winner {
                score += 1;
            }

            score
        }else if user.role.contains(&Role::Mafia(MafiaRole::Prostitute)) {
            let mut score = 0;

            for checked_user in users.iter() {
                let current_user_history = user_history.get(&checked_user.id).expect("user_history not found");
                current_user_history.iter().for_each(|(_, roles)| {
                    if roles.contains(&Role::Mafia(MafiaRole::Prostitute)) && (roles.contains(&Role::Mafia(MafiaRole::Mafia)) || roles.contains(&Role::Mafia(MafiaRole::Maniac))) {
                        score += 1;
                    }
                });
            }

            let p_user_history = user_history.get(&user.id).expect("user_history not found");
            for checked_user in users.iter().filter(|u| u.role.contains(&Role::Mafia(MafiaRole::Mafia))) {
                let current_user_history = user_history.get(&checked_user.id).expect("user_history not found");
                current_user_history.iter().for_each(|(mafia_round, roles)| {
                    if roles.contains(&Role::Mafia(MafiaRole::Prostitute)) {
                        p_user_history.iter().for_each(|(prostitute_round, roles)| {
                            if mafia_round == prostitute_round && 
                            (roles.contains(&Role::Mafia(MafiaRole::Mafia)) || 
                            roles.contains(&Role::Mafia(MafiaRole::Maniac))) 
                            && !roles.contains(&Role::Mafia(MafiaRole::Doctor)) {
                                score += 1;
                            }
                        });
                    }
                });
            }

            for checked_user in users.iter().filter(|u| u.role.contains(&Role::Mafia(MafiaRole::Maniac))) {
                let current_user_history = user_history.get(&checked_user.id).expect("user_history not found");
                current_user_history.iter().for_each(|(mafia_round, roles)| {
                    if roles.contains(&Role::Mafia(MafiaRole::Prostitute)) {
                        p_user_history.iter().for_each(|(prostitute_round, roles)| {
                            if mafia_round == prostitute_round && 
                            (roles.contains(&Role::Mafia(MafiaRole::Mafia)) || 
                            roles.contains(&Role::Mafia(MafiaRole::Maniac))) 
                            && !roles.contains(&Role::Mafia(MafiaRole::Doctor)) {
                                score += 1;
                            }
                        });
                    }
                });
            }

            if winner {
                score += 1;
            }

            score
        }else{
            if winner { 1 } else { 0 } 
        };

        let best_player = best_players.contains(&user.id);

        logs.push(UserLogs{
            id: user.id.clone(),
            name: user.name.clone(),
            is_guest: user.is_guest,
            role,
            role_index,
            role_score,
            best_player,
            score,
            winner,
            rounds,
        });
    }

    logs
}

#[component]
fn SelectWinners(on_close: impl Fn() -> () + Clone + 'static, on_finish: impl Fn() -> () + Clone + 'static) -> impl IntoView{
    let selected_winners = create_rw_signal(HashSet::<Role>::new());
    let selected_users = create_rw_signal(HashSet::<String>::new());

    let roles = [
        RoleInfo::Icon(IconRoleInfo{
            role: Role::Mafia(MafiaRole::Maniac),
            role_name: "Маньяк",
            role_name_color: "purple-950",
            role_icon: "🔪",
        }),
        RoleInfo::Icon(IconRoleInfo{
            role: Role::Mafia(MafiaRole::Mafia),
            role_name: "Мафия",
            role_name_color: "red-950",
            role_icon: "🔫",
        }),
        RoleInfo::Icon(IconRoleInfo{
            role: Role::Mafia(MafiaRole::Citizen),
            role_name: "Мирные",
            role_name_color: "green-950",
            role_icon: "✋",
        }),
    ];

    let is_selected = move |role: &Role|{
        selected_winners.get().contains(role)
    };

    let calculate_user_logs_fn = move || {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let users = game_ctx.users.get();
        let best_players = selected_users.get();
        let selected_winners = selected_winners.get();

        calculate_user_logs(users, best_players, selected_winners)
    };

    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    let users = move || {
        mafia_context.users.get()
    };

    view!{
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
            <div class="flex flex-row gap-1 justify-stretch w-full">
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
                            rust_create_new_game_log(calculate_user_logs_fn(), true);
                            
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
fn UserRow(user: Player) -> impl IntoView {
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
    let users = game_ctx.users;

    view! {
        <div class="flex gap-2">
            <div class="flex-1 px-3 py-1 text-base bg-gray-200 rounded-full">{user.name.clone()}</div>
            <button class="text-lg"
                on:click=move |_| {
                    users.update(|users| users.retain(|u| *u != user));
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
                <SelectUserForRole role=role />
            </div>
        </div>
        <TurnButtons role_info=role />
    }
}

#[component]
fn SetupRolesHeader<'a>(role: &'a RoleInfo) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <h2>
                {role.get_prepare_description()}
            </h2>
        </div>
    }
}

#[component]
fn SelectUserForRole<'a>(role: &'a RoleInfo) -> impl IntoView {
    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    let role = role.get_role();
    let users_sorted = move || users_sorted(mafia_context.users.get());

    view! {
        <div class="grid grid-cols-2 gap-y-1 gap-x-3">
            <For
                each=users_sorted
                key=|user| format!("{}_{}", user.id.clone(), user.role.len())
                children=move |user| {
                    let user_clone = user.clone();

                    view!{
                        <UserSelectRole
                            user=user_clone
                            disabled=false
                            is_selected=move |u| u.role.contains(&role)
                            highlighted=false
                            killed=false
                            on:click=move |_| {
                                mafia_context.users.update(|users| {
                                    if let Some(user) = users.iter_mut().find(|u| **u == user) {
                                        if user.role.contains(&role) {
                                            user.role.remove(&role);
                                        }else if user.role.is_empty(){
                                            user.role.insert(role);
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

fn user_background_role_picture(user: &Player) -> String {
    if user.role.contains(&Role::Mafia(MafiaRole::Mafia)) {
        "assets/mafia.png".to_string()
    } else if user.role.contains(&Role::Mafia(MafiaRole::Detective)){
        "assets/detective.png".to_string()
    } else if user.role.contains(&Role::Mafia(MafiaRole::Doctor)){
        "assets/doctor.png".to_string()
    } else if user.role.contains(&Role::Mafia(MafiaRole::Priest)){
        "assets/priest.png".to_string()
    } else if user.role.contains(&Role::Mafia(MafiaRole::Prostitute)){
        "assets/prostitute.png".to_string()
    } else if user.role.contains(&Role::Mafia(MafiaRole::Maniac)){
        "assets/maniac.png".to_string()
    } else {
        "assets/citizen.png".to_string()
    }
}

fn user_background_role_color(user: &Player) -> &str {
    if user.role.contains(&Role::Mafia(MafiaRole::Mafia)) {
        "bg-red-100"
    } else if user.role.contains(&Role::Mafia(MafiaRole::Detective)){
        "bg-cyan-100"
    } else if user.role.contains(&Role::Mafia(MafiaRole::Doctor)){
        "bg-emerald-100"
    } else if user.role.contains(&Role::Mafia(MafiaRole::Priest)){
        "bg-cyan-100"
    } else if user.role.contains(&Role::Mafia(MafiaRole::Prostitute)){
        "bg-emerald-100"
    } else if user.role.contains(&Role::Mafia(MafiaRole::Maniac)){
        "bg-fuchsia-100"
    } else {
        "bg-gray-100"
    }
}

#[component]
fn UserSelectRole(
    user: Player,
    is_selected: impl Fn(&Player) -> bool + 'static + Clone,
    disabled: bool,
    killed: bool,
    highlighted: bool,
    #[prop(default = "ring-red-600/50".to_string())] highlight_color: String,
) -> impl IntoView {
    let history = user.history_by.clone();
    let choosed = user.choosed_by.clone();
    let user_c1 = user.clone();
    // let user_c2 = user.clone();
    // let is_selected_clone = is_selected.clone();

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
                <div class="text-left">{user.name}</div>
                <UserRoleNames role=user.role />
            </div>
            <UserHistory hystory=history current=choosed />
            {move || if user.was_killed {
                view! {
                    <div class="text-[0.5rem]">"❌"</div>
                }.into_view()
            }else{
                "".into_view()
            }}
        </button>
    }
}

#[component]
fn UserRoleNames(role: HashSet<Role>) -> impl IntoView{
    view! {
        <div class="flex">
            {move || {
                if role.is_empty() {
                    view!{
                        <UserRoleName role=Role::Mafia(MafiaRole::None) />
                    }.into_view()
                }else{
                    role.iter().map(|role| {
                        let role = *role;

                        view!{
                            <UserRoleName role=role />
                        }
                    }).collect::<Vec<_>>().into_view()
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
                    Role::Mafia(MafiaRole::Mafia)
                        | Role::Mafia(MafiaRole::Maniac)
                        | Role::Mafia(MafiaRole::Prostitute)
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
fn UserHistory(hystory: Vec<(usize, HashSet<Role>)>, current: HashSet<Role>) -> impl IntoView {
    view! {
        <div class="relative z-20 flex flex-col gap-0.5 flex-wrap">
            <div class="flex items-center gap-0.5">
            {
                current.iter().map(|role| {
                    let role = *role;

                    if role != Role::WasKilled{
                        view!{
                            <UserRoleIcon role=role is_hystory=false />
                        }
                    }else{
                        "".into_view()
                    }
                }).collect::<Vec<_>>().into_view()
            }
            </div>
        </div>
    }
}

#[component]
fn UserRoleIcon(role: Role, is_hystory: bool) -> impl IntoView {
    MAFIA_ROLES
        .iter()
        .find(|r| r.get_role() == role)
        .map(|role_info| {
            view! {
                <div 
                    class=move || if is_hystory {"text-xs opacity-80 w-4 h-4"} else {"text-xs rounded-md bg-white w-4 h-4"}
                >
                    {role_info.get_role_icon()}
                </div>
            }
        })
}

#[component]
fn UserRoleName(role: Role) -> impl IntoView {
    MAFIA_ROLES
        .iter()
        .find(|r| r.get_role() == role)
        .map_or_else(move || {
            view! {
                <div 
                    class="text-xs opacity-20"
                >
                    "Не выбрано"
                </div>
            }
        }, move |role_info| {
            view! {
                <div
                    class=move || format!("text-xs opacity-50 {}", role_info.get_role_name_color())
                >
                    {role_info.get_role_name()}
                </div>
            }
        }).into_view()
}

#[component]
fn TurnButtons<'a>(role_info: &'a RoleInfo) -> impl IntoView {
    let role = role_info.get_role();
    let onclick_next_role = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
    
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>().expect("MafiaContext history not found");
        set_context_history.update(|history| history.push(game_ctx.get_history()));

        match get_next_prepare_role(role) {
            Some(role_info) => game_ctx.game_state.set(GameState::Mafia(MafiaGameState::SetupRoles(role_info))),
            None => {
                game_ctx.round.set(0);
                game_ctx.game_state.set(GameState::Mafia(MafiaGameState::Day));
            }
        };
    };

    let onclick_prev_role = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>().expect("MafiaContext history not found");
    
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
    is_disabled: impl Fn(&Player) -> bool + 'static,
    is_highlighted: impl Fn(&Player) -> bool + 'static,
    is_killed: impl Fn(&Player) -> bool + 'static,
    is_single_select: bool,
    #[prop(default = "ring-red-600/50".to_string())] highlight_color: String,
) -> impl IntoView {
    let mafia_context = use_context::<GameContext>().expect("MafiaContext not found");

    let users = move || mafia_context.users.get();
    let users_alive_len = move || users().iter().filter(|u| u.is_alive).count();
    let mafia_alive_len = move || {
        users()
            .iter()
            .filter(|u| u.is_alive && u.role.contains(&Role::Mafia(MafiaRole::Mafia)))
            .count()
    };
    let is_selected = move |user: &Player| selected_users.get().contains(&user.id);
    let users_sorted = move || users_sorted(mafia_context.users.get());

    view! {
        <div class="text-sm">"Осталось игроков: "{users_alive_len}", мафий: "{mafia_alive_len}</div>
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
                            is_selected=is_selected
                            highlight_color=highlight_color.clone()
                            killed=killed
                            on:click=move |_| {
                                set_selected_users.update(|selected_users| {
                                    if selected_users.contains(&user.id) {
                                        selected_users.remove(&user.id);
                                    } else {
                                        if is_single_select {
                                            selected_users.clear();
                                        }
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

#[component]
fn DayVote() -> impl IntoView {
    let clock_choose = create_rw_signal(true);
    let kill_player_choose = create_rw_signal(false);
    let start_player_choose = create_rw_signal(false);

    let (highlighted_player, set_highlighted_player) =
        create_signal::<HashSet<String>>(HashSet::new());

    let open_dialogue = use_context::<RwSignal<OpenFinishGameDialogue>>().expect("MafiaContext not found");
    let game_ctx =
        use_context::<GameContext>().expect("MafiaContext not found");

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());

    let onclick_next_role = move || {
        let selected_users = selected_users.get();

        let round = game_ctx.round.get();
        game_ctx.users.update(|users|{
            clear_was_killed(users);
            clear_choosed_by(users, round);
            
            users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.id) {
                    let mut citizen_history = HashSet::new();
                    citizen_history.insert(Role::Mafia(MafiaRole::Citizen));
                    citizen_history.insert(Role::WasKilled);
                    u.history_by.push((round + 1, citizen_history));
                    u.is_alive = false;
                    u.was_killed = true;
                }
            });
        });
        game_ctx.round.set(round + 2);

        let users = game_ctx.users.get();
        let mut next_role = Some(MAFIA_ROLES.first().unwrap());
        loop {
            if let Some(check_role) = next_role {
                if is_role_alive(check_role.get_role(), &users) {
                    game_ctx.game_state.set(GameState::Mafia(MafiaGameState::Night(check_role)));
                    break;
                }
                next_role = get_next_night_role(check_role.get_role());
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
            fn kill_user(user: &mut Player, round: usize) {
                if !user.is_alive {
                    return;
                }

                let mut citizen_history = HashSet::new();
                citizen_history.insert(Role::Mafia(MafiaRole::Citizen));
                citizen_history.insert(Role::WasKilled);
                user.history_by.push((round + 1, citizen_history));
                user.is_alive = false;
                user.was_killed = true;
            }

            users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.id) {
                    kill_user(u, round);
                }
            });
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
        let mut log = Vec::<MafiaHint>::new();

        let users = game_ctx.users.get();

        users.iter().for_each(|user| {
            if user.was_killed && !user.is_alive {
                log.push(MafiaHint::Killed(user.clone(), user.choosed_by.clone()));
            }
        });

        users.iter().for_each(|user| {
            if user
                .choosed_by
                .contains(&Role::Mafia(MafiaRole::Prostitute))
                && user.is_alive
            {
                log.push(MafiaHint::Prostitute(user.clone()));
            }
        });

        log
    });

    let is_highlighted = move |user: &Player| highlighted_player.get().contains(&user.id);

    view!{
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
                                is_single_select=true
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
                                is_single_select=false
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
                        <div class="flex flex-col gap-1 w-full">
                            <SelectUsersForVote 
                                selected_users 
                                set_selected_users 
                                is_disabled=move |user: &Player| !user.is_alive 
                                is_highlighted
                                highlight_color="ring-blue-600/80".to_string()
                                is_killed=move |user: &Player| !user.is_alive && !user.was_killed
                                is_single_select=false />
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
fn DisplayLogs(logs: Memo<Vec<MafiaHint>>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1 relative text-xs">
            {move || logs.get().iter().map(
                |log| {
                    match log {
                        MafiaHint::Killed(user, killed_by) => {
                            let user = user.clone();
                            view!{
                                <div class="w-full flex items-center justify-start gap-1 text-gray-500">
                                    "❌"<span class="bg-gray-100 text-gray-900 px-1 rounded-md">{user.name}</span><UserRoleNames role=user.role />" убит"<UserKilledBy killed_by=killed_by.clone() />"."
                                </div>
                            }.into_view()
                        },
                        MafiaHint::Prostitute(user) => {
                            let user = user.clone();
                            view!{
                                <div class="w-full flex items-center justify-start gap-1 text-gray-500">
                                    "💋"<span class="bg-gray-100 text-gray-900 px-1 rounded-md">{user.name}</span>"не может говорить."
                                </div>
                            }.into_view()
                        },
                        MafiaHint::Detective(users) => {
                            view!{
                                <div class="w-full flex-wrap flex items-center justify-start gap-1.5 text-gray-500">
                                    "🔍"{users.iter().map(|user| {
                                        let user = user.clone();
                                        view!{
                                            <span class="bg-gray-100 text-gray-900 px-1 rounded-md whitespace-nowrap">{user.name}</span>
                                        }.into_view()
                                    }).collect::<Vec<_>>().into_view()}"мафия."
                                </div>
                            }.into_view()
                        },
                        MafiaHint::Priest(users) => {
                            view!{
                                <div class="w-full flex-wrap flex items-center justify-start gap-1.5 text-gray-500">
                                    "🔍"{users.iter().map(|user| {
                                        let user = user.clone();
                                        view!{
                                            <span class="bg-gray-100 text-gray-900 px-1 rounded-md whitespace-nowrap">{user.name}</span>
                                        }.into_view()
                                    }).collect::<Vec<_>>().into_view()}"маньяк."
                                </div>
                            }.into_view()
                        }
                    }
                }
            ).collect::<Vec<_>>().into_view()}
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
fn NextTurnButtons<F>(onclick_next_role: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let onclick_prev_role = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>().expect("MafiaContext history not found");
    
        set_context_history.update(|history| {
            if let Some(prev_ctx) = history.pop() {
                game_ctx.set_history(prev_ctx);
            }
        });
    };

    let onclick_next = move |_| {
        let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>().expect("MafiaContext history not found");

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

fn is_role_alive(role: Role, users: &[Player]) -> bool {
    users.iter().any(|u| u.role.contains(&role) && u.is_alive)
}

fn get_next_night_alive_role(role: Role, users: &[Player]) -> Option<&'static RoleInfo> {
    let mut next_role = role;
    loop {
        let check_role = get_next_night_role(next_role);
        if let Some(check_role) = check_role {
            if is_role_alive(check_role.get_role(), users) {
                return Some(check_role);
            }
            next_role = check_role.get_role();
        } else {
            return None;
        }
    }
}

fn clear_choosed_by(users: &mut [Player], round: usize) {
    for user in users.iter_mut() {
        // filter Citizen role
        let choosed_by = user.choosed_by.clone();//.into_iter().filter(|r| *r != Role::Mafia(MafiaRole::Citizen)).collect::<HashSet<_>>();

        if !choosed_by.is_empty(){
            user.history_by.push((round, choosed_by));
        }
        user.choosed_by.clear();
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

    let killed_by_mafia = alive_users.iter_mut().find(|u| {
        u.choosed_by.contains(&Role::Mafia(MafiaRole::Mafia))
            && !u.choosed_by.contains(&Role::Mafia(MafiaRole::Doctor))
            && !u.choosed_by.contains(&Role::Mafia(MafiaRole::Prostitute))
    });

    if let Some(killed_by_mafia) = killed_by_mafia {
        killed_by_mafia.is_alive = false;
        killed_by_mafia.was_killed = true;
        killed_by_mafia.choosed_by.insert(Role::WasKilled);
        if killed_by_mafia.role.contains(&Role::Mafia(MafiaRole::Prostitute)) {
            let saved_by_prostitute = alive_users
                .iter_mut()
                .find(|u| u.choosed_by.contains(&Role::Mafia(MafiaRole::Prostitute)));

            if let Some(saved_by_prostitute) = saved_by_prostitute {
                if !saved_by_prostitute.role.contains(&Role::Mafia(MafiaRole::Mafia)) {
                    saved_by_prostitute.is_alive = false;
                    saved_by_prostitute.was_killed = true;
                    saved_by_prostitute.choosed_by.insert(Role::WasKilled);
                }
            }
        }
    }

    // Maniac killed choosed user
    let killed_by_maniac = alive_users.iter_mut().find(|u| {
        u.choosed_by.contains(&Role::Mafia(MafiaRole::Maniac))
            && !u.choosed_by.contains(&Role::Mafia(MafiaRole::Doctor))
            && !u.choosed_by.contains(&Role::Mafia(MafiaRole::Prostitute))
    });

    if let Some(killed_by_maniac) = killed_by_maniac {
        killed_by_maniac.is_alive = false;
        killed_by_maniac.was_killed = true;
        killed_by_maniac.choosed_by.insert(Role::WasKilled);
        if killed_by_maniac.role.contains(&Role::Mafia(MafiaRole::Prostitute)) {
            let saved_by_prostitute = alive_users
                .iter_mut()
                .find(|u| u.choosed_by.contains(&Role::Mafia(MafiaRole::Prostitute)));

            if let Some(saved_by_prostitute) = saved_by_prostitute {
                if !saved_by_prostitute.role.contains(&Role::Mafia(MafiaRole::Maniac)) {
                    saved_by_prostitute.is_alive = false;
                    saved_by_prostitute.was_killed = true;
                    saved_by_prostitute.choosed_by.insert(Role::WasKilled);
                }
            }
        }
    }
}

// check is user history contains role
fn check_user_history_for_role(user: &Player, role: &Role) -> bool {
    user.history_by.iter().any(|(_, roles)| {
        roles.contains(role)
    })
}

#[component]
fn NightTurn(role_info: &'static RoleInfo) -> impl IntoView {
    let game_ctx =
        use_context::<GameContext>().expect("MafiaContext not found");
    let night_description = role_info.get_night_description();

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());
    let role = role_info.get_role();

    let onclick_next_role = move || {
        let selected_user = selected_users.get();
        game_ctx.users.update(|users| {
            users.iter_mut().for_each(|u| {
                if selected_user.contains(&u.id) {
                    u.choosed_by.insert(role);
                }
            });
        });

        let users = game_ctx.users.get();
        let next_role = get_next_night_alive_role(role, &users);

        match next_role {
            Some(next_role) => {
                game_ctx.game_state.set(GameState::Mafia(MafiaGameState::Night(next_role)));
            }
            None => {
                game_ctx.users.update(|users| {
                    calculate_night_kills(users);
                });
                game_ctx.game_state.set(GameState::Mafia(MafiaGameState::Day));
            }
        }
    };

    let role_targeting_rules = role_info.get_targeting_rules();
    let is_disabled = move |user: &Player| {
        !user.is_alive || //(!role_info_clone.get_can_choose_twice() && user.hystory_by.contains(&role))
        match role_targeting_rules {
            NightTargetingRules::NotTheSame => check_user_history_for_role(&user, &role),
            _ => false,
        }
    };

    let is_highlighted = move |user: &Player| {
        role_info.get_role() == Role::Mafia(MafiaRole::Detective) && user.role.contains(&&Role::Mafia(MafiaRole::Mafia))
        || role_info.get_role() == Role::Mafia(MafiaRole::Priest) && user.role.contains(&&Role::Mafia(MafiaRole::Maniac))
    };

    let game_log: Memo<Vec<MafiaHint>> = create_memo(move |_| {
        let mut log = Vec::<MafiaHint>::new();

        let users = game_ctx.users.get();

        if role_info.get_role() == Role::Mafia(MafiaRole::Detective) {
            let mut ww_users = Vec::<Player>::new();

            users.iter().for_each(|user| {
                if (user.role.contains(&Role::Mafia(MafiaRole::Mafia)))
                    && user.is_alive
                {
                    ww_users.push(user.clone());
                }
            });

            log.push(MafiaHint::Detective(ww_users));
        }

        if role_info.get_role() == Role::Mafia(MafiaRole::Priest) {
            let mut ww_users = Vec::<Player>::new();

            users.iter().for_each(|user| {
                if (user.role.contains(&Role::Mafia(MafiaRole::Maniac)))
                    && user.is_alive
                {
                    ww_users.push(user.clone());
                }
            });

            log.push(MafiaHint::Priest(ww_users));
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
                selected_users set_selected_users is_disabled is_highlighted is_single_select=true />
            </div>
        </div>
        <NextTurnButtons onclick_next_role />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // create user list for test
    fn create_user_vec_for_test() -> Vec<Player>{
        vec![
            Player {
                id: "001".to_string(),
                name: "User1".to_string(),
                comment: "".to_string(),
                is_guest: false,
                role: HashSet::new(),
                additional_role: HashSet::new(),
                choosed_by: HashSet::new(),
                history_by: Vec::new(),
                is_alive: true,
                was_killed: false,
            },
            Player {
                id: "002".to_string(),
                name: "User2".to_string(),
                comment: "".to_string(),
                is_guest: false,
                role: HashSet::new(),
                additional_role: HashSet::new(),
                choosed_by: HashSet::new(),
                history_by: Vec::new(),
                is_alive: true,
                was_killed: false,
            },
            Player {
                id: "003".to_string(),
                name: "User3".to_string(),
                comment: "".to_string(),
                is_guest: false,
                role: HashSet::new(),
                additional_role: HashSet::new(),
                choosed_by: HashSet::new(),
                history_by: Vec::new(),
                is_alive: true,
                was_killed: false,
            },
        ]
    }

    // write a test that checks if the user is killed by the mafia
    #[test]
    fn test_calculate_night_kills() {
        let mut users = create_user_vec_for_test();

        users[0].role.insert(Role::Mafia(MafiaRole::Mafia));
        users[1].role.insert(Role::Mafia(MafiaRole::Doctor));
        users[2].role.insert(Role::Mafia(MafiaRole::Prostitute));

        users[0].choosed_by.insert(Role::Mafia(MafiaRole::Mafia));
        users[1].choosed_by.insert(Role::Mafia(MafiaRole::Doctor));
        users[2].choosed_by.insert(Role::Mafia(MafiaRole::Prostitute));

        calculate_night_kills(&mut users);

        assert_eq!(users[0].is_alive, false);
        assert_eq!(users[0].was_killed, true);
        assert_eq!(users[1].is_alive, true);
        assert_eq!(users[1].was_killed, false);
        assert_eq!(users[2].is_alive, true);
        assert_eq!(users[2].was_killed, false);
    }

    // write a test that checks if the user is killed by the maniac
    #[test]
    fn test_calculate_night_kills_maniac() {
        let mut users = create_user_vec_for_test();

        users[0].role.insert(Role::Mafia(MafiaRole::Maniac));
        users[1].role.insert(Role::Mafia(MafiaRole::Doctor));
        users[2].role.insert(Role::Mafia(MafiaRole::Prostitute));

        users[0].choosed_by.insert(Role::Mafia(MafiaRole::Maniac));
        users[1].choosed_by.insert(Role::Mafia(MafiaRole::Doctor));
        users[2].choosed_by.insert(Role::Mafia(MafiaRole::Prostitute));

        calculate_night_kills(&mut users);

        assert_eq!(users[0].is_alive, false);
        assert_eq!(users[0].was_killed, true);
        assert_eq!(users[1].is_alive, true);
        assert_eq!(users[1].was_killed, false);
        assert_eq!(users[2].is_alive, true);
        assert_eq!(users[2].was_killed, false);
    }

    // write a test that checks if the user is saved by the prostitute
    #[test]
    fn test_calculate_night_kills_check_prostitute_save() {
        let mut users = create_user_vec_for_test();

        users[0].role.insert(Role::Mafia(MafiaRole::Mafia));
        users[1].role.insert(Role::Mafia(MafiaRole::Doctor));
        users[2].role.insert(Role::Mafia(MafiaRole::Prostitute));

        users[1].choosed_by.insert(Role::Mafia(MafiaRole::Mafia));
        users[1].choosed_by.insert(Role::Mafia(MafiaRole::Prostitute));

        calculate_night_kills(&mut users);
        
        assert_eq!(users[0].is_alive, true);
        assert_eq!(users[0].was_killed, false);
        assert_eq!(users[1].is_alive, true);
        assert_eq!(users[1].was_killed, false);
        assert_eq!(users[2].is_alive, true);
        assert_eq!(users[2].was_killed, false);
    }

    // write a test that checks if the mafia is killed prostitute
    #[test]
    fn test_calculate_night_kills_with_check_prostitute_killed_by_mafia() {
        let mut users = create_user_vec_for_test();

        users[0].role.insert(Role::Mafia(MafiaRole::Mafia));
        users[1].role.insert(Role::Mafia(MafiaRole::Doctor));
        users[2].role.insert(Role::Mafia(MafiaRole::Prostitute));

        users[2].choosed_by.insert(Role::Mafia(MafiaRole::Mafia));
        users[1].choosed_by.insert(Role::Mafia(MafiaRole::Prostitute));

        calculate_night_kills(&mut users);
        
        assert_eq!(users[0].is_alive, true);
        assert_eq!(users[0].was_killed, false);
        assert_eq!(users[1].is_alive, false);
        assert_eq!(users[1].was_killed, true);
        assert_eq!(users[2].is_alive, false);
        assert_eq!(users[2].was_killed, true);
    }

    // write a test that checks if the mafia is killed prostitute with mafia
    #[test]
    fn test_calculate_night_kills_with_check_prostitute_saved_mafia() {
        let mut users = create_user_vec_for_test();

        users[0].role.insert(Role::Mafia(MafiaRole::Mafia));
        users[1].role.insert(Role::Mafia(MafiaRole::Doctor));
        users[2].role.insert(Role::Mafia(MafiaRole::Prostitute));

        users[2].choosed_by.insert(Role::Mafia(MafiaRole::Mafia));
        users[0].choosed_by.insert(Role::Mafia(MafiaRole::Prostitute));

        calculate_night_kills(&mut users);
        
        assert_eq!(users[0].is_alive, true);
        assert_eq!(users[0].was_killed, false);
        assert_eq!(users[1].is_alive, true);
        assert_eq!(users[1].was_killed, false);
        assert_eq!(users[2].is_alive, false);
        assert_eq!(users[2].was_killed, true);
    }
}