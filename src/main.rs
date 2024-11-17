use itertools::Itertools;
use leptos::*;
mod mafia;
mod roles;
mod user;
mod werewolf;

use mafia::*;
use roles::Role;
use serde::Serialize;
use user::*;
use wasm_bindgen::prelude::*;
use web_sys::{console, js_sys};
use werewolf::*;

#[derive(Serialize, Debug)]
pub struct UserLogs {
    id: String,
    name: String,
    role: String,
    score: u32,
    winner: bool,
    best_player: bool,
    role_index: u32,
    rounds: Vec<String>,
}

#[wasm_bindgen(module = "/src/js/GoogleSheetsAPI.js")]
extern "C" {
    pub fn initializeGAPI() -> JsValue; // JsValue <==> Promise
    pub fn handleSigninClick() -> JsValue; // JsValue <==> Promise
    pub fn loadAllUsers() -> JsValue; // JsValue <==> Promise
    pub fn createNewUser(id: &str, name: &str) -> JsValue; // JsValue <==> Promise
    pub fn createNewGameLog(users: &JsValue) -> JsValue; // JsValue <==> Promise
}

pub fn rust_create_new_game_log(log_users: Vec<UserLogs>) {
    // Convert the users into a JsValue that JS can understand
    let js_users = serde_wasm_bindgen::to_value(&log_users);

    if let Ok(js_users) = js_users {
        // Call the JS function
        createNewGameLog(&js_users);
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

impl Default for GameState<'_> {
    fn default() -> Self {
        GameState::SetupNames
    }
}

#[derive(Clone, Debug)]
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

impl Default for GameContext {
    fn default() -> Self {
        let starting_users = window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| {
                storage
                    .get_item(STORAGE_SELECTED_USERS_KEY)
                    .ok()
                    .flatten()
                    .and_then(|value| serde_json::from_str::<Vec<(String, String)>>(&value).ok())
            })
            .unwrap_or_default()
            .into_iter()
            .map(|(id, name)| Player::new_player(id, name));

        Self {
            users: create_rw_signal(starting_users.collect()),
            round: create_rw_signal(0),
            game_state: create_rw_signal(GameState::SetupNames),
        }
    }
}

impl GameContext {
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

const STORAGE_SELECTED_USERS_KEY: &str = "mafia_users";

#[derive(Clone, Debug)]
struct GlobalInfo {
    is_authenticated: RwSignal<bool>,
    users: RwSignal<Vec<UserSheetInfo>>,
}

#[component]
fn StartScreen() -> impl IntoView {
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

                        // Update the selected user names
                        game_context_clone.users.update(move |users| {
                            for user in users.iter_mut() {
                                if let Some(info) = global_info
                                    .users
                                    .get_untracked()
                                    .iter()
                                    .find(|u| *u == user)
                                {
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

    create_effect(move |_| {
        if let Ok(Some(storage)) = window().local_storage() {
            let user_names = &game_context
                .users
                .get()
                .iter()
                .map(|u| (u.id.clone(), u.name.clone()))
                .collect::<Vec<_>>();
            let json = serde_json::to_string(user_names).expect("couldn't serialize Users");
            if storage.set_item(STORAGE_SELECTED_USERS_KEY, &json).is_err() {
                //log::error!("error while trying to set item in localStorage");
            }
        }
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
            {game_state_view}
        </div>
    }
}

#[component]
fn SetupUsers() -> impl IntoView {
    let game_ctx = use_context::<GameContext>().expect("MafiaContext not found");

    let players = move || game_ctx.users.get().into_iter().enumerate();

    let is_adding_player = create_rw_signal(false);

    view! {
        <div class="relative flex flex-col gap-4 w-full h-full">
            <h2 class="flex w-full items-baseline justify-start gap-2">
                "Введите имена игроков"
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
    let create_id = create_rw_signal("".to_string());
    let is_guest = create_rw_signal(false);

    let filtered_users = move || {
        let filter = filter_name.get().to_lowercase().replace("#", "");
        global_users()
            .iter()
            .filter(|u| u.id().contains(&filter) || u.name().to_lowercase().starts_with(&filter))
            .cloned()
            // sort by name
            .sorted_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()))
            .collect::<Vec<_>>()
    };

    fn create_uniq_id(users: &[UserSheetInfo]) -> String {
        let mut id = 0;
        let mut is_unique = false;
        while !is_unique {
            id += 1;
            is_unique = !users.iter().any(|u| u.id() == format!("{:03}", id));
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

                        // check id
                        let id = create_id.get();
                        if id.is_empty() {
                            return;
                        }
                        // check if id is unique
                        if global_users().iter().any(|u| u.id() == id) {
                            return;
                        }

                        createNewUser(&id, &name);

                        let id_1 = id.clone();
                        let name_1 = name.clone();
                        game_ctx.users.update(move |users| {
                            users.push(Player::new_player(id_1, name_1));
                        });

                        global_info.users.update(move |users| {
                            users.push(UserSheetInfo::new(id, name));
                        });

                        filter_name.set("".to_string());
                        is_creating_player.set(false);
                    }>
                        <div class="flex gap-2 w-full">
                            "Добавление нового игрока:"
                        </div>
                        <div class="flex gap-2 w-full">
                            <input
                                class=move ||
                                    format!("w-14 px-3 text-center text-sm py-1 border-gray-200 border rounded-full {}",
                                        if is_guest.get() { "bg-gray-200 text-black/50" } else { "" }
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
                            // <button
                            //     class=move || format!("flex justify-center px-4 py-1 rounded-full {}",
                            //         if is_guest.get() { "bg-blue-300" } else { "bg-gray-200" }
                            //     )
                            //     type="button"
                            //     on:click=move |_| {
                            //         is_guest.set(!is_guest.get());
                            //     }
                            // >
                            //     "Гость"
                            // </button>
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
                    <div class="flex-1 flex flex-col justify-end gap-1 overflow-auto -mx-4 px-4">
                        <For
                            each=filtered_users
                            key=|user| user.id()
                            children=move |user| {
                                let user_id = &user.id();
                                let user_id2 = user.id();
                                let user_id3 = user.id();
                                let user_name = &user.name();

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

                                                users.push(Player::new_player(user.id(), user.name()));
                                            });
                                        }
                                    >
                                        <span class="opacity-70 text-sm w-9">#{user_id}</span>
                                        {user_name}
                                    </button>
                                }
                            }
                        />
                    </div>
                    <form class="flex flex-col gap-2 w-full" on:submit=move |ev| {
                        ev.prevent_default();

                        is_creating_player.set(true);

                        create_id.set(create_uniq_id(&global_users()));
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

    view! {
        <div class="flex gap-2 items-baseline">
            <div class="text-sm w-7 flex items-center justify-center bg-gray-100 rounded-full px-2">{index + 1}</div>
            <div class="flex-1 flex items-center justify-start px-3 py-1 text-base bg-gray-200 rounded-full">
                <span class="opacity-70 text-sm w-9 mr-1">#{user_id}</span>
                {user.name.clone()}
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
                    users.update(|users| users.retain(|u| *u != user));
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
