use leptos::*;

#[component]
pub fn ProductButton() -> impl IntoView {
    view! {
        <button>
            <nav>
                <a href="/products">"Products"</a>
            </nav>
        </button>
    }
}
