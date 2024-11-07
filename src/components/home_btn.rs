use leptos::*;
#[component]
pub fn HomeButton() -> impl IntoView{
    view!{
        <button>
            <nav>
                <a href="/">"Home"</a>
            </nav>
        </button>
    }
}