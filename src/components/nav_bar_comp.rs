use leptos::*;
use crate::components::contact_btn::ContactButton;
use crate::components::product_btn::ProductButton;
use crate::components::home_btn::HomeButton;
use crate::components::sign_up_btn::SignUp;
use crate::components::sign_in_btn::SignIn;

#[component]
pub fn NavBar() -> impl IntoView{
    view!{
        <div class="navigation-buttons">
            <HomeButton/>
            <ProductButton/>
            <ContactButton/>
            <SignUp/>
            <SignIn/>
        </div>
    }
}