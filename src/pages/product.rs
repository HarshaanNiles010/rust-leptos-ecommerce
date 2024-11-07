use leptos::*;
use crate::components::nav_bar_comp::NavBar;

#[component]
pub fn Product() -> impl IntoView {
    view! { 
        <NavBar/>
        <h1>"This is the products page"</h1>
    }
}
