use dioxus::prelude::*;
use tracing::debug;

use crate::components::buttons::ActionButton;

use palette::{Hsv, IntoColor, rgb::Srgb};

// Size of the sampling square in canvas pixels.
const SAMPLE_SIZE: u32 = 20;

#[component]
fn ColourInput(on_set: Callback<Hsv>) -> Element {
    let mut colour = use_signal(|| Hsv::new(0.0, 0.0, 0.0));

    // A unique ID shared between Rust and JS so the loop can detect unmount
    // without relying on querySelector finding (or not finding) a video element,
    // which is fragile when other video elements exist on the page.
    let stop_flag_id = use_memo(|| {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        format!("__colour_stop_{}", COUNTER.fetch_add(1, Ordering::Relaxed))
    });

    // On unmount: set the stop flag so the loop exits, and stop the camera
    // track directly via the stream reference stored on window.
    let flag_id_for_drop = stop_flag_id();
    use_drop(move || {
        let _ = document::eval(&format!(
            r#"
            window['{flag}'] = true;
            const stream = window['{flag}_stream'];
            if (stream) {{ stream.getTracks().forEach(t => t.stop()); }}
            delete window['{flag}_stream'];
            delete window['{flag}'];
            "#,
            flag = flag_id_for_drop,
        ));
    });

    let flag_id_for_future = stop_flag_id();
    use_future(move || {
        let flag_id = flag_id_for_future.clone();
        async move {
            let mut eval = document::eval(&format!(
                r#"
                window['{flag}'] = false;

                const video = document.querySelector('video');
                const stream = await navigator.mediaDevices.getUserMedia({{
                    video: {{ facingMode: 'environment' }}
                }});
                window['{flag}_stream'] = stream;
                video.srcObject = stream;

                const sz = {sz};
                let canvas = null;

                while (!window['{flag}']) {{
                    try {{
                        const width  = Number(video.videoWidth  || video.naturalWidth  || video.width);
                        const height = Number(video.videoHeight || video.naturalHeight || video.height);

                        if (!canvas || canvas.width !== width || canvas.height !== height) {{
                            canvas = new OffscreenCanvas(width, height);
                        }}

                        const ctx = canvas.getContext('2d');
                        ctx.drawImage(video, 0, 0);

                        const cx = Math.floor(width  / 2);
                        const cy = Math.floor(height / 2);
                        const px = ctx.getImageData(cx - sz / 2, cy - sz / 2, sz, sz).data;

                        let r = 0, g = 0, b = 0;
                        for (let i = 0; i < px.length; i += 4) {{
                            r += px[i];
                            g += px[i + 1];
                            b += px[i + 2];
                        }}
                        const n = px.length / 4;
                        dioxus.send(`${{Math.round(r / n)}},${{Math.round(g / n)}},${{Math.round(b / n)}}`);

                        await new Promise(resolve => setTimeout(resolve, 250));
                    }} catch(err) {{
                        console.log("ColourInput sampling error", err);
                        await new Promise(resolve => setTimeout(resolve, 200));
                    }}
                }}

                delete window['{flag}_stream'];
                delete window['{flag}'];
                "#,
                flag = flag_id,
                sz = SAMPLE_SIZE,
            ));

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
                        debug!("Invalid colour sample received");
                    }
                }
            }
        }
    });

    // Compute the live swatch colour as a CSS rgb() string for the overlay preview.
    let swatch_style = use_memo(move || {
        let rgb: Srgb = colour().into_color();
        format!(
            "background-color: rgb({}, {}, {})",
            (rgb.red * 255.0).round() as u8,
            (rgb.green * 255.0).round() as u8,
            (rgb.blue * 255.0).round() as u8,
        )
    });

    rsx! {
        div {
            class: "relative w-full bg-black overflow-hidden cursor-pointer",
            style: "aspect-ratio: 9 / 16;",
            onclick: move |_| { on_set(colour()); },
            video {
                class: "block w-full h-full object-contain",
                autoplay: true,
                playsinline: true,
            }

            // Target box: centred square showing the sampled region.
            div {
                class: "absolute inset-0 flex items-center justify-center pointer-events-none z-10",
                div {
                    class: "border-4 border-white ring-2 ring-black",
                    style: "width: 40px; height: 40px;",
                }
            }

            // Live colour swatch: bottom-right corner preview.
            div {
                class: "absolute bottom-2 right-2 w-8 h-8 border-2 border-white ring-1 ring-black pointer-events-none z-10",
                style: swatch_style(),
            }
        }
    }
}

#[component]
pub fn Colour(colour: Signal<(String, String, String)>) -> Element {
    let mut show = use_signal(|| false);

    rsx! {
        if show() {
            h1 { "Please scan colour" }
            ColourInput {
                on_set: move |c: Hsv| {
                    colour.set((
                        c.hue.into_inner().to_string(),
                        c.saturation.to_string(),
                        c.value.to_string(),
                    ));
                    show.set(false);
                },
            }
            ActionButton { on_click: move |_| show.set(false), "cancel" }
        } else {
            ActionButton { on_click: move |_| show.set(true), "scan" }
        }
    }
}
