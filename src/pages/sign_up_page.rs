use leptos::*;
use crate::components::nav_bar_comp::NavBar;

pub fn sign_up_page() -> impl IntoView{
    view!{
        <NavBar/>
        <h1> "This is the Sign Up page"</h1>
    }
}