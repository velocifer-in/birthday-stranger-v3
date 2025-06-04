# Happy Birthday v3 🎉

A fun and interactive birthday celebration app built with **Dioxus** (Rust). It features fireworks, balloons,
snowflakes, a personalized greeting, custom fonts, and a poem banner — all running in a festive animated environment!

---

## 📁 Project Structure

```
happyBirthdayv3/
├── assets/               # Static assets like CSS, JS, fonts, images
│   ├── main.css          # Custom styles including font-face and animations
│   └── fonts/            # Contains Komigo or fallback Google fonts
├── public/               # Assets to be served directly (fonts/images)
├── src/                  # Rust source files
│   ├── main.rs           # Entry point of the app
│   ├── components/       # Reusable UI components
│   │   ├── mod.rs
│   │   └── hero.rs
│   └── views/            # Route-specific views
│       ├── mod.rs
│       ├── home.rs
│       └── blog.rs
├── Dioxus.toml           # Platform and asset configuration
├── Cargo.toml            # Rust dependencies and metadata
└── README.md             # You're reading it
```

---

## 🚀 Running the App

Start the development server:

```bash
dx serve
```

To run on a specific platform (e.g. web):

```bash
dx serve --platform web
```

---

## 🎨 Features

- Randomized birthday poems
- Interactive balloons (click/tap to pop)
- Fireworks and spark effects
- Falling snow and animated fireflies
- Personalized name banner using custom fonts
- Surprise gift drops with messages
- All effects written in Rust (wasm-bindgen + web-sys)

---

## 🖋️ Custom Fonts

Make sure your font is placed in `public/assets/fonts/` and defined in `assets/main.css` like:

```css
@font-face {
    font-family: 'Komigo';
    src: url('/assets/fonts/Komigo3D-Regular.ttf') format('truetype');
    font-display: swap;
}
```

---

## ✨ License

MIT — free to use and modify.

