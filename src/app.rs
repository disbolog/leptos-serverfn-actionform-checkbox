use leptos::prelude::*;
use leptos::either::Either;
// use leptos::html::Input;
use server_fn::ServerFnError;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub on_or_off: bool,
}

#[cfg(feature = "ssr")]
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(feature = "ssr")]
static ON_OR_OFF: AtomicBool = AtomicBool::new(true);

#[server]
pub async fn load_settings() -> Result<Settings, ServerFnError> {
    Ok(Settings {
        on_or_off: ON_OR_OFF.load(Ordering::Relaxed)
    })
}

#[server]
pub async fn change_settings(on_or_off: bool) -> Result<(), ServerFnError> {
    ON_OR_OFF.store(on_or_off, Ordering::Relaxed);
    Ok(())
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() /> <HydrationScripts options />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <header>
                <h1>"Checkbox Testing"</h1>
            </header>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/vanilla-checkbox") view=ViewSettings />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <a href="/vanilla-checkbox">"Vanilla Checkbox"</a>
         </div>
    }
}

#[component]
pub fn ViewSettings() -> impl IntoView {
    // let on_or_off_input_ref = NodeRef::<Input>::new();

    // let checked = RwSignal::new(true);

    let change_settings = ServerMultiAction::<ChangeSettings>::new();

    let settings = Resource::new(
        move || (change_settings.version().get(),),
        move |_| load_settings(),
    );

    let existing_settings = move || {
        Suspend::new(async move {
            match settings.await {
                Ok(settings) => Either::Left(view! {
                    <div>
                        <MultiActionForm action=change_settings>
                            <label>"On or off: "
                                <input
                                    type="checkbox"
                                    name="on_or_off"
                                    // value=settings.on_or_off //.to_string()
                                    prop:checked=settings.on_or_off
                                    // bind:checked=checked
                                    // node_ref=on_or_off_input_ref
                                    // on:input
                                    // prop:value=
                                />
                            </label>
                            <br />
                            <input type="submit" value="Save changes" />
                        </MultiActionForm>
                    </div>
                    <div>
                    <br /><br />
                    <MultiActionForm action=change_settings>
                        <label>"On or off: "
                            <input
                                type="text"
                                name="on_or_off"
                                value=settings.on_or_off.to_string()
                            />
                        </label>
                        <br />
                        <input type="submit" value="Save changes" />
                    </MultiActionForm>
                </div>

                }),
                Err(e) => Either::Right(
                    view! { <div class="error">{e.to_string()}</div> },
                ),
            }
        })
    };

    view! {
        <div>
            <a href="/">"Back to home"</a>
            <Suspense fallback=move || { view! { <div>"Loading..."</div> } }>
                <ErrorBoundary fallback=|_errors| { view! { <div>"Failed to load"</div> } }>
                    <div>{existing_settings}</div>
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
