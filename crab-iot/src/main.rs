use log::info;
use std::fs::read;
use std::sync::{Arc, Mutex};
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::Message;
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let button_state = use_state(|| false);
    let ws = WebSocket::open("ws://localhost:4000").unwrap();
    let (mut tx, mut rx) = ws.split();
    let arc_tx = Arc::new(Mutex::new(tx));
    let button_state_clone = button_state.clone();

    use_effect(move || {
        let button_state = button_state_clone.clone();
       spawn_local(async move {
           while let Some(data) = rx.next().await {
               if let Ok(msg) = data {
                   if let Message::Text(data) = msg {
                       if data == "ON" {
                           button_state.set(true);
                       } else {
                           button_state.set(false);
                       }
                   }
               }
           }
       });
    });

    let on_click =  {
        let tx1 = arc_tx.clone();
        let button_state = button_state.clone();
        move |_| {
            let tx1 = tx1.clone();
            let button_state = button_state.clone();
            spawn_local(async move {
                let mut tx2 = tx1.lock().unwrap();
                let button_state = button_state.clone();
                if *button_state {
                    tx2.send(Message::Text(String::from("OFF"))).await.unwrap();
                } else {
                    tx2.send(Message::Text(String::from("ON"))).await.unwrap();
                }
            });
        }
    };

    let button_text = if *button_state { "TURN OFF" } else { "TURN ON" };

    html! {
        <div class="container mx-auto max-w-lg h-screen">
            <div class="flex flex-col items-center h-full justify-center">
                <p class="text-2xl my-12">{"This application will connect to the webserver"}</p>
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={on_click}>{button_text}</button>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
