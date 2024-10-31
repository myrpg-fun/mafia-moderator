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

pub const MAFIA_ROLES: [RoleInfo; 5] = [
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
                    on:click=move |_| set_mafia_context.set(MafiaContext::default())>
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
) -> impl IntoView {
    let user_clone = user.clone();
    let history = user.history_by.clone();
    let choosed = user.choosed_by.clone();

    view! {
        <button
            disabled=disabled
            class=move || {
                let mut main_class = "relative flex-1 px-3 py-1 text-sm rounded-2xl flex flex-col items-center justify-end".to_string();
                main_class.push_str(if is_selected(&user_clone) {
                    " bg-blue-300"
                } else {
                    " bg-gray-200"
                });
                main_class.push_str(if disabled {
                    " opacity-50"
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
            <UserRoleNames role=user.role />
            {user.name}
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
fn UserHistory(hystory: HashSet<Role>, current: HashSet<Role>) -> impl IntoView {
    view! {
        <div class="flex gap-1 h-4">
            {
                hystory.iter().map(|role| {
                    let role = *role;

                    view!{
                        <UserRoleIcon role=role is_hystory=true />
                    }
                }).collect::<Vec<_>>().into_view()
            }
            {
                current.iter().map(|role| {
                    let role = *role;

                    view!{
                        <UserRoleIcon role=role is_hystory=false />
                    }
                }).collect::<Vec<_>>().into_view()
            }
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
                    class=move || if is_hystory {"text-xs opacity-70 w-4 h-4"} else {"text-xs rounded-md bg-white w-4 h-4"}
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
fn SelectUserForVote(
    selected_user: ReadSignal<Option<String>>,
    set_selected_user: WriteSignal<Option<String>>,
    is_disabled: impl Fn(&User) -> bool + 'static,
) -> impl IntoView {
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");

    let users = move || mafia_context.get().users;

    view! {
        <div class="grid grid-cols-3 gap-1">
            <For
                each=users
                key=|user| user.name.clone()
                children=move |user| {
                    let disabled = is_disabled(&user);
                    view!{
                        <UserSelectRole
                            user=user.clone()
                            disabled=disabled
                            is_selected= move |u| {
                                let selected_user = selected_user.get();

                                if let Some(selected_user) = selected_user{
                                    selected_user == u.name
                                }else{
                                    false
                                }
                            }
                            on:click=move |_| {
                                let name = Some(user.name.clone());
                                if selected_user.get() == name {
                                    set_selected_user.set(None);
                                }else{
                                    set_selected_user.set(name);
                                }
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

    let (selected_user, set_selected_user) = create_signal::<Option<String>>(None);

    let onclick_next_role = move || {
        let selected_user = selected_user.get();
        if let Some(selected_user) = selected_user {
            set_game_state.update(|state| {
                clear_was_killed(&mut state.users);
            
                if let Some(user) = state.users.iter_mut().find(|u| u.name == selected_user) {
                    user.choosed_by.insert(Role::Mafia(MafiaRole::Citizen));
                    user.is_alive = false;
                    user.was_killed = true;
                }
            })
        }

        set_game_state.update(|state| {
            clear_choosed_by(&mut state.users);

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
                <SelectUserForVote selected_user set_selected_user is_disabled=move |user: &User| !user.is_alive />
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

fn clear_choosed_by(users: &mut [User]) {
    for user in users.iter_mut() {
        user.history_by.extend(user.choosed_by.iter());
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
                if saved_by_prostitute.role.contains(&Role::Mafia(MafiaRole::Mafia)) {
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
                if saved_by_prostitute.role.contains(&Role::Mafia(MafiaRole::Maniac)) {
                    saved_by_prostitute.is_alive = false;
                    saved_by_prostitute.was_killed = true;
                }
            }
        }
    }
}

#[component]
fn NightTurn<'a>(role_info: &'a RoleInfo) -> impl IntoView {
    let set_game_state =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");
    let night_description = role_info.get_night_description();

    let (selected_user, set_selected_user) = create_signal::<Option<String>>(None);
    let role = role_info.get_role();

    let onclick_next_role = move || {
        let selected_user = selected_user.get();
        if let Some(selected_user) = selected_user {
            set_game_state.update(|state| {
                if let Some(user) = state.users.iter_mut().find(|u| u.name == selected_user) {
                    user.choosed_by.insert(role);
                }
            })
        }

        set_game_state.update(|state| {
            let next_role = get_next_night_alive_role(role, &state.users);

            match next_role {
                Some(next_role) => {
                    state.game_state = GameState::Mafia(MafiaGameState::Night(next_role));
                }
                None => {
                    calculate_night_kills(&mut state.users);
                    state.game_state = GameState::Mafia(MafiaGameState::Day);
                }
            }
        });
    };

    let role_targeting_rules = role_info.get_targeting_rules();
    let is_disabled = move |user: &User| {
        !user.is_alive || //(!role_info_clone.get_can_choose_twice() && user.hystory_by.contains(&role))
        match role_targeting_rules {
            NightTargetingRules::NotTheSame => user.history_by.contains(&role),
            _ => false,
        }
    };

    view! {
        <h2>
            {night_description}
        </h2>
        <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
            <div class="flex-1"></div>
            <div class="flex flex-col gap-1 w-full">
                <SelectUserForVote selected_user set_selected_user is_disabled />
            </div>
        </div>
        <NextTurnButtons onclick_next_role />
    }
}
