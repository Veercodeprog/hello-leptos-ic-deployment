// use leptos::*;
// use wasm_bindgen::prelude::*;
//
// // Entry point for the WASM application
// #[wasm_bindgen(start)]
// pub fn main() {
//     // This ensures the following code only runs in the browser (client-side).
//     leptos::mount_to_body(|| view! { <App /> });
// }
//
// // Define your main app component
// #[component]
// fn App() -> impl IntoView {
//     let (count, set_count) = create_signal(0); // A signal to store and update the count
//
//     view! {
//         <div>
//             // Display a static message in the frontend
//             <h1>"Hello, Frontend!"</h1>
//             <p>"This content is rendered in the browser using Leptos!"</p>
//             <button on:click=move |_| {
//                 set_count(count.get() + 1);
//             }>
//                 // Display the current count
//                 "Click me: " {move || count()}
//             </button>
//         </div>
//     }
// }
