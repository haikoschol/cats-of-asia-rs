# Cats of Asia

This is a partial rewrite of [cats-of-asia](https://github.com/haikoschol/cats-of-asia) in Leptos.

I went just as far as necessary to satisfy my curiosity about Leptos and Rust/wasm in the browser
in general. It was fun and quite productive in the beginning. But then I struggled to properly
integrate with leaflet.js from Rust and ended up writing all of the popup rendering in
DOM-manipulating Javascript again.

Then I somehow ended up in borrow checker hell when implementing the `<Favorites>` component and
only got out of it with a lot of un-Leptos seeming code and hacks. The result works, but the
code sucks.

I couldn't be arsed to do the backend stuff, so this relies on fetching a JSON with image
metadata from https://catsof.asia/images.

Overall I think Leptos looks promising for complex frontends that don't need to interact with
JS libraries which don't fit into it's rendering philosophy. Considering that I still don't
know much about managing lifetimes in Rust I got pretty far with it. Also ChatGPT was much more
useful than I expected for this pretty niche tech.
