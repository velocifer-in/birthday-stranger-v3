use dioxus::html::a::r#type;
use dioxus::html::g::crossorigin;
use dioxus::html::link::r#as;
use dioxus::prelude::*;
use gloo_timers::callback::{Interval, Timeout};
use gloo_utils::document;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Math;
use web_sys::{
    js_sys, window, AddEventListenerOptions, CssStyleDeclaration, Document, HtmlElement,
    MouseEvent, Touch, TouchEvent, Url, UrlSearchParams,
};

const BANNER_IMG: Asset = asset!("/assets/banner.png");
const FONT_FILE: Asset = asset!("/assets/fonts/Komigo3D-Regular.ttf");

pub fn random_color() -> &'static str {
    const COLORS: [&str; 5] = ["#ff4d4d", "#4da6ff", "#33cc33", "#ffcc00", "#cc66ff"];
    let index = (js_sys::Math::random() * COLORS.len() as f64).floor() as usize;
    COLORS[index]
}

pub fn random_position() -> f64 {
    js_sys::Math::random() * 100.0
}

pub fn create_snowflakes() {
    let document = document();
    let body = document.body().unwrap();

    for _ in 0..40 {
        let snow = document.create_element("div").unwrap();
        snow.set_class_name("snowflake");

        let style = snow
            .dyn_ref::<HtmlElement>()
            .expect("Expected HtmlElement")
            .style();

        let left = format!("{}vw", js_sys::Math::random() * 100.0);
        let duration = format!("{}s", js_sys::Math::random() * 4.0 + 5.0);
        let font_size = "18px";
        let opacity = format!("{}", js_sys::Math::random());

        style.set_property("left", &left).unwrap();
        style.set_property("animation-duration", &duration).unwrap();
        style.set_property("font-size", font_size).unwrap();
        style.set_property("opacity", &opacity).unwrap();

        snow.set_text_content(Some("❄"));

        body.append_child(&snow).unwrap();
    }
}
pub fn create_spark(x: f64, y: f64, color: &str) {
    let doc = document();
    let body = doc.body().unwrap();

    let spark = doc.create_element("div").unwrap();
    spark.set_class_name("spark");

    let style = spark
        .dyn_ref::<HtmlElement>()
        .expect("Expected HtmlElement")
        .style();

    style.set_property("background-color", color).unwrap();
    style.set_property("left", &format!("{x}vw")).unwrap();
    style.set_property("top", &format!("{y}vh")).unwrap();
    style
        .set_property("scale", &format!("{}", js_sys::Math::random() * 2.0 + 2.0))
        .unwrap();

    // Random polar direction
    let angle = js_sys::Math::random() * std::f64::consts::PI * 4.0;
    let radius = js_sys::Math::random() * 35.0;

    let dx = format!("{}vw", angle.cos() * radius);
    let dy = format!("{}vh", angle.sin() * radius);

    style.set_property("--x", &dx).unwrap();
    style.set_property("--y", &dy).unwrap();

    // Remove on animation end
    let spark_clone = spark.clone();
    let closure = Closure::wrap(Box::new(move || {
        let _ = spark_clone.remove();
    }) as Box<dyn Fn()>);

    spark
        .add_event_listener_with_callback("animationend", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
    body.append_child(&spark).unwrap();
}

pub fn launch_firework() {
    let doc = document();
    let body = doc.body().unwrap();
    let color = random_color();

    let firework = doc.create_element("div").unwrap();
    firework.set_class_name("firework");

    let style = firework
        .dyn_ref::<HtmlElement>()
        .expect("Expected HtmlElement")
        .style();

    style
        .set_property(
            "font-size",
            &format!("{}px", js_sys::Math::random() * 10.0 + 30.0),
        )
        .unwrap();
    style.set_property("background-color", color).unwrap();
    style
        .set_property("box-shadow", &format!("0 0 20px {}", color))
        .unwrap();
    style
        .set_property("left", &format!("{}vw", random_position()))
        .unwrap();
    style
        .set_property(
            "bottom",
            &format!("{}vh", 15.0 + js_sys::Math::random() * 30.0),
        )
        .unwrap();

    let firework_clone = firework.clone();
    let color = color.to_string();

    let closure = Closure::wrap(Box::new(move || {
        let style = firework_clone.dyn_ref::<HtmlElement>().unwrap().style();

        let x = style
            .get_property_value("left")
            .unwrap_or("0".into())
            .replace("vw", "")
            .parse::<f64>()
            .unwrap_or(0.0);

        let y = style
            .get_property_value("bottom")
            .unwrap_or("0".into())
            .replace("vh", "")
            .parse::<f64>()
            .unwrap_or(0.0);

        for _ in 0..30 {
            create_spark(x, y, &color);
        }

        firework_clone.remove();
    }) as Box<dyn Fn()>);

    firework
        .add_event_listener_with_callback("animationend", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
    body.append_child(&firework).unwrap();
}

pub fn spawn_balloon() {
    let doc = document();
    let body = doc.body().unwrap();

    let balloon = doc.create_element("div").unwrap();
    balloon.set_class_name("balloon");

    let color = random_color();
    let size = 30.0 + js_sys::Math::random() * 30.0;

    let style = balloon
        .dyn_ref::<HtmlElement>()
        .expect("Expected HtmlElement")
        .style();

    style.set_property("background-color", color).unwrap();
    style.set_property("width", &format!("{}px", size)).unwrap();
    style
        .set_property("height", &format!("{}px", size * 1.3))
        .unwrap();
    style
        .set_property("left", &format!("{}vw", random_position()))
        .unwrap();

    body.append_child(&balloon).unwrap();

    // Pop after 8 seconds
    let balloon_clone = balloon.clone();
    let balloon_click = balloon.clone();
    let balloon_touch = balloon.clone();

    let color_clone = color.to_string();
    Timeout::new(8000, move || {
        if js_sys::Math::random() < 0.3 {
            let x =
                js_sys::Math::random() * window().unwrap().inner_width().unwrap().as_f64().unwrap();
            let y = js_sys::Math::random()
                * window().unwrap().inner_height().unwrap().as_f64().unwrap()
                * 0.5;
            create_pop(x, y, &color_clone);
        }
        balloon_clone.remove();
    })
    .forget();

    // Click event
    {
        let balloon = balloon_click.clone();
        let color = color.to_string();

        let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let x = e.client_x() as f64;
            let y = e.client_y() as f64;
            let _ = balloon.remove();
            create_pop(x, y, &color);
        }) as Box<dyn FnMut(_)>);

        balloon_click
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // Touch event
    {
        let balloon = balloon_touch.clone();
        let color = color.to_string();

        let closure = Closure::wrap(Box::new(move |e: web_sys::TouchEvent| {
            e.prevent_default();
            let touch = e.touches().item(0).unwrap();
            let x = touch.client_x() as f64;
            let y = touch.client_y() as f64;
            let _ = balloon.remove();
            create_pop(x, y, &color);
        }) as Box<dyn FnMut(_)>);

        let mut opts = AddEventListenerOptions::new();
        opts.set_passive(true);
        balloon_touch
            .add_event_listener_with_callback_and_add_event_listener_options(
                "touchstart",
                closure.as_ref().unchecked_ref(),
                &opts,
            )
            .unwrap();
        closure.forget();
    }
}

pub fn create_pop(x: f64, y: f64, color: &str) {
    let doc = document();
    let body = doc.body().unwrap();

    for _ in 0..10 {
        let pop = doc.create_element("div").unwrap();
        pop.set_class_name("pop");
        pop.set_text_content(Some("❄"));

        let style = pop.dyn_ref::<HtmlElement>().unwrap().style();

        let offset_x = js_sys::Math::random() * 100.0 - 50.0;
        let offset_y = js_sys::Math::random() * 100.0 - 50.0;
        let scale = 0.8 + js_sys::Math::random();

        style.set_property("position", "absolute").unwrap();
        style.set_property("left", &format!("{}px", x)).unwrap();
        style.set_property("top", &format!("{}px", y)).unwrap();
        style.set_property("color", color).unwrap();
        style
            .set_property(
                "transform",
                &format!("translate({offset_x}px, {offset_y}px) scale({scale})"),
            )
            .unwrap();

        body.append_child(&pop).unwrap();
        animate_pop(pop.clone());

        Timeout::new(1000, move || {
            let _ = pop.remove();
        })
        .forget();
    }

    // Add message
    if js_sys::Math::random() < 0.3 {
        let messages = [
            "You are amazing!",
            "Smile, it's your day!",
            "Keep shining!",
            "Wishing you joy!",
        ];
        let msg = messages[(js_sys::Math::random() * messages.len() as f64).floor() as usize];

        let div = doc.create_element("div").unwrap();
        div.set_text_content(Some(msg));

        let style = div.dyn_ref::<HtmlElement>().unwrap().style();

        style.set_property("position", "absolute").unwrap();
        style.set_property("left", &format!("{}px", x)).unwrap();
        style
            .set_property("top", &format!("{}px", y - 30.0))
            .unwrap();
        style.set_property("color", color).unwrap();
        style.set_property("font-size", "14px").unwrap();
        style.set_property("font-weight", "bold").unwrap();
        style.set_property("z-index", "9999").unwrap();
        style.set_property("text-shadow", "0 0 5px #000").unwrap();

        body.append_child(&div).unwrap();
        Timeout::new(2000, move || {
            let _ = div.remove();
        })
        .forget();
    }
}

pub fn animate_pop(pop: web_sys::Element) {
    let window = web_sys::window().unwrap();
    let document = web_sys::window().unwrap().document().unwrap();
    let html = pop.dyn_ref::<HtmlElement>().unwrap();

    let drift_x = (js_sys::Math::random() - 0.5) * 4.0;
    let drift_y = -(1.0 + js_sys::Math::random());

    let x = html
        .style()
        .get_property_value("left")
        .unwrap()
        .replace("px", "")
        .parse::<f64>()
        .unwrap_or(0.0);
    let y = html
        .style()
        .get_property_value("top")
        .unwrap()
        .replace("px", "")
        .parse::<f64>()
        .unwrap_or(0.0);

    let pos = Rc::new(RefCell::new((x, y)));
    let pop_clone = pop.clone();
    let pos_clone = pos.clone();

    Interval::new(50, move || {
        let (mut x, mut y) = *pos_clone.borrow();

        x += drift_x;
        y += drift_y;

        let style = pop_clone.dyn_ref::<HtmlElement>().unwrap().style();
        style.set_property("left", &format!("{x}px")).ok();
        style.set_property("top", &format!("{y}px")).ok();

        *pos_clone.borrow_mut() = (x, y);

        let w = window.inner_width().unwrap().as_f64().unwrap();
        let h = window.inner_height().unwrap().as_f64().unwrap();

        if x < -10.0 || x > w + 10.0 || y < -10.0 || y > h + 10.0 {
            let _ = pop_clone.remove();
        }
    })
    .forget();
}

pub fn spawn_firefly_swarm() {
    let doc = document();
    let body = doc.body().unwrap();
    let swarm_size = 5;

    for _ in 0..swarm_size {
        let firefly = doc.create_element("div").unwrap();
        firefly.set_class_name("firefly");

        let duration = 5.0 + js_sys::Math::random() * 3.0;
        let alt_duration = 10.0 + js_sys::Math::random() * 5.0;
        let size = 3.0 + js_sys::Math::random() * 4.0;
        let x = random_position();
        let y = random_position();

        let style = firefly.dyn_ref::<HtmlElement>().unwrap().style();

        style
            .set_property(
                "animation-duration",
                &format!("{duration}s, {alt_duration}s"),
            )
            .unwrap();
        style.set_property("width", &format!("{size}px")).unwrap();
        style.set_property("height", &format!("{size}px")).unwrap();
        style.set_property("left", &format!("{x}vw")).unwrap();
        style.set_property("top", &format!("{y}vh")).unwrap();

        body.append_child(&firefly).unwrap();

        let firefly_clone = firefly.clone();
        Timeout::new((duration * 2000.0) as u32, move || {
            firefly_clone.remove();
        })
        .forget();
    }
}

pub fn attach_mouse_fireflies() {
    let cooldown = Rc::new(Cell::new(false));
    let cooldown_clone = cooldown.clone();

    let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        if cooldown_clone.get() {
            return;
        }
        cooldown_clone.set(true);

        let doc = document();
        let firefly = doc.create_element("div").unwrap();
        firefly.set_class_name("firefly");

        let style = firefly.dyn_ref::<HtmlElement>().unwrap().style();

        style
            .set_property("left", &format!("{}px", e.page_x()))
            .unwrap();
        style
            .set_property("top", &format!("{}px", e.page_y()))
            .unwrap();

        doc.body().unwrap().append_child(&firefly).unwrap();

        // Remove after 4s
        let clone = firefly.clone();
        Timeout::new(4000, move || {
            clone.remove();
        })
        .forget();

        // Cooldown reset
        let c = cooldown_clone.clone();
        Timeout::new(300, move || {
            c.set(false);
        })
        .forget();
    }) as Box<dyn FnMut(_)>);

    window()
        .unwrap()
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}

pub fn drop_gift_box() {
    let doc = document();
    let body = doc.body().unwrap();

    let gift = doc.create_element("div").unwrap();
    gift.set_text_content(Some("🎁"));
    gift.set_class_name("gift");

    let style = gift
        .dyn_ref::<HtmlElement>()
        .expect("Expected HtmlElement")
        .style();

    style.set_property("position", "absolute").unwrap();
    style
        .set_property("left", &format!("{}vw", js_sys::Math::random() * 100.0))
        .unwrap();
    style.set_property("top", "0px").unwrap();
    style.set_property("font-size", "30px").unwrap();
    style
        .set_property("animation", "floatGift 10s linear forwards")
        .unwrap();
    style.set_property("z-index", "1006").unwrap();
    style.set_property("cursor", "pointer").unwrap();

    // Handle click => alert + remove
    let gift_clone = gift.clone();
    let closure = Closure::wrap(Box::new(move || {
        web_sys::window()
            .unwrap()
            .alert_with_message("✨ Surprise! Your presence makes the world better.")
            .ok();
        let _ = gift_clone.remove();
    }) as Box<dyn FnMut()>);

    gift.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    body.append_child(&gift).unwrap();

    // Auto-remove after 5 seconds
    let gift_clone = gift.clone();
    Timeout::new(5000, move || {
        let _ = gift_clone.remove();
    })
    .forget();
}

pub fn display_random_poem() {
    let poems = [
        "May your day be full of cheer,\nWith laughter ringing far and near.",
        "Another year, a brand new start,\nFilled with joy and happy heart.",
        "Wishing you skies that are always blue,\nAnd dreams that surely do come true.",
        "Candles glow and stars align,\nTo bless your life with things divine.",
        "Let every smile on your face,\nBring sunshine to this lovely place.",
        "From dawn to dusk, from sun to moon,\nMay happiness find you very soon.",
        "On wings of joy this message flies,\nTo wish you bliss that never dies.",
        "With every candle's gentle light,\nYour future glows, your path is bright.",
        "This day is yours, a sparkling hue,\nA day to cherish all that's you.",
        "A wish upon a birthday star,\nFor love and peace just as you are.",
    ];

    let index = (Math::random() * poems.len() as f64).floor() as usize;
    let chosen = poems[index];

    let html_content = format!(
        r#"<div style="flex: 1; white-space: pre-wrap; overflow: hidden;">{}</div>
        <div style="text-align: right; color: gold; font-size: 0.9em; padding-top: 0.5em;">— With love, Phineas</div>"#,
        chosen
    );

    let doc: Document = document();
    if let Some(poem_el) = doc.get_element_by_id("poemBanner") {
        poem_el.set_inner_html(&html_content);
    }
}

pub fn init_birthday_scene() {
    let window = window().unwrap();
    let location = window.location();
    let search = location.search().unwrap_or_default();
    let url_params = UrlSearchParams::new_with_str(&search).unwrap();

    let name = url_params.get("name").unwrap_or("Friend".to_string());
    let colors = ["#ff4d4d", "#4da6ff", "#33cc33", "#ffcc00", "#cc66ff"];

    let styled_name_html = name
        .to_uppercase()
        .chars()
        .map(|ch| {
            let color = colors[(Math::random() * colors.len() as f64).floor() as usize];
            format!(
                r#"<span style="color: {}; font-family: 'Komigo', sans-serif; display: inline-block;">{}</span>"#,
                color, ch
            )
        })
        .collect::<Vec<_>>()
        .join("");

    if let Some(name_el) = document().get_element_by_id("nameField") {
        name_el.set_inner_html(&styled_name_html);
    }
}

#[component]
pub fn Hero() -> Element {
    use_effect(|| {
        init_birthday_scene();
        create_snowflakes();
        display_random_poem();
        Interval::new(2000, || {
            launch_firework();
        })
        .forget();
        Interval::new(1000, || {
            spawn_balloon();
        })
        .forget();
        Interval::new(15000, || {
            drop_gift_box();
        })
        .forget();
        || ();
    });

    rsx! {
        div {
            id: "hero",
            img { src: BANNER_IMG, alt:"Happy Birthday", class:"birthday-banner" }
            div { class:"personal-name-banner", span { id:"nameField", "Friend"} }
            div { class:"poem-banner", id:"poemBanner"}
        }
    }
}
