# WAV playback

Generate the Step 0 test file, then play it through the default output device:

```bash
cargo run -- generate
cargo run -- play
```

To play another WAV file, pass its path after `play`:

```bash
cargo run -- play path/to/audio.wav
```

Confirm that playback reaches the expected device, sounds at the expected pitch,
and stops when the file ends. This check requires audio hardware and is not run
in CI.
