# clap-fundsp

An example how to plug FunDSP into a nih-plug template.

This is a modified version of Gain from: https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs

## How to compile the sources:

```
cargo xtask bundle -p clap-fundsp --release
```

That's it.  The plugin is in `target/bundled/clap-fundsp.clap`.
