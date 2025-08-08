use dioxus::prelude::*;
use tracing::debug;

use crate::{components::buttons::ActionButton, forms::fields::ColourButton};

use palette::{Hsv, IntoColor, rgb::Srgb};

const TARGET_SVG: Asset = asset!("/assets/target.svg");

#[component]
fn Colourinput(on_set: Callback<Hsv>) -> Element {
    let mut colour = use_signal(|| Hsv::new(0.0, 0.0, 0.0));

    use_future(move || async move {
        let mut eval = document::eval(
            r#"
            // Define video as the video element. You can pass the entire element to the colour detector!
            const video = document.querySelector('video');

            // Get a stream for the rear camera, else the front (or side?) camera, and show it in the video element.
            video.srcObject = await navigator.mediaDevices.getUserMedia({ video: { facingMode: 'environment' } });

            // let rawValue = null;
            let canvas = null;

            while(true) {
                try {
                    const video = document.querySelector('video');
                    if (!video) {
                        return
                    }

                    const width  = Number(video['naturalWidth'] || video['videoWidth'] || video['width'])
                    const height = Number(video['naturalHeight'] || video['videoHeight'] || video['height'])

                    if (!canvas || canvas.width != width || canvas.height != height) {
                        canvas = new OffscreenCanvas(width, height);
                    }

                    const context = canvas.getContext('2d');
                    context.drawImage(video, 0, 0);

                    let p = context.getImageData(canvas.width/2, canvas.height/2, 1, 1).data;
                    let red = p[0];
                    let green = p[1];
                    let blue = p[2];
                    dioxus.send(`${red},${green},${blue}`);
                    await new Promise(r => setTimeout(r, 1000));
                }
                catch(err) {
                    console.log("failed", err);
                    //Wait till video is ready
                    //colourDetector.detect(video) might fail the first time
                    await new Promise(r => setTimeout(r, 200));
                }
            }
        "#,
        );

        while let Ok(value) = eval.recv::<String>().await {
            let split: Vec<&str> = value.split(",").collect();

            match split[..] {
                [r, g, b] => {
                    let r: f32 = r.parse().unwrap_or(0.0) / 255.0;
                    let g: f32 = g.parse().unwrap_or(0.0) / 255.0;
                    let b: f32 = b.parse().unwrap_or(0.0) / 255.0;
                    let c = Srgb::new(r, g, b);
                    let c: Hsv = c.into_color();
                    colour.set(c);
                }
                _ => {
                    debug!("Invalid color");
                }
            }
        }
    });

    rsx! {
        div {
            class: "relative",
            video {
                autoplay: true,
                playsinline: true,
            }
            div {
                class: "absolute w-full h-full top-0 left-0 opacity-40 z-20",
                onclick: move |_| {
                    on_set(colour());
                },
                img {
                    class: "w-full h-full",
                    src: TARGET_SVG
                }
            }
        }
        ColourButton {
            colour: colour(),
            name: "Set",
            on_click: move |colour| {
                on_set(colour);
            },
            selected: false
        }
    }
}

#[component]
pub fn Colour(colour: Signal<(String, String, String)>) -> Element {
    let mut show = use_signal(|| false);

    rsx! {
        if show() {
            h1 { "Please scan color"}
            Colourinput {
                on_set: move |c: Hsv| {
                    colour
                        .set((
                            c.hue.into_inner().to_string(),
                            c.saturation.to_string(),
                            c.value.to_string(),
                        ));
                    show.set(false);
                },
            }
            ActionButton {
                on_click: move |_| show.set(false),
                "cancel"
            }
        } else {
            ActionButton {
                on_click: move |_| show.set(true),
                "scan"
            }
        }
    }
}
