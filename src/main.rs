use leptos::*;

mod mafia;
mod roles;
mod user;
mod werewolf;

use mafia::*;
use user::*;
use werewolf::*;

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
struct MafiaContext<'a> {
    users: Vec<User>,
    game_state: GameState<'a>,
}

const STORAGE_KEY: &str = "mafia_users";

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
        <div class="flex flex-col justify-start items-center w-full h-full gap-4 p-4">
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
                    // check if user already exists
                    if ctx.users.iter().any(|u| u.name == name.get()) {
                        return;
                    }

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
            <div class="flex gap-1 items-center justify-between">
                <button
                        on:click=move |_| set_mafia_ctx.update(|ctx| {
                            reset_user_roles(&mut ctx.users);
                            set_context_history.update(|history| {
                                history.clear();
                                history.push(ctx.clone());
                            });

                            let first_role = WEREWOLF_ROLES.first().unwrap();
                            ctx.game_state = GameState::Werewolf(WerewolfGameState::SetupRoles(first_role));
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

                        let first_role = MAFIA_ROLES.first().unwrap();
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
