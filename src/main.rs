use leptos::*;
use std::collections::HashSet;

fn main() {
    mount_to_body(|| {
        view! {
            <Mafia />
        }
    })
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Role {
    None,
    Citizen,
    Mafia,
    Doctor,
    Detective,
    Maniac,
    Prostitute,
}

const CHECK_ROLES: [Role; 3] = [Role::Doctor, Role::Detective, Role::Prostitute];

#[derive(Clone, Debug)]
struct User {
    name: String,
    role: Role,
    additional_role: HashSet<Role>,
    choosed_by: HashSet<Role>,
    hystory_by: HashSet<Role>,
    is_alive: bool,
}

impl User {
    fn new(name: String) -> Self {
        Self {
            name,
            role: Role::None,
            additional_role: HashSet::new(),
            choosed_by: HashSet::new(),
            hystory_by: HashSet::new(),
            is_alive: true,
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.role == other.role
    }
}

fn reset_user_roles(users: &mut Vec<User>) {
    for user in users.iter_mut() {
        user.role = Role::None;
        user.is_alive = true;
        user.additional_role.clear();
        user.choosed_by.clear();
        user.hystory_by.clear();
    }
}

#[derive(Clone, Copy, Debug)]
struct SetupRoles(Role);

#[derive(Clone, Copy, Debug)]
enum GameState {
    // default
    SetupNames,
    SetupRoles(Role),
    Day,
    Night(Role),
    End,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::SetupNames
    }
}

#[derive(Clone, Debug)]
struct MafiaContext {
    users: Vec<User>,
    game_state: GameState,
}

const STORAGE_KEY: &str = "mafia_users";

impl Default for MafiaContext {
    fn default() -> Self {
        let starting_users = window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| {
                storage
                    .get_item(STORAGE_KEY)
                    .ok()
                    .flatten()
                    .and_then(|value| serde_json::from_str::<Vec<String>>(&value).ok())
            })
            .unwrap_or_default()
            .into_iter()
            .map(|name| User::new(name));

        Self {
            users: starting_users.collect(),
            game_state: GameState::SetupNames,
        }
    }
}

const MAFIA_PREPARE_ROLES: [Role; 5] = [
    Role::Mafia,
    Role::Detective,
    Role::Doctor,
    Role::Maniac,
    Role::Prostitute,
];

fn get_next_prepare_role(role: Role) -> Option<Role> {
    let role_index = MAFIA_PREPARE_ROLES.iter().position(|r| r == &role).unwrap();

    MAFIA_PREPARE_ROLES.get(role_index.wrapping_add(1)).copied()
}

fn get_prev_prepare_role(role: Role) -> Option<Role> {
    let role_index = MAFIA_PREPARE_ROLES.iter().position(|r| r == &role).unwrap();

    let prev_index = role_index.checked_sub(1);
    match prev_index {
        Some(prev_index) => MAFIA_PREPARE_ROLES.get(prev_index).copied(),
        None => None,
    }
}

const MAFIA_NIGHT_ROLES: [Role; 5] = [
    Role::Mafia,
    Role::Detective,
    Role::Doctor,
    Role::Maniac,
    Role::Prostitute,
];

fn get_next_night_role(role: Role) -> Option<Role> {
    let role_index = MAFIA_NIGHT_ROLES.iter().position(|r| r == &role).unwrap();

    MAFIA_NIGHT_ROLES.get(role_index.wrapping_add(1)).copied()
}

#[component]
fn Mafia() -> impl IntoView {
    let (mafia_context, set_mafia_context) = create_signal(MafiaContext::default());

    provide_context(mafia_context);
    provide_context(set_mafia_context);

    create_effect(move |_| {
        if let Ok(Some(storage)) = window().local_storage() {
            let user_names = &mafia_context
                .get()
                .users
                .iter()
                .map(|u| u.name.clone())
                .collect::<Vec<_>>();
            let json = serde_json::to_string(user_names).expect("couldn't serialize Users");
            if storage.set_item(STORAGE_KEY, &json).is_err() {
                //log::error!("error while trying to set item in localStorage");
            }
        }
    });

    let game_state_view = move || match mafia_context.get().game_state {
        GameState::SetupNames => view! {
            <SetupUsers />
        }
        .into_view(),
        GameState::SetupRoles(role) => view! {
            <SetupRolesView role={role} />
        }
        .into_view(),

        GameState::Day => view! {
            <DayVote />
        }
        .into_view(),
        GameState::Night(role) => view! {
            <NightTurn role={role} />
        }
        .into_view(),
        GameState::End => view! {
            <div>"Конец игры"</div>
        }
        .into_view(),
    };

    view! {
        <div class="flex flex-col justify-start items-center h-screen gap-4 p-4">
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
fn SetupUsers() -> impl IntoView {
    let mafia_ctx = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_mafia_ctx = use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let (name, set_name) = create_signal("".to_string());

    let users = move || mafia_ctx.get().users;

    view! {
        <div class="flex-1 flex flex-col gap-4 w-full">
            <h2>"Введите имена игроков"</h2>
            <div class="flex-1 flex flex-col gap-1 overflow-auto">
                <For
                    each=users
                    key=|user| user.name.clone()
                    children=move |user| {
                        view!{
                            <UserRow user=user />
                        }
                    }
                />
            </div>
            <form class="flex gap-2 w-full" on:submit=move |ev| {
                ev.prevent_default();

                if name.get().is_empty() {
                    return;
                }

                set_mafia_ctx.update(|ctx| {
                    ctx.users.push(User::new(name.get().clone()));
                    // sort users by name
                    ctx.users.sort_by(|a, b| a.name.cmp(&b.name));
                });

                set_name.set("".to_string());
            }>
                <input
                    class="flex-1 px-3 text-sm py-1 border-gray-200 border rounded-full"
                    placeholder="Имя игрокa"
                    value=name
                    type="text"
                    on:input=move |ev| {
                        set_name.set(event_target_value(&ev));
                    }
                />
                <button
                    class="bg-gray-300 rounded-full px-4 py-1"
                    type="submit"
                >
                    "Добавить"
                </button>
            </form>
            <button
                on:click=move |_| set_mafia_ctx.update(|ctx| {
                    reset_user_roles(&mut ctx.users);
                    ctx.game_state = GameState::SetupRoles(Role::Mafia);
                })
                class="px-4 py-1 bg-gray-200 rounded-full"
            >
                "Начать игру"
            </button>
        </div>
    }
}

#[component]
fn UserRow(user: User) -> impl IntoView {
    let users = use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    view! {
        <div class="flex gap-2">
            <div class="flex-1 px-3 py-1 text-sm bg-gray-200 rounded-full">{user.name.clone()}</div>
            <button
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
fn SetupRolesView(role: Role) -> impl IntoView {
    view! {
        <div class="flex-1 flex flex-col gap-4 w-full">
            <SetupRolesHeader role=role />
            <div class="flex-1 flex flex-col justify-end gap-1 w-full overflow-auto">
                <SelectUserForRole role=role />
            </div>
            <TurnButtons role=role />
        </div>
    }
}

#[component]
fn SetupRolesHeader(role: Role) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <h2>
                {move || match role {
                    Role::Mafia => "Выберите игроков Мафии",
                    Role::Detective => "Выберите игрока Детектива",
                    Role::Doctor => "Выберите игрока Доктора",
                    Role::Maniac => "Выберите игрока Маньяка",
                    Role::Prostitute => "Выберите игрока Проститутку",
                    _ => "",
                }}
            </h2>
        </div>
    }
}

#[component]
fn SelectUserForRole(role: Role) -> impl IntoView {
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_mafia_context =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let users = move || mafia_context.get().users;

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
                            is_selected=move |u| u.role == role
                            on:click=move |_| {
                                set_mafia_context.update(|ctx| {
                                    if let Some(user) = ctx.users.iter_mut().find(|u| **u == user) {
                                        if user.role == role {
                                            user.role = Role::None;
                                        } else {
                                            user.role = role;
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
    let history = user.hystory_by.clone();
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
            {user.name}
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
    view! {
        <div
            class=move || if is_hystory {"text-xs opacity-70 w-4 h-4"} else {"text-xs rounded-md bg-white w-4 h-4"}
        >
            {move || match role {
                Role::Mafia => "🔫",
                Role::Detective => "🔍",
                Role::Doctor => "🚑",
                Role::Maniac => "🔪",
                Role::Prostitute => "💋",
                _ => "",
            }}
        </div>
    }
}

#[component]
fn UserRoleName(role: Role) -> impl IntoView {
    view! {
        <div
            class=move || match role {
                Role::Mafia => "text-xs text-red-950 opacity-50",
                Role::Detective => "text-xs text-blue-950 opacity-50",
                Role::Doctor => "text-xs text-green-950 opacity-50",
                Role::Maniac => "text-xs text-gray-950 opacity-50",
                Role::Prostitute => "text-xs text-purple-950 opacity-50",
                _ => "text-xs opacity-20",
            }
        >
            {move || match role {
                Role::Mafia => "Мафия",
                Role::Detective => "Детектив",
                Role::Doctor => "Доктор",
                Role::Maniac => "Маньяк",
                Role::Prostitute => "Проститутка",
                _ => "Не выбрано",
            }}
        </div>
    }
}

#[component]
fn TurnButtons(role: Role) -> impl IntoView {
    let game_state = use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let onclick_next_role = move |_| {
        game_state.update(|ctx| match get_next_prepare_role(role) {
            Some(role) => ctx.game_state = GameState::SetupRoles(role),
            None => {
                ctx.game_state = GameState::Day;
            }
        });
    };

    let onclick_prev_role = move |_| {
        game_state.update(|ctx| {
            ctx.game_state = match get_prev_prepare_role(role) {
                Some(role) => GameState::SetupRoles(role),
                None => GameState::SetupNames,
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
                    user.choosed_by.insert(Role::Citizen);
                    user.is_alive = false;
                }
            })
        }

        set_game_state.update(|state| {
            clear_choosed_by(&mut state.users);

            let mut next_role = Some(Role::Mafia);
            loop {
                if let Some(check_role) = next_role {
                    if is_role_alive(check_role, &state.users) {
                        state.game_state = GameState::Night(check_role);
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
            <EndDayTurnButtons onclick_next_role />
        </div>
    }
}

#[component]
fn EndDayTurnButtons(onclick_next_role: impl Fn() + 'static) -> impl IntoView {
    view! {
        <div class="flex gap-2 w-full items-center">
            <div></div>
            <button
                class="flex-1 px-4 py-2 text-sm bg-gray-200 rounded-full"
                on:click=move |_| {
                    onclick_next_role()
                }
            >
                "Далее"
            </button>
        </div>
    }
}

fn is_role_alive(role: Role, users: &[User]) -> bool {
    users.iter().any(|u| u.role == role && u.is_alive)
}

fn get_next_night_alive_role(role: Role, users: &[User]) -> Option<Role> {
    let mut next_role = role;
    loop {
        let check_role = get_next_night_role(next_role);
        if let Some(check_role) = check_role {
            if is_role_alive(check_role, users) {
                return Some(check_role);
            }
            next_role = check_role;
        } else {
            return None;
        }
    }
}

fn clear_choosed_by(users: &mut Vec<User>) {
    for user in users.iter_mut() {
        user.hystory_by.extend(user.choosed_by.iter());
        user.choosed_by.clear();
    }
}

fn calculate_night_kills(users: &mut Vec<User>) {
    // Mafia killed choosed user if he is not protected by doctor or prostitute
    let mut alive_users = users.iter_mut().filter(|u| u.is_alive).collect::<Vec<_>>();

    let killed_by_mafia = alive_users.iter_mut().find(|u| {
        u.choosed_by.contains(&Role::Mafia)
            && !u.choosed_by.contains(&Role::Doctor)
            && !u.choosed_by.contains(&Role::Prostitute)
    });

    if let Some(killed_by_mafia) = killed_by_mafia {
        killed_by_mafia.is_alive = false;
        if killed_by_mafia.role == Role::Prostitute {
            let saved_by_prostitute = alive_users
                .iter_mut()
                .find(|u| u.choosed_by.contains(&Role::Prostitute));

            if let Some(saved_by_prostitute) = saved_by_prostitute {
                if saved_by_prostitute.role != Role::Mafia {
                    saved_by_prostitute.is_alive = false;
                }
            }
        }
    }

    // Maniac killed choosed user
    let killed_by_maniac = alive_users.iter_mut().find(|u| {
        u.choosed_by.contains(&Role::Maniac)
            && !u.choosed_by.contains(&Role::Doctor)
            && !u.choosed_by.contains(&Role::Prostitute)
    });

    if let Some(killed_by_maniac) = killed_by_maniac {
        killed_by_maniac.is_alive = false;
        if killed_by_maniac.role == Role::Prostitute {
            let saved_by_prostitute = alive_users
                .iter_mut()
                .find(|u| u.choosed_by.contains(&Role::Prostitute));

            if let Some(saved_by_prostitute) = saved_by_prostitute {
                if saved_by_prostitute.role != Role::Maniac {
                    saved_by_prostitute.is_alive = false;
                }
            }
        }
    }
}

#[component]
fn NightTurn(role: Role) -> impl IntoView {
    let mafia_context = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_game_state =
        use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    let (selected_user, set_selected_user) = create_signal::<Option<String>>(None);

    let onclick_next_role = move || {
        let selected_user = selected_user.get();
        if let Some(selected_user) = selected_user {
            set_game_state.update(|state| {
                if let Some(user) = state.users.iter_mut().find(|u| u.name == selected_user) {
                    user.choosed_by.insert(role);
                }
            })
        }

        let users = &mut mafia_context.get().users;

        let next_role = get_next_night_alive_role(role, users);
        set_game_state.update(|state| {
            match next_role {
                Some(next_role) => state.game_state = GameState::Night(next_role),
                None => {
                    calculate_night_kills(&mut state.users);
                    state.game_state = GameState::Day;
                }
            };
        });
    };

    let is_disabled = move |user: &User| {
        !user.is_alive || (CHECK_ROLES.contains(&role) && user.hystory_by.contains(&role))
    };

    view! {
        <div class="flex-1 flex flex-col gap-4 w-full">
            <h2>
                {move || match role {
                    Role::Mafia => "Кого убьет Мафия?",
                    Role::Detective => "Кого проверит Детектив?",
                    Role::Doctor => "Кого спасет Доктор?",
                    Role::Maniac => "Кого убьет Маньяк?",
                    Role::Prostitute => "К кому зайдет Проститутка?",
                    _ => "",
                }}
            </h2>
            <div class="flex-1 flex flex-col justify-end gap-1 w-full overflow-auto">
                <SelectUserForVote selected_user set_selected_user is_disabled />
            </div>
            <EndDayTurnButtons onclick_next_role />
        </div>
    }
}
