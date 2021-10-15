use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::audioplayback::FmOsc;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn adsr_curve() {
    use web_sys::console;
    let fmosc = FmOsc::new().unwrap();
    fmosc.set_note(50);
    fmosc.set_gain(0.0);
    let mut gain = 0.0;
    let mut note = 0;
    let window = window();
    let a = Closure::wrap(Box::new(move || {
        fmosc.set_gain(gain);
        gain += 0.1;
        note += 1;
        let js: JsValue = note.into();
        console::log_1(&js);
    }) as Box<dyn FnMut()>);
    (0..1000).step_by(100).into_iter().for_each(|step| {
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), step)
            .unwrap();
    });
    a.forget();
}
