use leptos::*;

#[component]
pub fn ContactButton() -> impl IntoView{
    view! {
        <button>
            <nav>
                <a href="/contacts">"Contacts"</a>
            </nav>
        </button>
    }   
}