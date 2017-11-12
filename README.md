coolsounds
==========

Experiment to learn Rust using the rodio audio library.

Currently plays:

* random chords from a pentatonic scale once per second, with varying degrees of chirpiness
* bass notes overlapped with a beat frequency vibe
* samples from Oliver Wendell Holmes

Install Rust:

    curl https://sh.rustup.rs -sSf | sh

Run code:

    cargo run --release

(Using more efficient release build seems to help with audio playback -- debug build uses a lot of CPU.)



