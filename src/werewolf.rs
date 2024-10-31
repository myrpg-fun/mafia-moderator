use itertools::Itertools;
use serde::de;
use std::collections::HashSet;

use crate::roles::*;
use crate::user::*;
use crate::GameState;
use crate::MafiaContext;
use leptos::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
    Witch,
    WitchPoison,
    ToughGuy,
    Seer,
    Lycan,
    Mayor,
    Hunter,
    Huntress,
    //    ParanormalInvestigator,
    Prince,
    Diseased,
    Mason,
    Lovers,
    // Doppelganger,
    // AlphaWolf,
    // MadBomber,
    //*** TODO:
    // Revealer,
    // ApprenticeSeer,
    // AuraSeer,
    // Spellcaster,
}

pub const WEREWOLF_ROLES: [RoleInfo; 14] = [
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Priest),
        check_role: None,
        role_icon: "🙏",
        role_name: "Priest",
        role_name_color: "text-green-950",
        prepare_description: "Выберите игрока Priest",
        night_description: "Кого освятит Priest?",
        targeting_rules: NightTargetingRules::OnlyOne,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Bodyguard),
        check_role: None,
        role_name: "Bodyguard",
        role_name_color: "text-green-950",
        role_icon: "🛡️",
        prepare_description: "Выберите игрока Bodyguard",
        night_description: "Кого защитит Bodyguard?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Werewolf),
        check_role: None,
        role_name: "Werewolf",
        role_name_color: "text-red-950",
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
        role_name_color: "text-red-950",
        prepare_description: "Выберите игрока Minion",
    }),
    RoleInfo::Additional(AdditionalRoleInfo {
        role: Role::Werewolf(WerewolfRole::DireWolf),
        role_icon: "💙",
        prepare_description: "Поставте сердечки игроку Dire Wolf и в кого он влюблен",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Cursed),
        role_name: "Cursed",
        role_icon: "",
        additional_role: None,
        role_name_color: "text-purple-950",
        prepare_description: "Выберите игрока Cursed",
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Witch),
        check_role: None,
        role_icon: "🌿",
        role_name: "Witch",
        role_name_color: "text-green-950",
        prepare_description: "Выберите игрока Witch",
        night_description: "Кого вылечит Witch?",
        targeting_rules: NightTargetingRules::OnlyOne,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::WitchPoison),
        check_role: Some(Role::Werewolf(WerewolfRole::Witch)),
        role_icon: "☠️",
        role_name: "Witch",
        role_name_color: "text-green-950",
        prepare_description: "",
        night_description: "Кого отравит Witch?",
        targeting_rules: NightTargetingRules::OnlyOne,
    }),
    RoleInfo::Night(NightRoleInfo {
        role: Role::Werewolf(WerewolfRole::Seer),
        check_role: None,
        role_name: "Seer",
        role_name_color: "text-blue-950",
        role_icon: "🔍",
        prepare_description: "Выберите игрока Seer",
        night_description: "Кого проверит Seer?",
        targeting_rules: NightTargetingRules::NotTheSame,
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::ToughGuy),
        role_name: "ToughGuy",
        role_icon: "💛",
        additional_role: Some(Role::Werewolf(WerewolfRole::ToughGuy)),
        role_name_color: "text-blue-950",
        prepare_description: "Выберите игрока ToughGuy",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Mayor),
        role_name: "Mayor",
        role_icon: "🎖️",
        additional_role: Some(Role::Werewolf(WerewolfRole::Mayor)),
        role_name_color: "text-blue-950",
        prepare_description: "Выберите игрока Mayor",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Lycan),
        role_name: "Lycan",
        role_icon: "🐺",
        additional_role: None,
        role_name_color: "text-blue-950",
        prepare_description: "Выберите игрока Lycan",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Mason),
        role_name: "Mason",
        role_icon: "",
        additional_role: None,
        role_name_color: "text-purple-950",
        prepare_description: "Выберите игроков Mason",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Lovers),
        role_name: "Lovers",
        role_icon: "❤️",
        additional_role: None,
        role_name_color: "text-purple-950",
        prepare_description: "Выберите игроков Lovers",
    }),
];

#[derive(Clone, Copy, Debug)]
pub enum WerewolfGameState<'a> {
    SetupRoles(&'a RoleInfo),
    Day,
    Night(&'a RoleInfo),
    End,
}

fn get_next_prepare_role(role: Role) -> Option<&'static RoleInfo> {
    let role_index = WEREWOLF_ROLES
        .iter()
        .position(|r| r.get_role() == role)
        .unwrap();

    let next_role = WEREWOLF_ROLES
        .iter()
        .skip(role_index.wrapping_add(1))
        .find(|r| !r.get_prepare_description().is_empty());

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

#[component]
pub fn WerewolfGameView() -> impl IntoView {
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_mafia_context =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let game_state_view = move || match mafia_context.get().game_state {
        GameState::Werewolf(game_state) => match game_state {
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
                <SelectUserForRole role_info=role />
            </div>
        </div>
        <TurnButtons role_info=role />
    }
}

#[component]
fn SetupRolesHeader<'a>(role: &'a RoleInfo) -> impl IntoView {
    let len = move || {
        WEREWOLF_ROLES
            .iter()
            .filter(|r| !r.get_prepare_description().is_empty())
            .count()
    };

    let role_clone = role.clone();
    let index = move || {
        WEREWOLF_ROLES
            .iter()
            .filter(|r| !r.get_prepare_description().is_empty())
            .position(|r| r == &role_clone)
            .unwrap_or(0)
            + 1
    };

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
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_mafia_context =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let users = move || mafia_context.get().users;
    let role = role_info.get_role();
    let role_info = role_info.clone();

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
                            highlighted=false
                            is_selected=move |u| {
                                match role_info{
                                    RoleInfo::Additional(_) => u.additional_role.contains(&role),
                                    _ => u.role.contains(&role),
                                }
                            }
                            on:click=move |_| {
                                set_mafia_context.update(|ctx| {
                                    if let Some(user) = ctx.users.iter_mut().find(|u| **u == user) {
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
    highlighted: bool,
) -> impl IntoView {
    let history = user.history_by.clone();
    let choosed = user.choosed_by.clone();
    let user_clone = user.clone();

    view! {
        <button
            disabled=disabled
            class=move || {
                let mut main_class = "relative flex-1 px-3 py-1 text-sm rounded-2xl flex flex-col items-center justify-end".to_string();
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
            <div class="flex items-baseline justify-center flex-wrap">{user.name} <UserAdditionalRoles roles=user.additional_role /></div>
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
fn UserHistory(hystory: HashSet<Role>, current: HashSet<Role>) -> impl IntoView {
    view! {
        <div class="flex gap-1 h-4">
            {
                hystory.iter().map(|role| {
                    let role = *role;

                    view!{
                        <UserRoleIcon role=role is_history=UserRoleIconType::History />
                    }
                }).collect::<Vec<_>>().into_view()
            }
            {
                current.iter().map(|role| {
                    let role = *role;

                    view!{
                        <UserRoleIcon role=role is_history=UserRoleIconType::Current />
                    }
                })
                .collect::<Vec<_>>().into_view()

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
        .map(|role_info| {
            view! {
                <div
                    class=move || match is_history {
                        UserRoleIconType::History => "text-xs opacity-70 w-4 h-4",
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
                        class=move || {
                            let mut class = "text-xs opacity-50 ".to_string();
                            class.push_str(role_info.get_role_name_color());
                            class
                        }
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
    let set_game_state =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_context_history =
        use_context::<WriteSignal<Vec<MafiaContext>>>().expect("MafiaContext history not found");

    let role = role_info.get_role();
    let onclick_next_role = move |_| {
        set_game_state.update(|ctx| {
            set_context_history.update(|history| history.push(ctx.clone()));

            match get_next_prepare_role(role) {
                Some(role_info) => {
                    ctx.game_state = GameState::Werewolf(WerewolfGameState::SetupRoles(role_info))
                }
                None => {
                    initialize_user_roles(&mut ctx.users);
                    ctx.game_state = GameState::Werewolf(WerewolfGameState::Day);
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
    selected_users: ReadSignal<HashSet<String>>,
    set_selected_users: WriteSignal<HashSet<String>>,
    is_disabled: impl Fn(&User) -> bool + 'static,
    is_highlighted: impl Fn(&User) -> bool + 'static,
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

                    view!{
                        <UserSelectRole
                            user=user.clone()
                            disabled=disabled
                            highlighted=highlighted
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
    let set_game_state =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());

    let onclick_next_role = move || {
        let selected_users = selected_users.get();
        set_game_state.update(|state| {
            clear_was_killed(&mut state.users);

            fn kill_user(user: &mut User) {
                if user
                    .additional_role
                    .contains(&Role::Werewolf(WerewolfRole::ToughGuy))
                {
                    user.additional_role
                        .remove(&Role::Werewolf(WerewolfRole::ToughGuy));
                    return;
                }

                user.is_alive = false;
                user.was_killed = true;
            }

            state.users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.name) {
                    kill_user(u);
                }
            });

            calculate_after_kills(&mut state.users);
        });

        set_game_state.update(|state| {
            clear_choosed_by(&mut state.users);

            let mut next_role = Some(WEREWOLF_ROLES.first().unwrap());
            loop {
                if let Some(check_role) = next_role {
                    if is_role_alive(check_role.get_role(), &state.users) {
                        state.game_state =
                            GameState::Werewolf(WerewolfGameState::Night(check_role));
                        break;
                    }
                    next_role = get_next_night_role(check_role);
                } else {
                    state.game_state = GameState::SetupNames;
                    break;
                }
            }
        });
    };

    view! {
        <h2>"Кого мирные жители убъют этим Днем?"</h2>
        <p class="opacity-50 text-sm">"Если убили Hunter или Mad Bomber, выберите несколько целей"</p>
        <div class="flex-1 flex flex-col relative overflow-auto px-4 -mx-4">
            <div class="flex-1"></div>
            <div class="flex flex-col gap-1 w-full">
                <SelectUserForVote selected_users set_selected_users
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
    let set_context_history =
        use_context::<WriteSignal<Vec<MafiaContext>>>().expect("MafiaContext history not found");

    let onclick_prev_role = move |_| {
        set_context_history.update(|history| {
            if let Some(prev_ctx) = history.pop() {
                let set_game_state =
                    use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");
                set_game_state.set(prev_ctx);
            }
        });
    };

    let onclick_next = move |_| {
        let ctx = use_context::<ReadSignal<MafiaContext>>()
            .expect("MafiaContext not found")
            .get();

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

fn get_next_night_alive_role(role_info: &RoleInfo, users: &[User]) -> Option<&'static RoleInfo> {
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

    fn is_user_protected(user: &User) -> bool {
        user.choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Bodyguard))
            || user
                .choosed_by
                .contains(&Role::Werewolf(WerewolfRole::Witch))
    }

    fn kill_user(user: &mut User, check_protection: &[Role]) {
        for role in check_protection {
            if user.additional_role.contains(role) {
                user.additional_role.remove(role);
                return;
            }
        }

        user.is_alive = false;
        user.was_killed = true;
    }

    for user in alive_users.iter_mut() {
        // Priest check
        if user
            .choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Priest))
        {
            if user.role.contains(&Role::Werewolf(WerewolfRole::Werewolf))
                && !is_user_protected(user)
            {
                kill_user(user, &[Role::Werewolf(WerewolfRole::ToughGuy)]);
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
            if !is_user_protected(user) {
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
            kill_user(user, &[]);
        }
    }
}

fn initialize_user_roles(users: &mut [User]) {
    // init additional roles
}

fn calculate_after_kills(users: &mut [User]) {
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
            u.is_alive = false;
            u.was_killed = true;
            u.choosed_by.insert(Role::Werewolf(WerewolfRole::Villager));
        }
    }
}

#[component]
fn NightTurn(role_info: &'static RoleInfo) -> impl IntoView {
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_game_state =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");
    let night_description = role_info.get_night_description();

    let users = move || mafia_context.get().users;

    let (selected_users, set_selected_users) = create_signal::<HashSet<String>>(HashSet::new());
    let role = role_info.get_role();

    let onclick_next_role = move || {
        let selected_users = selected_users.get();
        set_game_state.update(|state| {
            state.users.iter_mut().for_each(|u| {
                if selected_users.contains(&u.name) {
                    u.choosed_by.insert(role);
                }
            })
        });

        set_game_state.update(|state| {
            let next_role = get_next_night_alive_role(role_info, &state.users);

            match next_role {
                Some(next_role) => {
                    state.game_state = GameState::Werewolf(WerewolfGameState::Night(next_role));
                }
                None => {
                    calculate_night_kills(&mut state.users);
                    calculate_after_kills(&mut state.users);
                    state.game_state = GameState::Werewolf(WerewolfGameState::Day);
                }
            }
        });
    };

    let role_targeting_rules = role_info.get_targeting_rules();
    let is_fully_disabled = match role_targeting_rules {
        NightTargetingRules::OnlyOne => users().iter().any(|u| u.history_by.contains(&role)),
        _ => false,
    };
    let is_disabled = move |user: &User| {
        is_fully_disabled
            || !user.is_alive
            || match role_targeting_rules {
                NightTargetingRules::NotTheSame => user.history_by.contains(&role),
                _ => false,
            }
    };

    let role_info = role_info.clone();
    let is_highlighted = move |user: &User| {
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
            <SelectUserForVote selected_users set_selected_users is_disabled is_highlighted />
            </div>
        </div>
        <NextTurnButtons onclick_next_role />
    }
}
