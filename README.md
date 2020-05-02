# http-gpio

This is a small [platform daemon][article] written in Rust, exposing Linux's GPIO interface via a simple HTTP server.

Primarily, it is being used as a sample daemon for the purposes of the Mastering Embedded Linux series, but it should also be a useful example of [`gpio-cdev`][gpio-cdev] and [`warp`][warp].

Disclaimer: This code is not production ready.
If it breaks, you get to keep all the pieces.

[article]: https://www.thirtythreeforty.net/posts/2020/05/mastering-embedded-linux-part-5-platform-daemons/
[gpio-cdev]: https://github.com/rust-embedded/gpio-cdev
[warp]: https://github.com/seanmonstar/warp
