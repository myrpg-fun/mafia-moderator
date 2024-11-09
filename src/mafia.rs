use std::collections::HashSet;

use leptos::*;
use crate::user::*;
use crate::roles::*;
use crate::GameState;
use crate::MafiaContext;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MafiaRole {
    None,
    Citizen,
    Mafia,
    Doctor,
    Detective,
    Maniac,
    Prostitute,
}

pub const MAFIA_ROLES: [RoleInfo; 6] = [
    RoleInfo::Icon(IconRoleInfo{
        role: Role::Mafia(MafiaRole::Citizen),
        role_icon: "✋",
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Mafia),
        check_role: None,
        role_name: "Мафия",
        role_name_color: "text-red-950",
        role_icon: "🔫",
        prepare_description: "Выберите игроков Мафии",
        night_description: "Кого убьет Мафия?",
        targeting_rules: NightTargetingRules::Anyone,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Detective),
        check_role: None,
        role_name: "Детектив",
        role_name_color: "text-blue-950",
        role_icon: "🔍",
        prepare_description: "Выберите игрока Детектива",
        night_description: "Кого проверит Детектив?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Maniac),
        check_role: None,
        role_name: "Маньяк",
        role_name_color: "text-gray-950",
        role_icon: "🔪",
        prepare_description: "Выберите игрока Маньяка",
        night_description: "Кого убьет Маньяк?",
        targeting_rules: NightTargetingRules::Anyone,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Doctor),
        check_role: None,
        role_name: "Доктор",
        role_name_color: "text-green-950",
        role_icon: "🚑",
        prepare_description: "Выберите игрока Доктора",
        night_description: "Кого спасет Доктор?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Mafia(MafiaRole::Prostitute),
        check_role: None,
        role_name: "Проститутка",
        role_name_color: "text-purple-950",
        role_icon: "💋",
        prepare_description: "Выберите игрока Проститутку",
        night_description: "К кому зайдет Проститутка?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
];

#[derive(Clone, Copy, Debug)]
pub enum MafiaGameState<'a> {
    SetupRoles(&'a RoleInfo),
    Day,
    Night(&'a RoleInfo),
    End,
}

fn get_next_prepare_role(role: Role) -> Option<&'static RoleInfo> {
    let role_index = MAFIA_ROLES.iter().position(|r| *r == role).unwrap();

    MAFIA_ROLES.get(role_index.wrapping_add(1))
}

fn get_next_night_role(role: Role) -> Option<&'static RoleInfo> {
    let role_index = MAFIA_ROLES.iter().position(|r| *r == role).unwrap();

    MAFIA_ROLES.get(role_index.wrapping_add(1))
}

#[component]
pub fn MafiaGameView() -> impl IntoView {
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_mafia_context =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let game_state_view = move || match mafia_context.get().game_state {
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
            MafiaGameState::End => view! {
                <div>"Конец игры"</div>
            }
            .into_view(),
        },
        _ => view! {
            <div>"Ошибка"</div>
        }.into_view()
    };

    view! {
        <div class="relative flex flex-col gap-4 w-full h-full">
            <h1 class="text-lg relative w-full text-left">
                "Мафия"
                <button
                    class="absolute text-sm right-0 top-0 px-2 py-1 bg-gray-200 rounded-full"
                    on:click=move |_| {
                        if window().confirm_with_message("Вернуться в главное меню?").expect("REASON") {
                            set_mafia_context.set(MafiaContext::default());
                        }
                    }>
                    "Главное меню"
                </button>
            </h1>
            {game_state_view}
        </div>
    }
}

#[component]
fn UserRow(user: User) -> impl IntoView {
    let users = use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    view! {
        <div class="flex gap-2">
            <div class="flex-1 px-3 py-1 text-base bg-gray-200 rounded-full">{user.name.clone()}</div>
            <button class="text-lg"
                on:click=move |_| {
                    users.update(|ctx| ctx.users.retain(|u| *u != user));
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
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_mafia_context =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let users = move || mafia_context.get().users;
    let role = role.get_role();

    view! {
        <div class="grid grid-cols-3 gap-1">
            <For
                each=users
                key=|user| user.name.clone()
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
                                set_mafia_context.update(|ctx| {
                                    if let Some(user) = ctx.users.iter_mut().find(|u| **u == user) {
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

#[component]
fn UserSelectRole(
    user: User,
    is_selected: impl Fn(&User) -> bool + 'static,
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
                main_class.push_str(if is_selected(&user_clone) {
                    " bg-blue-300"
                } else {
                    " bg-gray-200"
                });
                main_class.push_str(if killed {
                    " opacity-20"
                } else if disabled {
                    " opacity-60"
                } else {
                    ""
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
            <div class="flex items-baseline justify-center flex-wrap">{user.name}</div>
            <UserRoleNames role=user.role />
            <UserHistory hystory=history current=choosed />
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
                                        <UserRoleIcon role=role is_hystory=true />
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
                        <UserRoleIcon role=role is_hystory=false />
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
                    class=move || if is_hystory {"text-xs opacity-60 w-4 h-4"} else {"text-xs rounded-md bg-white w-4 h-4"}
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
                    class=move || {
                        let mut class = "text-xs opacity-50 ".to_string();
                        class.push_str(role_info.get_role_name_color());
                        class
                    }
                >
                    {role_info.get_role_name()}
                </div>
            }
        }).into_view()
}

#[component]
fn TurnButtons<'a>(role_info: &'a RoleInfo) -> impl IntoView {
    let set_game_state = use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_context_history = use_context::<WriteSignal<Vec<MafiaContext>>>().expect("MafiaContext history not found");

    let role = role_info.get_role();
    let onclick_next_role = move |_| {
        set_game_state.update(|ctx| {
            set_context_history.update(|history| history.push(ctx.clone()));

            match get_next_prepare_role(role) {
                Some(role_info) => ctx.game_state = GameState::Mafia(MafiaGameState::SetupRoles(role_info)),
                None => {
                    ctx.round = 0;
                    ctx.game_state = GameState::Mafia(MafiaGameState::Day);
                }
            }
        });
    };

    let onclick_prev_role = move |_| {
        set_context_history.update(|history| {
            if let Some(prev_ctx) = history.pop() {
                set_game_state.set(prev_ctx);
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
    is_disabled: impl Fn(&User) -> bool + 'static,
    is_highlighted: impl Fn(&User) -> bool + 'static,
    is_killed: impl Fn(&User) -> bool + 'static,
    is_single_select: bool,
) -> impl IntoView {
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");

    let users = move || mafia_context.get().users;
    let is_selected = move |user: &User| selected_users.get().contains(&user.name);

    view! {
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
                            is_selected=is_selected
                            killed=killed
                            on:click=move |_| {
                                set_selected_users.update(|selected_users| {
                                    if selected_users.contains(&user.name) {
                                        selected_users.remove(&user.name);
                                    } else {
                                        if is_single_select {
                                            selected_users.clear();
                                        }
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
    let set_game_state =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());

    let onclick_next_role = move || {
        let selected_users = selected_users.get();
        set_game_state.update(|state| {
            let round = state.round;

            clear_was_killed(&mut state.users);
            clear_choosed_by(&mut state.users, round);

            state.users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.name) {
                    let mut citizen_history = HashSet::new();
                    citizen_history.insert(Role::Mafia(MafiaRole::Citizen));
                    u.history_by.push((round+1, citizen_history));
                    u.is_alive = false;
                    u.was_killed = true;
                }
            });

            let mut next_role = Some(MAFIA_ROLES.first().unwrap());
            loop {
                if let Some(check_role) = next_role {
                    if is_role_alive(check_role.get_role(), &state.users) {
                        state.game_state = GameState::Mafia(MafiaGameState::Night(check_role));
                        break;
                    }
                    next_role = get_next_night_role(check_role.get_role());
                } else {
                    state.game_state = GameState::SetupNames;
                    break;
                }
            }
        });
    };

    view! {
        <h2>"Кого мирные жители убъют этим Днем?"</h2>
        <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
            <div class="flex-1"></div>
            <div class="flex flex-col gap-1 w-full">
                <SelectUsersForVote 
                    selected_users 
                    set_selected_users 
                    is_disabled=move |user: &User| !user.is_alive 
                    is_highlighted=move |_| false 
                    is_killed=move |user: &User| !user.is_alive && !user.was_killed
                    is_single_select=false />
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
    let set_context_history = use_context::<WriteSignal<Vec<MafiaContext>>>().expect("MafiaContext history not found");

    let onclick_prev_role = move |_| {
        set_context_history.update(|history| {
            if let Some(prev_ctx) = history.pop() {
                let set_game_state = use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");
                set_game_state.set(prev_ctx);
            }
        });
    };

    let onclick_next = move |_| {
        let ctx = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found").get();

        set_context_history.update(|history| history.push(ctx.clone()));

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

fn is_role_alive(role: Role, users: &[User]) -> bool {
    users.iter().any(|u| u.role.contains(&role) && u.is_alive)
}

fn get_next_night_alive_role(role: Role, users: &[User]) -> Option<&'static RoleInfo> {
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

fn clear_choosed_by(users: &mut [User], round: usize) {
    for user in users.iter_mut() {
        // filter Citizen role
        let choosed_by = user.choosed_by.clone();//.into_iter().filter(|r| *r != Role::Mafia(MafiaRole::Citizen)).collect::<HashSet<_>>();

        if !choosed_by.is_empty(){
            user.history_by.push((round, choosed_by));
        }
        user.choosed_by.clear();
    }
}

fn clear_was_killed(users: &mut [User]) {
    for user in users.iter_mut() {
        user.was_killed = false;
    }
}

fn calculate_night_kills(users: &mut [User]) {
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
        if killed_by_mafia.role.contains(&Role::Mafia(MafiaRole::Prostitute)) {
            let saved_by_prostitute = alive_users
                .iter_mut()
                .find(|u| u.choosed_by.contains(&Role::Mafia(MafiaRole::Prostitute)));

            if let Some(saved_by_prostitute) = saved_by_prostitute {
                if !saved_by_prostitute.role.contains(&Role::Mafia(MafiaRole::Mafia)) {
                    saved_by_prostitute.is_alive = false;
                    saved_by_prostitute.was_killed = true;
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
        if killed_by_maniac.role.contains(&Role::Mafia(MafiaRole::Prostitute)) {
            let saved_by_prostitute = alive_users
                .iter_mut()
                .find(|u| u.choosed_by.contains(&Role::Mafia(MafiaRole::Prostitute)));

            if let Some(saved_by_prostitute) = saved_by_prostitute {
                if !saved_by_prostitute.role.contains(&Role::Mafia(MafiaRole::Maniac)) {
                    saved_by_prostitute.is_alive = false;
                    saved_by_prostitute.was_killed = true;
                }
            }
        }
    }
}

// check is user history contains role
fn check_user_history_for_role(user: &User, role: &Role) -> bool {
    user.history_by.iter().any(|(_, roles)| {
        roles.contains(role)
    })
}

#[component]
fn NightTurn(role_info: &'static RoleInfo) -> impl IntoView {
    let set_game_state =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");
    let night_description = role_info.get_night_description();

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());
    let role = role_info.get_role();

    let onclick_next_role = move || {
        let selected_user = selected_users.get();
        set_game_state.update(|state| {
            state.users.iter_mut().for_each(|u| {
                if selected_user.contains(&u.name) {
                    u.choosed_by.insert(role);
                }
            });
        });

        set_game_state.update(|state| {
            let next_role = get_next_night_alive_role(role, &state.users);

            match next_role {
                Some(next_role) => {
                    state.game_state = GameState::Mafia(MafiaGameState::Night(next_role));
                }
                None => {
                    calculate_night_kills(&mut state.users);
                    state.round += 1;
                    state.game_state = GameState::Mafia(MafiaGameState::Day);
                }
            }
        });
    };

    let role_targeting_rules = role_info.get_targeting_rules();
    let is_disabled = move |user: &User| {
        !user.is_alive || //(!role_info_clone.get_can_choose_twice() && user.hystory_by.contains(&role))
        match role_targeting_rules {
            NightTargetingRules::NotTheSame => check_user_history_for_role(&user, &role),
            _ => false,
        }
    };

    let is_highlighted = move |user: &User| {
        role_info.get_role() == Role::Mafia(MafiaRole::Detective) && user.role.contains(&&Role::Mafia(MafiaRole::Mafia))
    };

    view! {
        <h2>
            {night_description}
        </h2>
        <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
            <div class="flex-1"></div>
            <div class="flex flex-col gap-1 w-full">
                <SelectUsersForVote 
                is_killed=move |user: &User| !user.is_alive && !user.was_killed
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
    fn create_user_vec_for_test() -> Vec<User>{
        vec![
            User {
                name: "User1".to_string(),
                role: HashSet::new(),
                additional_role: HashSet::new(),
                choosed_by: HashSet::new(),
                history_by: Vec::new(),
                is_alive: true,
                was_killed: false,
            },
            User {
                name: "User2".to_string(),
                role: HashSet::new(),
                additional_role: HashSet::new(),
                choosed_by: HashSet::new(),
                history_by: Vec::new(),
                is_alive: true,
                was_killed: false,
            },
            User {
                name: "User3".to_string(),
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