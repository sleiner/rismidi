# rismidi is a collection of small MIDI plugins

These plugins are modelled after the legendary [Piz MIDI](https://www.thepiz.org/plugins/?p=pizmidi) plugin collection.
They are designed to be really small, stable and perform a single job well.

This repository contains several crates:

- [rismidi](./rismidi/) contains common utility code for all plugins.
- Subdirectories of [plugins/](./plugins/) contain one plugin each.
- [validate_vst3](./validate_vst3/) tests the plugins using the [VST3 SDK](https://github.com/steinbergmedia/vst3sdk).
- [xtask](./xtask) is a Cargo subcommand [automating some common development tasks](https://github.com/matklad/cargo-xtask/).
