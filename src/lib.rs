use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::product::Product;
use crate::pages::sign_up_page::sign_up_page;
use crate::pages::sign_in_page::sign_in_page;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Welcome to Leptos CSR" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/about" view=About />
                <Route path="/products" view=Product />
                <Route path="/sign-up" view=sign_up_page />
                <Route path="/sign-in" view=sign_in_page />
                <Route path="/*" view=NotFound />
            </Routes>
        </Router>
    }
}
