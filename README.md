# clap-fundsp

clap-fundsp

Examples of using [FunDSP] with a [CLAP] plugin template.

This repository contains templates that differ based on the CLAP API wrapper for
Rust used to implement plugin functionality:

* `nih_plug-chord`: Plays an organ chord indefinitely. This is a modified
  version of the [Gain] example from [nih-plug].

A few examples how to use [FunDSP] with a [CLAP] plugin template.


[CLAP]: https://cleveraudio.org/

[FunDSP]: https://github.com/SamiPerttu/fundsp

[Gain]: https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs

[nih-plug]: https://github.com/robbert-vdh/nih-plug

## How to compile the plugins

We assume Rust Edition 2024 (available on *nightly* Rust 1.85).

Build the examples with:

```
cargo build -r
```

The compiled CLAP plugins are standard (C ABI) dynamic libraries located in the
`target/release` directory. Their filenames are OS-specific. For example:

```
target/release/libnih_plug_chord.so
```

This is the filename of the `nih_plug-chord` plugin on Linux. To use the plugin
in a CLAP host (e.g., a DAW), copy the compiled library to a location your host
can access and rename it to a more appropriate filename, such as:

```
nih_plug_chord.clap 
```
