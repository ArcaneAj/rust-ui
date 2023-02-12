use yew::prelude::*;
use yew_router::prelude::*;

use crate::enums::routes::Route;

#[derive(PartialEq, Properties)]
pub struct MenuComponentProps {
    pub title: String,
    // on_click: Callback<String>
}

#[function_component(Menu)]
pub fn menu_component(MenuComponentProps { title }: &MenuComponentProps) -> Html {
    let home_navigator = use_navigator().unwrap();
    let contact_navigator = use_navigator().unwrap();
    let links_navigator = use_navigator().unwrap();
    let virtual_mancave_navigator = use_navigator().unwrap();
    let on_home_click = Callback::from(move |_| home_navigator.push(&Route::Home));
    let on_contact_click = Callback::from(move |_| contact_navigator.push(&Route::Contact));
    let on_links_click = Callback::from(move |_| links_navigator.push(&Route::Links));
    let on_virtual_mancave_navigator = Callback::from(move |_| virtual_mancave_navigator.push(&Route::VirtualManCave));
    html! {
        <menu class={classes!("top-align")}>
            <menu-content>
                <title onclick={on_home_click} class={classes!("first", "logo")}>{ title }</title>
                <title onclick={on_contact_click}>{ "Contact Info" }</title>
                <title onclick={on_links_click}>{ "Links" }</title>
                <title onclick={on_virtual_mancave_navigator}>{ "Virtual Man Cave" }</title>
            </menu-content>
        </menu>
    }
}
