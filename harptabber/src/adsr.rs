use crate::audioplayback::FmOsc;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// takes attack(a) in ms, decay(d) in ms, sustain(s) in ms and sustain_volume(sl) as volume, release(r) in ms
pub fn adsr_curve(a: u32, d: u32, s: u32, sl: f32, r: u32) {
    let mut adsr_curve = Vec::new();
    let mut volume = 0.0;

    // create attack
    let attack_step = 1.0 / a as f32;
    (0..a).into_iter().for_each(|_| {
        adsr_curve.push(volume);
        volume += attack_step;
    });

    // create decay
    let decay_step: f32 = (1.0 - sl) / d as f32;
    (0..d).into_iter().for_each(|_| {
        adsr_curve.push(volume);
        volume -= decay_step;
    });

    // create sustain
    (0..s).into_iter().for_each(|_| {
        adsr_curve.push(volume);
    });

    // create release
    let release_step = volume / r as f32;
    (0..r).into_iter().for_each(|_| {
        adsr_curve.push(volume);
        volume -= release_step;
    });
    let js: JsValue = r.into();

    animation_loop(adsr_curve).unwrap();
}

#[wasm_bindgen]
pub fn animation_loop(adsr_curve: Vec<f32>) -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let max = adsr_curve.len();
    let mut i = 0;
    let fmosc = FmOsc::new().unwrap();
    fmosc.set_note(50);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i == max {
            let _ = f.borrow_mut().take();
            return;
        }

        let vol = adsr_curve[i];
        i += 1;
        fmosc.set_gain(vol);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
