use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct VirtualManCaveComponentProps {
}

#[function_component(VirtualManCave)]
pub fn virtual_man_cave_component(VirtualManCaveComponentProps {}: &VirtualManCaveComponentProps) -> Html {
    html! { 
        <h1 class={classes!("mountain-text")}>
        <title>{"Certified old school time wasters:"}</title>
        <p>{ "Vsauce" }</p>
        <iframe width="560" height="315" src="https://www.youtube.com/embed/jHbyQ_AQP8c" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
        <p> { "Cool math games" } </p>
        <iframe src="https://www.miniplay.com/embed/bloxorz" width="600" height="400" frameborder="0" allowfullscreen=true></iframe>
        // todo: embed this properly without ruining the webpage layout
        // <p> {"park the taxi game"} </p>
        // <iframe src="https://1000webgames.com/games/parkthetaxi/html5/" frameborder="0" width="800" height="450" scrolling="no" allowfullscreen=true></iframe>
        </h1> }
}
