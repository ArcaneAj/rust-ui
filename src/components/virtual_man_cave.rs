use yew::prelude::*;

use crate::components::nav_button::NavButton;

#[derive(PartialEq, Properties)]
pub struct VirtualManCaveComponentProps {
}

#[function_component(VirtualManCave)]
pub fn virtual_man_cave_component(VirtualManCaveComponentProps {}: &VirtualManCaveComponentProps) -> Html {
    let counter = use_state(|| 0);
    
    let on_click_next = {
        let counter = counter.clone();
        move |_| {
            let value = (*counter + 1) % WIDGET_COUNT;
            counter.set(value);
        }
    };
    
    let on_click_back = {
        let counter = counter.clone();
        move |_| {
            let value = (*counter + WIDGET_COUNT - 1) % WIDGET_COUNT;
            counter.set(value);
        }
    };

    html! { 
        <div class={classes!("man-cave")}>
            <div>
                <h1 style="display: inline-block;">{"Certified old school time wasters:"}</h1>
                <div style="float: right; font-size: 0;">
                    <NavButton text="<" onclick={on_click_back}></NavButton>
                    <NavButton text=">" onclick={on_click_next}></NavButton>
                </div>
            </div>
            { switch(get_widget_by_index(*counter)) }
        </div>
         }
}

fn get_widget_by_index(index: usize) -> Widget {
    match index {
        0 => Widget::YouTube,
        1 => Widget::Bloxorz,
        2 => Widget::Taxi,
        _ => Widget::YouTube, // Fallback should not happen if done correctly, can be done safer I'm sure
    }
}

const WIDGET_COUNT: usize = 3;

#[derive(Clone, PartialEq)]
enum Widget {
    YouTube,
    Bloxorz,
    Taxi,
}

fn switch(routes: Widget) -> Html {
    match routes {
        Widget::YouTube => html! { <YouTube title="Vsauce" link="https://www.YouTube.com/embed/jHbyQ_AQP8c"/> },
        Widget::Bloxorz => html! { <Bloxorz/> },
        Widget::Taxi => html! { <Taxi/> },
    }
}

#[derive(PartialEq, Properties)]
struct YouTubeComponentProps {
    pub title: AttrValue,
    pub link: AttrValue,
}

#[function_component(YouTube)]
fn youtube_component(YouTubeComponentProps { title, link }: &YouTubeComponentProps) -> Html {

    html! { 
        <div id="YouTube">
            <p>{ title }</p>
            <iframe src={ link } width="560" height="315" frameborder="0" title="YouTube video player" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
        </div>
         }
}

#[derive(PartialEq, Properties)]
struct BloxorzComponentProps {
}

#[function_component(Bloxorz)]
fn bloxorz_component(BloxorzComponentProps {}: &BloxorzComponentProps) -> Html {

    html! { 
        <div id="bloxorz">
            <p> { "Cool math games" } </p>
            <iframe src="https://www.miniplay.com/embed/bloxorz" width="560" height="315" frameborder="0" allowfullscreen=true></iframe>
        </div>
         }
}

#[derive(PartialEq, Properties)]
struct TaxiComponentProps {
}

#[function_component(Taxi)]
fn taxi_component(TaxiComponentProps {}: &TaxiComponentProps) -> Html {

    html! { 
        <div id="bloxorz">
        <p> { "Park the taxi game" } </p>
        <iframe src="https://1000webgames.com/games/parkthetaxi/html5/" width="560" height="315" frameborder="0" scrolling="no" allowfullscreen=true></iframe>
        </div>
         }
}