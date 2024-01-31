use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;

fn main() {
    leptos::mount_to_body(|| leptos::view! { <App /> })
}

#[component]
pub fn App() -> impl IntoView {
    let (authenticated, set_authenticated) = create_signal(false);

    let data = create_resource(
        move || (),
        move |_| async move { api_check_authenticated().await },
    );

    let auth = move || {
        data.with(|d| match d {
            Some(auth) => {
                logging::log!("we received auth status {}", auth);
                set_authenticated.set(*auth);
                *auth
            }
            None => {
                logging::log!("api didn't return auth status yet");
                false
            }
        })
    };

    view! {
        <header>
        <nav>
            <a href="/">Home</a>
            <Show when=move || authenticated.get() fallback=|| view! { }>
                <a href="/info">Super info</a>
            </Show>

            </nav>
            </header>
        <Router>
        <Routes>
            <Route path="/" view=Dashboard/>
            <Route path="/info" view=Info/>
            <Route path="/aws_cognito_signin" view=AwsCognitoSignin/>
            <Route path="/*any" view=move || view!{"Not Found"}/>
        </Routes>
    </Router>
    <b> auth  data</b>
    <p>{ auth }</p>
    }
}

#[component]
pub fn Info() -> impl IntoView {
    view! {
        <b>Very important information</b><br/>
    }
}

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <h1>Super Dashboard</h1>

    }
}

#[component]
pub fn AwsCognitoSignin() -> impl IntoView {
    logging::log!("calling API, authorising, redirecting to  home...");
    // won't add real redirect to ease testing
    view! {
        "Let's pretend we don't see it"<br/>
    }
}

async fn api_check_authenticated() -> bool {
    logging::log!("performing api call to check if user is authenticated");
    TimeoutFuture::new(1_000).await;
    true
}
