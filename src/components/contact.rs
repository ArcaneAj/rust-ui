use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ContactComponentProps {
}

#[function_component(Contact)]
pub fn contact_component(ContactComponentProps {}: &ContactComponentProps) -> Html {
    html! { <h1 class={classes!("mountain-text")}>{"Don't contact me"}</h1> }
}
