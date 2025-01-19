use std::vec;

use itertools::Itertools;
use leptos::*;
mod mafia;
mod roles;
mod user;
mod werewolf;

use mafia::*;
use roles::Role;
use serde::{Deserialize, Serialize};
use user::*;
use wasm_bindgen::prelude::*;
use web_sys::{console, js_sys, Document, Window};
use werewolf::*;

#[derive(Serialize, Debug)]
pub struct UserLogs {
    id: String,
    name: String,
    is_guest: bool,
    role: String,
    score: u32,
    winner: bool,
    best_player: bool,
    role_index: String,
    role_score: u32,
    rounds: Vec<String>,
}

#[wasm_bindgen(module = "/src/js/GoogleSheetsAPI.js")]
extern "C" {
    pub fn initializeGAPI() -> JsValue; // JsValue <==> Promise
    pub fn handleSigninClick() -> JsValue; // JsValue <==> Promise
    pub fn loadAllUsers() -> JsValue; // JsValue <==> Promise
    pub fn createNewUser(id: &str, name: &str) -> JsValue; // JsValue <==> Promise
    pub fn createNewGameLog(users: &JsValue, isMafia: bool) -> JsValue; // JsValue <==> Promise
}

pub fn rust_create_new_game_log(log_users: Vec<UserLogs>, is_mafia: bool) {
    // Convert the users into a JsValue that JS can understand
    let js_users = serde_wasm_bindgen::to_value(&log_users);

    if let Ok(js_users) = js_users {
        wasm_bindgen_futures::spawn_local(async move {
            let save_log_state =
                use_context::<RwSignal<SaveLogState>>().expect("SaveLogState not found");

            save_log_state.set(SaveLogState(true));

            // Convert JsValue to Promise
            let promise_as_js_value = createNewGameLog(&js_users, is_mafia);
            let promise = js_sys::Promise::from(promise_as_js_value);

            // Convert the promise to a future
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

            save_log_state.set(SaveLogState(false));
        });
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <StartScreen />
        }
    })
}

#[derive(Clone, Copy, Debug)]
pub enum GameState<'a> {
    SetupNames,
    Mafia(MafiaGameState<'a>),
    Werewolf(WerewolfGameState<'a>),
}

impl Serialize for GameState<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            GameState::SetupNames => ("SetupNames", "", Role::None).serialize(serializer),
            GameState::Mafia(state) => match state {
                MafiaGameState::Day => ("Mafia", "Day", Role::None).serialize(serializer),
                MafiaGameState::Night(role) => {
                    ("Mafia", "Night", role.get_role()).serialize(serializer)
                }
                MafiaGameState::SetupRoles(role) => {
                    ("Mafia", "SetupRoles", role.get_role()).serialize(serializer)
                }
            },
            GameState::Werewolf(state) => match state {
                WerewolfGameState::Day => ("Werewolf", "Day", Role::None).serialize(serializer),
                WerewolfGameState::Night(role) => {
                    ("Werewolf", "Night", role.get_role()).serialize(serializer)
                }
                WerewolfGameState::SelectActiveRoles => {
                    ("Werewolf", "SelectActiveRoles", Role::None).serialize(serializer)
                }
                WerewolfGameState::End => ("Werewolf", "End", Role::None).serialize(serializer),
                WerewolfGameState::SetupRoles(role) => {
                    ("Werewolf", "SetupRoles", role.get_role()).serialize(serializer)
                }
            },
        }
    }
}

impl<'de, 'a> Deserialize<'de> for GameState<'a> {
    fn deserialize<D>(deserializer: D) -> Result<GameState<'a>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = serde::de::Deserialize::deserialize(deserializer)?;

        match value {
            ("SetupNames", "", Role::None) => Ok(GameState::SetupNames),
            ("Mafia", "Day", Role::None) => Ok(GameState::Mafia(MafiaGameState::Day)),
            ("Mafia", "Night", role) => MAFIA_ROLES
                .iter()
                .find(|r| r.get_role() == role)
                .map_or(Err(serde::de::Error::custom("invalid value")), |role| {
                    Ok(GameState::Mafia(MafiaGameState::Night(role)))
                }),
            ("Mafia", "SetupRoles", role) => MAFIA_ROLES
                .iter()
                .find(|r| r.get_role() == role)
                .map_or(Err(serde::de::Error::custom("invalid value")), |role| {
                    Ok(GameState::Mafia(MafiaGameState::SetupRoles(role)))
                }),
            ("Werewolf", "Day", Role::None) => Ok(GameState::Werewolf(WerewolfGameState::Day)),
            ("Werewolf", "Night", role) => WEREWOLF_ROLES
                .iter()
                .find(|r| r.get_role() == role)
                .map_or(Err(serde::de::Error::custom("invalid value")), |role| {
                    Ok(GameState::Werewolf(WerewolfGameState::Night(role)))
                }),
            ("Werewolf", "SelectActiveRoles", Role::None) => {
                Ok(GameState::Werewolf(WerewolfGameState::SelectActiveRoles))
            }
            ("Werewolf", "End", Role::None) => Ok(GameState::Werewolf(WerewolfGameState::End)),
            ("Werewolf", "SetupRoles", role) => WEREWOLF_ROLES
                .iter()
                .find(|r| r.get_role() == role)
                .map_or(Err(serde::de::Error::custom("invalid value")), |role| {
                    Ok(GameState::Werewolf(WerewolfGameState::SetupRoles(role)))
                }),
            _ => Err(serde::de::Error::custom("invalid value")),
        }
    }
}

impl Default for GameState<'_> {
    fn default() -> Self {
        GameState::SetupNames
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GameContextHistory {
    users: Vec<Player>,
    round: usize,
    game_state: GameState<'static>,
}

#[derive(Clone, Debug)]
struct GameContext {
    users: RwSignal<Vec<Player>>,
    round: RwSignal<usize>,
    game_state: RwSignal<GameState<'static>>,
}

const STORAGE_LAST_STATE: &str = "last_state";

impl Default for GameContext {
    fn default() -> Self {
        // load from local storage
        let local_storage = window().local_storage().ok().flatten();
        let starting_users = local_storage.and_then(|storage| {
            storage
                .get_item(STORAGE_LAST_STATE)
                .ok()
                .flatten()
                .and_then(|value| serde_json::from_str::<GameContextHistory>(&value).ok())
        });

        if let Some(history) = starting_users {
            return Self {
                users: create_rw_signal(history.users),
                round: create_rw_signal(history.round),
                game_state: create_rw_signal(history.game_state),
            };
        }

        Self {
            users: create_rw_signal(vec![]),
            round: create_rw_signal(0),
            game_state: create_rw_signal(GameState::SetupNames),
        }
    }
}

impl GameContext {
    pub fn store_context_to_local_storage(&self) {
        let context = self.get_history();

        let json = serde_json::to_string(&context).expect("couldn't serialize GameContext");

        if let Ok(Some(storage)) = window().local_storage() {
            if storage.set_item(STORAGE_LAST_STATE, &json).is_err() {
                //log::error!("error while trying to set item in localStorage");
            }
        }
    }

    pub fn get_history(&self) -> GameContextHistory {
        GameContextHistory {
            users: self.users.get(),
            round: self.round.get(),
            game_state: self.game_state.get(),
        }
    }

    pub fn set_history(&self, history: GameContextHistory) {
        self.users.set(history.users);
        self.round.set(history.round);
        self.game_state.set(history.game_state);
    }
}

#[derive(Clone, Debug)]
struct GlobalInfo {
    is_authenticated: RwSignal<bool>,
    users: RwSignal<Vec<UserSheetInfo>>,
}

#[derive(Clone, Debug)]
struct SaveLogState(bool);

#[component]
fn StartScreen() -> impl IntoView {
    let save_log_state = create_rw_signal(SaveLogState(false));

    provide_context(save_log_state);

    let game_context = GameContext::default();

    provide_context(game_context.clone());

    let global_info = GlobalInfo {
        is_authenticated: create_rw_signal(false),
        users: create_rw_signal(Vec::new()),
    };

    provide_context(global_info);

    let global_info = use_context::<GlobalInfo>().expect("GlobalInfo not found");

    let check_auth = async move {
        // Convert JsValue to Promise
        let promise_as_js_value = initializeGAPI();
        let promise = js_sys::Promise::from(promise_as_js_value);

        // Convert the promise to a future
        let future = wasm_bindgen_futures::JsFuture::from(promise);

        match future.await {
            Ok(content) => {
                // Convert the JsValue to a Boolean
                let is_authenticated = js_sys::Boolean::from(content).value_of();

                logging::log!("is_authenticated {:?}", is_authenticated);
                global_info.is_authenticated.set(is_authenticated);
            }
            Err(err) => {
                // Handle the error
                console::log_1(&err);
            }
        }
    };

    wasm_bindgen_futures::spawn_local(check_auth);

    let game_context_clone = game_context.clone();

    create_effect(move |_| {
        if global_info.is_authenticated.get() {
            wasm_bindgen_futures::spawn_local(async move {
                // Convert JsValue to Promise
                let promise_as_js_value = loadAllUsers();
                let promise = js_sys::Promise::from(promise_as_js_value);

                // Convert the promise to a future
                let future = wasm_bindgen_futures::JsFuture::from(promise);

                match future.await {
                    Ok(content) => {
                        // Handle the content
                        let users = js_sys::Array::from(&content);

                        // Convert the JsValue to a Vec<UserSheetInfo>
                        global_info.users.update(move |gl_users| {
                            for i in 0..users.length() {
                                let user_data = users.get(i);
                                let user_info = js_sys::Array::from(&user_data);

                                let info = UserSheetInfo::from(user_info);

                                gl_users.push(info);
                            }

                            logging::log!("{:?}", gl_users);
                        });

                        let global_info_users = global_info.users.get_untracked();

                        // Update the selected user names
                        game_context_clone.users.update(move |users| {
                            for user in users.iter_mut() {
                                if let Some(info) = global_info_users.iter().find(|u| *u == user) {
                                    user.name = info.name().to_string();
                                }
                            }
                        });
                    }
                    Err(err) => {
                        // Handle the error
                        console::log_1(&err);
                    }
                }
            });
        }
    });

    let (context_history, set_context_history) = create_signal(Vec::<GameContextHistory>::new());

    provide_context(context_history);
    provide_context(set_context_history);

    let game_context_clone = game_context.clone();

    create_effect(move |_| {
        // game_context_clone.users.get();
        // game_context_clone.round.get();
        // game_context_clone.game_state.get();

        game_context_clone.store_context_to_local_storage();
    });

    let game_state_view = move || match game_context.game_state.get() {
        GameState::SetupNames => view! {
            <SetupUsers />
        }
        .into_view(),
        GameState::Mafia(_) => view! {
            <MafiaGameView />
        }
        .into_view(),
        GameState::Werewolf(_) => view! {
            <WerewolfGameView />
        }
        .into_view(),
    };

    view! {
        <div class="relative w-full h-full p-4 overflow-hidden">
            <Show when={move || save_log_state.get().0}>
                <div class="absolute top-1 left-1 right-1 z-10">
                    <div class="bg-red-500 text-white flex items-center justify-center px-4 py-1 rounded-lg w-full">
                        "Сохранение игры..."
                    </div>
                </div>
            </Show>
            {game_state_view}
        </div>
    }
}

#[component]
fn SetupUsers() -> impl IntoView {
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");

    let players = move || game_ctx.users.get().into_iter().enumerate();
    let player_len = move || players().count();

    let is_adding_player = create_rw_signal(false);

    view! {
        <div class="relative flex flex-col gap-4 w-full h-full">
            <h2 class="flex w-full items-baseline justify-start gap-2">
                "Выбранные игроки ("{player_len}")"
            </h2>
            {move ||
                if is_adding_player.get() {
                    view! {
                        <SelectPlayersForGame on_close=move || {
                            is_adding_player.set(false);
                        } />
                    }
                } else {
                    view! {
                        <div class="flex-1 flex flex-col gap-1 overflow-auto -mx-4 px-4">
                            <For
                                each=players
                                key=|(index, user)| format!("{}_{}", index.clone(), user.id.clone())
                                children=move |(index, user)| {
                                    view!{
                                        <UserRow
                                            index=index
                                            user=user
                                        />
                                    }
                                }
                            />
                        </div>
                        <button
                            class="bg-gray-300 rounded-full px-4 py-1"
                            on:click=move |_| {
                                is_adding_player.set(true);
                            }
                        >
                            "Добавить игроков"
                        </button>
                        <StartGames />
                    }.into_view()
                }
            }
        </div>
    }
}

#[component]
fn SelectPlayersForGame(on_close: impl Fn() -> () + Clone + 'static) -> impl IntoView {
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");

    let global_info = use_context::<GlobalInfo>().expect("GlobalInfo not found");
    let is_authenticated = move || global_info.is_authenticated.get();

    let global_users = move || global_info.users.get();

    let is_creating_player = create_rw_signal(false);
    let filter_name = create_rw_signal("".to_string());
    let comment = create_rw_signal("".to_string());
    let create_id = create_rw_signal("".to_string());
    let is_guest = create_rw_signal(false);

    let filtered_users = move || {
        let filter = filter_name.get().to_lowercase().replace("#", "");
        global_users()
            .iter()
            .filter(|u| {
                u.id().contains(&filter)
                    || u.name().to_lowercase().starts_with(&filter)
                    || u.comment().to_lowercase().contains(&filter)
            })
            .cloned()
            // sort by name
            .sorted_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()))
            // sort by is_guest
            .sorted_by(|a, b| b.is_guest().cmp(&a.is_guest()))
            .collect::<Vec<_>>()
    };

    fn create_uniq_id(db_users: &[UserSheetInfo], game_users: &[Player]) -> String {
        let mut id = 0;
        let mut is_unique = false;
        while !is_unique {
            id += 1;
            is_unique = !db_users.iter().any(|u| u.id() == format!("{:03}", id))
                && !game_users.iter().any(|u| u.id == format!("{:03}", id));
        }

        // return id with 00x padding
        format!("{:03}", id)
    }

    view! {
        {move ||
            if !is_authenticated() {
                view! {
                    <button
                        class="bg-gray-300 rounded-full px-4 py-1"
                        on:click=move |_| {
                            // Convert JsValue to Promise
                            let promise_as_js_value = handleSigninClick();
                            let promise = js_sys::Promise::from(promise_as_js_value);

                            // Convert the promise to a future
                            let future = wasm_bindgen_futures::JsFuture::from(promise);

                            wasm_bindgen_futures::spawn_local(async move {
                                match future.await {
                                    Ok(content) => {
                                        // Convert the JsValue to a Boolean
                                        let is_authenticated = js_sys::Boolean::from(content).value_of();

                                        logging::log!("is_authenticated {:?}", is_authenticated);
                                        global_info.is_authenticated.set(is_authenticated);
                                    }
                                    Err(err) => {
                                        // Handle the error
                                        console::log_1(&err);
                                    }
                                }
                            });
                        }
                    >
                        "Авторизоваться"
                    </button>
                }
                .into_view()
            } else if is_creating_player.get() {
                view! {
                    <div class="flex-1" />
                    <form class="flex flex-col gap-2 w-full" on:submit=move |ev| {
                        ev.prevent_default();

                        // check name
                        let name = filter_name.get();
                        if name.is_empty() {
                            return;
                        }

                        let is_guest_1 = is_guest.get();

                        // check id
                        let id = create_id.get();
                        if id.is_empty() {
                            return;
                        }

                        let id = if is_guest_1 {
                            create_uniq_id(&global_users(), &game_ctx.users.get())
                        } else {id};

                        let comment_str = comment.get();

                        // check if id is unique
                        if global_users().iter().any(|u| u.id() == id) {
                            return;
                        }

                        if game_ctx.users.get().iter().any(|u| u.id == id) {
                            return;
                        }

                        if !is_guest_1 {
                            if !comment_str.is_empty() {
                                // append comment to name
                                createNewUser(&id, &format!("{} ({})", name, comment_str));
                            }else{
                                createNewUser(&id, &name);
                            }
                        }

                        let id_1 = id.clone();
                        let name_1 = name.clone();
                        let comment_1 = comment_str.clone();
                        game_ctx.users.update(move |users| {
                            if is_guest_1 {
                                users.push(Player::new_guest(id_1, name_1));
                                return;
                            }
                            users.push(Player::new_player(id_1, name_1, comment_1));
                        });

                        let id_1 = id.clone();
                        global_info.users.update(move |users| {
                            users.push(UserSheetInfo::new(id_1, name, comment_str, is_guest_1));
                        });

                        filter_name.set("".to_string());
                        comment.set("".to_string());
                        is_creating_player.set(false);
                    }>
                        <div class="flex gap-2 w-full">
                            "Добавление нового игрока:"
                        </div>
                        <div class="flex gap-2 w-full">
                            <input
                                class=move ||
                                    format!("w-14 px-3 text-center text-sm py-1 border-gray-200 border rounded-full {}",
                                        if is_guest.get() { "bg-gray-200 text-black/50 opacity-50" } else { "" }
                                    )
                                disabled=is_guest
                                maxlength="3"
                                inputmode="numeric"
                                placeholder="ID"
                                on:input=move |ev| {
                                    let name = event_target_value(&ev);
                                    // crop id to 3 symbols only numbers
                                    let name = name.chars().filter(|c| c.is_numeric()).take(3).collect::<String>();
                                    create_id.set(name);
                                }
                                prop:value=create_id
                            />
                            <input
                                class="flex-1 px-3 text-sm py-1 border-gray-200 border rounded-full"
                                placeholder="Имя игрока"
                                on:input=move |ev| {
                                    let name = event_target_value(&ev);
                                    filter_name.set(name);
                                }
                                prop:value=filter_name
                            />
                            <button
                                class=move || format!("flex justify-center px-4 py-1 rounded-full {}",
                                    if is_guest.get() { "bg-blue-300" } else { "bg-gray-200" }
                                )
                                type="button"
                                on:click=move |_| {
                                    is_guest.set(!is_guest.get());
                                }
                            >
                                "Гость"
                            </button>
                        </div>
                        <div class="flex gap-2 w-full">
                            <input
                                disabled=is_guest
                                class=move ||
                                    format!(
                                    "flex-1 px-3 text-sm py-1 border-gray-200 border rounded-full {}",
                                        if is_guest.get() { "bg-gray-200 text-black/50 opacity-50" } else { "" }
                                    )
                                placeholder="Комментарий"
                                on:input=move |ev| {
                                    let name = event_target_value(&ev);
                                    comment.set(name);
                                }
                                prop:value=comment
                            />
                        </div>
                        <div class="flex gap-2 w-full">
                            <button
                                type="button"
                                class="flex-1 px-4 py-1 bg-gray-200 rounded-full"
                                on:click=move |_| {
                                    is_creating_player.set(false);
                                }
                            >
                                "Отмена"
                            </button>
                            <button
                                class="flex-1 flex justify-center px-4 py-1 bg-gray-200 rounded-full"
                            >
                                "Создать игрока"
                            </button>
                        </div>
                    </form>
                }.into_view()
            } else {
                view! {
                    <div class="flex-1 overflow-auto -mx-4 px-4">
                        <div class="flex flex-col justify-end gap-1">
                            <button
                                type="button"
                                class=move ||
                                    format!("flex gap-1 items-baseline justify-start px-3 py-1 mb-2 text-base rounded-full bg-red-200")
                                on:click=move |_| {
                                    game_ctx.users.update(|users| {
                                        // clear all users
                                        users.clear();
                                    });
                                }
                            >
                                "Очистить выбор всех игроков"
                            </button>
                            <For
                                each=filtered_users
                                key=|user| user.id()
                                children=move |user| {
                                    let user_id2 = user.id();
                                    let user_id3 = user.id();
                                    let user_id4 = user.id();
                                    let user_name = &user.name();
                                    let user_comment = &user.comment();
                                    let user_is_guest = user.is_guest();

                                    let is_selected = move || {
                                        let user_id_ref = &user_id2;
                                        game_ctx.users.get().iter().any(|u| {
                                            u.id == *user_id_ref
                                        })
                                    };

                                    view!{
                                        <button
                                            type="button"
                                            class=move ||
                                                format!("flex gap-1 items-baseline justify-start px-3 py-1 text-base rounded-full {}",
                                                    if is_selected() { "bg-blue-300" } else { "bg-gray-200" }
                                                )
                                            on:click=move |_| {
                                                game_ctx.users.update(|users| {
                                                    // check if user already exists
                                                    if users.iter().any(|u| u.id == user_id3) {
                                                        // remove
                                                        users.retain(|u| u.id != user_id3);
                                                        return;
                                                    }

                                                    if user.is_guest() {
                                                        users.push(Player::new_guest(user.id(), user.name().to_string()));
                                                    }else{
                                                        users.push(Player::new_player(user.id().to_string(), user.name().to_string(), user.comment().to_string()));
                                                    }
                                                });
                                            }
                                        >
                                            <span class="opacity-70 text-sm w-9">
                                                {move || if user_is_guest { "guest".to_string() } else { format!("#{}", user_id4) }}
                                            </span>
                                            {user_name}
                                            <span class="opacity-50 text-sm">{user_comment}</span>
                                        </button>
                                    }
                                }
                            />
                        </div>
                    </div>
                    <form class="flex flex-col gap-2 w-full" on:submit=move |ev| {
                        ev.prevent_default();

                        is_creating_player.set(true);

                        create_id.set(create_uniq_id(&global_users(), &game_ctx.users.get()));
                    }>
                        <div class="flex gap-2 w-full">
                            <input
                                class="flex-1 px-3 text-sm py-1 border-gray-200 border rounded-full"
                                placeholder="Имя игрока или номер"
                                on:input=move |ev| {
                                    let name = event_target_value(&ev);
                                    filter_name.set(name);
                                }
                                prop:value=filter_name
                            />
                            <button
                                type="submit"
                                class="flex px-4 py-1 bg-gray-200 rounded-full"
                            >
                                "Создать"
                            </button>
                        </div>
                        <button
                            type="button"
                            class="flex-1 px-4 py-1 bg-gray-200 rounded-full"
                            on:click={
                                let on_close = on_close.clone();
                                move |_| {
                                    on_close();
                                }
                            }
                        >
                            "Закрыть"
                        </button>
                    </form>
                }.into_view()
            }
        }
    }
}

#[component]
fn StartGames() -> impl IntoView {
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");

    let game_ctx_clone = game_ctx.clone();
    let start_werewolf_game = move |_| {
        game_ctx.users.update(|users| {
            reset_user_roles(users);
        });

        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>()
            .expect("MafiaContext history not found");

        set_context_history.update(|history| {
            history.clear();
            history.push(game_ctx_clone.get_history());
        });

        // // let first_role = WEREWOLF_ROLES.iter().filter(|r| r.get_role() == Role::Werewolf(WerewolfRole::Bodyguard)).next().unwrap();

        game_ctx
            .game_state
            .set(GameState::Werewolf(WerewolfGameState::SelectActiveRoles));
    };

    let game_ctx_clone = game_ctx.clone();
    let start_mafia_game = move |_| {
        game_ctx.users.update(|users| {
            reset_user_roles(users);
        });

        let set_context_history = use_context::<WriteSignal<Vec<GameContextHistory>>>()
            .expect("MafiaContext history not found");

        set_context_history.update(|history| {
            history.clear();
            history.push(game_ctx_clone.get_history());
        });

        let first_role = MAFIA_ROLES
            .iter()
            .filter(|r| r.get_role() == Role::Mafia(MafiaRole::Mafia))
            .next()
            .unwrap();
        game_ctx
            .game_state
            .set(GameState::Mafia(MafiaGameState::SetupRoles(first_role)));
    };

    view! {
        <div class="flex gap-1 items-center justify-between">
            <button
                on:click=start_werewolf_game
                    class="flex-grow px-4 py-1 bg-gray-200 rounded-full"
            >
                "Начать в Werewolf"
            </button>
            <button
                on:click=start_mafia_game
                class="flex-grow px-4 py-1 bg-gray-200 rounded-full"
            >
                "Начать в Мафию"
            </button>
        </div>
    }
}

#[component]
fn UserRow(user: Player, index: usize) -> impl IntoView {
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");
    let users = game_ctx.users;

    let user_id = user.id.clone();
    let user_is_guest = user.is_guest;
    let user_c1 = user.clone();

    view! {
        <div class="flex gap-2 items-baseline">
            <div class="text-sm w-7 flex items-center justify-center bg-gray-100 rounded-full px-2">{index + 1}</div>
            <div class="flex-1 gap-1 flex items-baseline justify-start px-3 py-1 text-base bg-gray-200 rounded-full">
                <span class="opacity-70 text-sm w-9">{move || if user_is_guest { "guest".to_string() } else { format!("#{}", user_id) }}</span>
                {user.name}
                <span class="opacity-50 text-sm">{user.comment}</span>
                <div class="flex-1"></div>
                //down
                <button
                    class="text-center text-lg w-7 opacity-50"
                    on:click=move |_| {
                        users.update(|users| {
                            if index < users.len() - 1 {
                                users.swap(index, index + 1);
                            }
                        });
                    }
                >
                    "↓"
                </button>
                //up
                <button
                    class="text-center text-lg w-7 opacity-50"
                    on:click=move |_| {
                        users.update(|users| {
                            if index > 0 {
                                users.swap(index, index - 1);
                            }
                        });
                    }
                >
                    "↑"
                </button>
            </div>
            <button class="text-lg"
                on:click=move |_| {
                    users.update(|users| users.retain(|u| *u != user_c1));
                }
            >
                "✕"
            </button>
        </div>
    }
}

#[component]
fn GlobalUserRow(id: String, name: String, is_added: impl Fn() -> bool) -> impl IntoView {
    view! {
        <button class="flex-1 flex items-center justify-start px-3 py-1 text-base bg-gray-200 rounded-full">
            #{id}
            {name}
        </button>
    }
}
