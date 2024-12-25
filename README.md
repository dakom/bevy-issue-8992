# Live demo

[https://dakom.github.io/bevy-issue-8992](https://dakom.github.io/bevy-issue-8992)

_note: the gltf assets are external, but the wasm itself is around 50MB... all in all, wait a while for things to load - no progress bar atm_

# About

Currently, Bevy only supports 1 directional light, even on WebGL targets that are not constrained by the Fog/Adreno problem

See https://github.com/bevyengine/bevy/issues/8992 and related issues for more info

This example changes the max directional lights to 10 on webgl, and shows a scene with 2 directional lights and fog.

(the change made to MAX_DIRECTIONAL_LIGHTS) is via a fork, included in a submodule here, just https://github.com/dakom/bevy/commit/eb816056bfb97e2700e72d001a1dad05cf2e90cd

In order to make this example run on the web, it also builds off a currently-open PR to add http support (https://github.com/bevyengine/bevy/pull/16366)

# Run in dev mode

```bash
trunk serve --port 8000 
```

# Build for release (happens automatically in CI)

```bash
trunk build --release --cargo-profile wasm-release
```