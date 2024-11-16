use leptos::*;
mod mafia;
mod roles;
mod user;
mod werewolf;

use mafia::*;
use roles::Role;
use user::*;
use wasm_bindgen::prelude::*;
use web_sys::{
    console,
    js_sys::{self, Promise},
};
use werewolf::*;

#[wasm_bindgen(module = "/src/js/GoogleSheetsAPI.js")]
extern "C" {
    pub fn initializeGAPI() -> JsValue; // JsValue <==> Promise
    pub fn loadAllUsersAsync() -> JsValue; // JsValue <==> Promise
}

fn main() {
    initializeGAPI();
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
struct MafiaContext<'a> {
    users: Vec<User>,
    round: usize,
    game_state: GameState<'a>,
}

const STORAGE_KEY: &str = "mafia_users";

#[derive(Clone, Debug)]
struct UserSheetInfo {
    id: String,
    name: String,
    score: i32,
    mafia: UserMafiaSheetInfo,
    werewolf: UserWerewolfSheetInfo,
}

#[derive(Clone, Debug)]
struct UserMafiaSheetInfo {
    score: i32,
    games: i32,
    wins: i32,
    win_citizen: i32,
    win_mafia: i32,
    win_maniac: i32,
    win_commissar: i32,
    win_prostitute: i32,
    win_doctor: i32,
    best_player: i32,
}

#[derive(Clone, Debug)]
struct UserWerewolfSheetInfo {
    score: i32,
    games: i32,
    wins: i32,
    win_villager: i32,
    win_werewolf: i32,
    best_player: i32,
}

impl From<js_sys::Array> for UserSheetInfo {
    fn from(user_info: js_sys::Array) -> Self {
        UserSheetInfo {
            id: user_info.get(0).as_string().unwrap(),
            name: user_info.get(1).as_string().unwrap(),
            score: user_info.get(2).as_string().unwrap().parse().unwrap(),
            mafia: UserMafiaSheetInfo {
                score: user_info.get(3).as_string().unwrap().parse().unwrap(),
                games: user_info.get(4).as_string().unwrap().parse().unwrap(),
                wins: user_info.get(5).as_string().unwrap().parse().unwrap(),
                win_citizen: user_info.get(6).as_string().unwrap().parse().unwrap(),
                win_mafia: user_info.get(7).as_string().unwrap().parse().unwrap(),
                win_maniac: user_info.get(8).as_string().unwrap().parse().unwrap(),
                win_commissar: user_info.get(9).as_string().unwrap().parse().unwrap(),
                win_prostitute: user_info.get(10).as_string().unwrap().parse().unwrap(),
                win_doctor: user_info.get(11).as_string().unwrap().parse().unwrap(),
                best_player: user_info.get(12).as_string().unwrap().parse().unwrap(),
            },
            werewolf: UserWerewolfSheetInfo {
                score: user_info.get(13).as_string().unwrap().parse().unwrap(),
                games: user_info.get(14).as_string().unwrap().parse().unwrap(),
                wins: user_info.get(15).as_string().unwrap().parse().unwrap(),
                win_villager: user_info.get(16).as_string().unwrap().parse().unwrap(),
                win_werewolf: user_info.get(17).as_string().unwrap().parse().unwrap(),
                best_player: user_info.get(18).as_string().unwrap().parse().unwrap(),
            },
        }
    }
}

impl Default for MafiaContext<'_> {
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
            round: 0,
            game_state: GameState::SetupNames,
        }
    }
}

#[component]
fn StartScreen() -> impl IntoView {
    let (context_history, set_context_history) = create_signal(Vec::<MafiaContext>::new());
    let (mafia_context, set_mafia_context) = create_signal(MafiaContext::default());

    provide_context(context_history);
    provide_context(set_context_history);
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
    let mafia_ctx = use_context::<ReadSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_mafia_ctx = use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");
    let set_context_history =
        use_context::<WriteSignal<Vec<MafiaContext>>>().expect("MafiaContext history not found");

    let (name, set_name) = create_signal("".to_string());

    let users = move || mafia_ctx.get().users.into_iter().enumerate();

    async fn load_users() {
        // Convert JsValue to Promise
        let promise_as_js_value = loadAllUsersAsync();
        let promise = Promise::from(promise_as_js_value);

        // Convert the promise to a future
        let future = wasm_bindgen_futures::JsFuture::from(promise);

        match future.await {
            Ok(content) => {
                // Handle the content
                let users = js_sys::Array::from(&content);

                // Convert the JsValue to a Vec<String>
                let mut users_sheet_info = Vec::<UserSheetInfo>::new();
                for i in 0..users.length() {
                    let user_data = users.get(i);
                    let user_info = js_sys::Array::from(&user_data);

                    let info = UserSheetInfo::from(user_info);

                    users_sheet_info.push(info);
                }

                logging::log!("{:?}", users_sheet_info);
            }
            Err(err) => {
                // Handle the error
                console::log_1(&err);
            }
        }
    }

    view! {
        <div class="relative flex flex-col gap-4 w-full h-full">
            <h2 class="flex w-full items-baseline justify-between gap-2">
                "Введите имена игроков"
                <button on:click=move |_| {
                    wasm_bindgen_futures::spawn_local(load_users());
                }>
                Загрузить
                </button>
            </h2>
            <div class="flex-1 flex flex-col gap-1 overflow-auto -mx-4 px-4">
                <For
                    each=users
                    key=|(_, user)| user.name.clone()
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
            <form class="flex gap-2 w-full" on:submit=move |ev| {
                ev.prevent_default();

                if name.get().is_empty() {
                    return;
                }

                set_mafia_ctx.update(|ctx| {
                    // check if user already exists
                    if ctx.users.iter().any(|u| u.name == name.get()) {
                        return;
                    }

                    ctx.users.push(User::new(name.get().clone()));
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
            <div class="flex gap-1 items-center justify-between">
                <button
                        on:click=move |_| set_mafia_ctx.update(|ctx| {
                            reset_user_roles(&mut ctx.users);
                            set_context_history.update(|history| {
                                history.clear();
                                history.push(ctx.clone());
                            });

                            // let first_role = WEREWOLF_ROLES.iter().filter(|r| r.get_role() == Role::Werewolf(WerewolfRole::Bodyguard)).next().unwrap();
                            ctx.game_state = GameState::Werewolf(WerewolfGameState::SelectActiveRoles);
                        })
                        class="flex-grow px-4 py-1 bg-gray-200 rounded-full"
                >
                    "Начать в Werewolf"
                </button>
                <button
                    on:click=move |_| set_mafia_ctx.update(|ctx| {
                        reset_user_roles(&mut ctx.users);
                        set_context_history.update(|history| {
                            history.clear();
                            history.push(ctx.clone());
                        });

                        let first_role = MAFIA_ROLES.iter().filter(|r| r.get_role() == Role::Mafia(MafiaRole::Mafia)).next().unwrap();
                        ctx.game_state = GameState::Mafia(MafiaGameState::SetupRoles(first_role));
                    })
                    class="flex-grow px-4 py-1 bg-gray-200 rounded-full"
                >
                    "Начать в Мафию"
                </button>
            </div>
        </div>
    }
}

#[component]
fn UserRow(user: User, index: usize) -> impl IntoView {
    let users = use_context::<WriteSignal<MafiaContext>>().expect("MafiaContext not found");

    view! {
        <div class="flex gap-2 items-baseline">
            <div class="text-sm w-7 flex items-center justify-center bg-gray-100 rounded-full px-2">{index + 1}</div>
            <div class="flex-1 flex items-center justify-start px-3 py-1 text-base bg-gray-200 rounded-full">
                {user.name.clone()}
                <div class="flex-1"></div>
                //down
                <button
                    class="text-center text-lg w-7 opacity-50"
                    on:click=move |_| {
                        users.update(|ctx| {
                            if index < ctx.users.len() - 1 {
                                ctx.users.swap(index, index + 1);
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
                        users.update(|ctx| {
                            if index > 0 {
                                ctx.users.swap(index, index - 1);
                            }
                        });
                    }
                >
                    "↑"
                </button>
            </div>
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
