use leptos::*;
use wasm_bindgen::prelude::*;

// Entry point for the WASM application, but only when running in the browser
pub fn main() {
    leptos::mount_to_body(|| view! { <App /> });
}

// Define your main app component
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <h1>"Hello, Frontend!"</h1>
            <p>"This content is rendered in the browser using Leptos!"</p>
            <button on:click=move |_| {
                set_count(count.get() + 1);
            }>"Click me: " {count()}</button>
        </div>
    }
}
