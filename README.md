#clap-fundsp-example 

An example how to plug FunDSP into nih-plug.

This is a modified version of Gain from: https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs

What's changed is the implementation of `Default` for `Gain` where the FunDSP patch is constructed, and `<Gain as Plugin>::process()`.

# How to compile

1. Put this repo into `nih-plug/plugins/examples`
2. Add "plugins/examples/clap-fundsp" to nih-plug's Cargo.toml:

```toml
[workspace]
resolver = "2"
members = [
  "nih_plug_derive",
  "nih_plug_egui",
  "nih_plug_iced",
  "nih_plug_vizia",
  "nih_plug_xtask",

  "cargo_nih_plug",
  "xtask",

  "plugins/examples/gain",
  "plugins/examples/clap-fundsp",

...
etc.
```

3. Go to nih-plug's root dir and compile with:

```
cargo xtask bundle clap-fundsp --release
```


