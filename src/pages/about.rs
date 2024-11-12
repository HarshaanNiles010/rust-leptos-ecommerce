use leptos::*;
use crate::components::nav_bar_comp::NavBar;
#[component]
pub fn About() -> impl IntoView {
    view! {
        <NavBar/>
        <h1>"This is the contacts page"</h1>
        <h1>"this is not for you"</h1>
        <h1>"Why are you still here?"</h1>
    }
}
