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
    DireWolf,
    //    LoneWolf,
    WolfCub,
    Minion,
    Cursed,
    Bodyguard,
    Priest,
    Witch,
    WitchPoison,
    ToughGuy,
    Seer,
    ApprenticeSeer,
    AuraSeer,
    Spellcaster,
    Lycan,
    Mayor,
    Hunter,
    Huntress,
    ParanormalInvestigator,
    Prince,
    Diseased,
    Mason,
    Lovers,
    Doppelganger,
    AlphaWolf,
    MadBomber,
    Revealer,
}

pub const WEREWOLF_ROLES: [RoleInfo; 13] = [
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
        prepare_description: "Просыпаются Werewolf и Minion (выберите только игроков Werewolf)",
        night_description: "Кого убьют Werewolf?",
        targeting_rules: NightTargetingRules::Anyone,
    }),
    RoleInfo::Additional(AdditionalRoleInfo {
        role: Role::Werewolf(WerewolfRole::DireWolf),
        role_icon: "💙",
        prepare_description: "Dire Wolf открывает глаза и выбирает в кого он влюблен (выберите Dire Wolf и второго игрока)",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Cursed),
        role_name: "Cursed",
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
        role_name: "Tough Guy",
        role_name_color: "text-blue-950",
        prepare_description: "Выберите игрока Tough Guy",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Mayor),
        role_name: "Mayor",
        role_name_color: "text-blue-950",
        prepare_description: "Выберите игрока Mayor",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Lycan),
        role_name: "Lycan",
        role_name_color: "text-blue-950",
        prepare_description: "Выберите игроков Lycan",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Mason),
        role_name: "Mason",
        role_name_color: "text-blue-950",
        prepare_description: "Выберите игроков Mason",
    }),
    RoleInfo::Passive(PassiveRoleInfo {
        role: Role::Werewolf(WerewolfRole::Lovers),
        role_name: "Lovers",
        role_name_color: "text-blue-950",
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
        <h1 class="text-lg relative w-full text-left">
            "Werewolf"
            <button
                class="absolute text-sm right-0 top-0 px-2 py-1 bg-gray-200 rounded-full"
                on:click=move |_| set_mafia_context.set(MafiaContext::default())>
                "Главное меню"
            </button>
        </h1>
        {game_state_view}
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
        <div class="flex-1 flex flex-col gap-4 w-full">
            <SetupRolesHeader role=role />
            <div class="flex-1 flex flex-col justify-end gap-1 w-full overflow-auto">
                <SelectUserForRole role_info=role />
            </div>
            <TurnButtons role_info=role />
        </div>
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
                            is_selected=move |u| {
                                match role_info{
                                    RoleInfo::Night(_) => u.role == role,
                                    _ => u.additional_role.contains(&role)
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
                                            _ => {
                                                match user.role{
                                                    Role::None => {
                                                        user.role = role;
                                                    }
                                                    _ => {
                                                        if user.role == role {
                                                            user.role = Role::None;
                                                        }
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
            {move || if !&user.is_alive && !&user.choosed_by.is_empty() {
                view! {
                    <div class="absolute text-[0.5rem] right-2 top-[0.15rem]">"❌"</div>
                }.into_view()
            }else{
                "".into_view()
            }}
            <UserRoleName role=user.role />
            <div class="flex items-baseline justify-center flex-wrap">{user.name} <UserAdditionalRoles roles=user.additional_role /></div>
            <UserHistory hystory=history current=choosed />
        </button>
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
        <div class="flex gap-1">
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
                if let Some(user) = state.users.iter_mut().find(|u| u.name == selected_user) {
                    user.choosed_by
                        .insert(Role::Werewolf(WerewolfRole::Villager));
                    user.is_alive = false;
                }
            })
        }

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
        <div class="flex-1 flex flex-col gap-4 w-full">
            <h2>"Кого мирные жители убъют этим Днем?"</h2>
            <div class="flex-1 flex flex-col justify-end gap-1 w-full overflow-auto">
                <SelectUserForVote selected_user set_selected_user is_disabled=move |user: &User| !user.is_alive />
            </div>
            <NextTurnButtons onclick_next_role />
        </div>
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
    users.iter().any(|u| u.role == role && u.is_alive)
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

fn calculate_night_kills(users: &mut [User]) {
    // Mafia killed choosed user if he is not protected by doctor or prostitute
    let mut alive_users = users.iter_mut().filter(|u| u.is_alive).collect::<Vec<_>>();

    let killed_by_mafia = alive_users.iter_mut().find(|u| {
        u.choosed_by
            .contains(&Role::Werewolf(WerewolfRole::Werewolf))
            && !u
                .choosed_by
                .contains(&Role::Werewolf(WerewolfRole::Bodyguard))
    });
}

#[component]
fn NightTurn(role_info: &'static RoleInfo) -> impl IntoView {
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_game_state =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");
    let night_description = role_info.get_night_description();

    let users = move || mafia_context.get().users;

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
            let next_role = get_next_night_alive_role(role_info, &state.users);

            match next_role {
                Some(next_role) => {
                    state.game_state = GameState::Werewolf(WerewolfGameState::Night(next_role));
                }
                None => {
                    calculate_night_kills(&mut state.users);
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

    view! {
        <div class="flex-1 flex flex-col gap-4 w-full">
            <h2>
                {night_description}
            </h2>
            <div class="flex-1 flex flex-col justify-end gap-1 w-full overflow-auto">
                <SelectUserForVote selected_user set_selected_user is_disabled />
            </div>
            <NextTurnButtons onclick_next_role />
        </div>
    }
}
