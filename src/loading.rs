use bevy::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

#[derive(Resource)]
pub struct BevyReadySender(Option<futures::channel::oneshot::Sender<()>>);

impl BevyReadySender {
    pub fn spawn() -> Self {
        let (sender, receiver) = futures::channel::oneshot::channel();

        // this will not block, it's kicked off into the background on JS microtask queue
        spawn_local(async move {
            // this will fire when the bevy app is ready (at the end of `play_start_sys()`)
            receiver.await.unwrap();

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            document
                .get_element_by_id("canvas")
                .unwrap()
                .unchecked_into::<HtmlElement>()
                .style()
                .set_property("display", "block")
                .unwrap();

            document
                .get_element_by_id("loading")
                .unwrap()
                .unchecked_into::<HtmlElement>()
                .style()
                .set_property("display", "none")
                .unwrap();
        });

        Self(Some(sender))
    }

    pub fn send(&mut self) {
        if let Some(sender) = self.0.take() {
            sender.send(()).unwrap();
        }
    }
}
