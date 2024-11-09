use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView{
    view!{
        <div class="navigation-buttons">
            /* <HomeButton/>
            <ProductButton/>
            <ContactButton/>
            <SignUp/>
            <SignIn/> */
            <ul class="navigation">
                <li><a href="/">"Home"</a></li>
                <li><a href="/products">"Products"</a></li>
                <li><a href="/contacts">"Contact"</a></li>
                <li><a href="/sign-up">"Sign Up"</a></li>
                <li><a href="/sign-in">"Sign In"</a></li>
            </ul>
        </div>
    }
}