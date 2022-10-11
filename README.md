# zerocal 🚫📆

Welcome to zerocal, the _serverless calendar_.  
It allows you to create calendar invites from the convenience of your terminal!  
🔗 Here's my [blog post about the project](https://endler.dev/2022/zerocal/).

## Usage

```sh
curl https://zerocal.shuttleapp.rs?start=2022-11-04+20:00&duration=3h&title=Birthday&des
cription=paaarty > party.ics
open party.ics
```

## Web UI

You can also use the web UI at https://zerocal.shuttleapp.rs

![web ui](assets/ui.png)

## Self-hosting

You can also self-host zerocal.
To do so, compile the binary with `cargo build --release --features local` and
run it with `./target/release/zerocal`.
The server will listen on port 8000 by default.

## Contributing

Please check the issue tracker for contribution ideas. Any pull request is welcome. ❤️

To run a local development version install [cargo-watch](https://crates.io/crates/cargo-watch)
and then run

```
make local
```

You can also run a dev version on shuttle.rs with

```
make dev
```

## Derivatives

Was your project inspired by zerocal? Add it here!

- [kiwical](https://github.com/maheshsundaram/kiwi) - Kiwi Calendar built with Typescript on Deno Deploy.

## Credits

This app was built with the help of 🚀 [shuttle.rs](https://www.shuttle.rs/),
the web application platform for Rust.
