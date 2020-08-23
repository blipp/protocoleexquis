# Protocole Exquis – A Cryptographic Multi-Party Computation Party Game

A project to be continued. Meant to be hosted at [protocoleexquis.net](https://protocoleexquis.net/),
which only displays a mockup and an about page right now.

## TODO aka Future Work

* Finish the game!
* Add some actual crypto: Why not some actual Secure Multi-Party Computation that let's us hide the precious protocols from the server? Maybe some public-key crypto is enough?
* Game mode for more than two entities?
* Syntax highlighting? Emojis?
* Automatic security analysis?
* 3-dimensional version?

## Resources

Built with

* [Rust](https://www.rust-lang.org/) with the [Actix web framework](https://actix.rs/) and the [Diesel ORM](https://diesel.rs/)
* [Tachyons CSS toolkit](https://tachyons.io/)

The following examples, discussions, and blog posts inspired and helped
with this project so far:

* [actix' websocket chat broker example](https://github.com/actix/examples/tree/master/websocket-chat-broker)
* [actix-web: r2d2 + websockets (how to pass a connection pool to a websocket server)](https://github.com/actix/actix-web/issues/1273)
* [actix-web: WebSocket Connections List](https://github.com/actix/actix-web/issues/704)
* [Rust API with Diesel and r2d2 on MySQL](https://blog.sufrago.com/rust-api-with-diesel-and-r2d2-on-mysql/)

## Usage

Start the server with: `cargo run` and open [http://localhost:34787/](http://localhost:34787/)

## Build

There is a `build.sh` script and a `Dockerfile` which build the project
for a Debian stretch target, which happened to be my web hoster's OS.
Docker-compiling turned out to be easier than cross-compiling directly
on my machine, let alone building a fat binary.

## License

AGPL 3.0
