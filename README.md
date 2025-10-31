# doppelganger-wrapper
Wrapper to easily build `doppelganger` and `doppelganger-parachain` nodes.

<div align="center">
<p>A cli tool to easily fork live networks (e.g kusama/polkadot).</p>
</div>

## :warning: :construction: Under Active Development :construction: :warning:

## Doppelganger

You can read the origina _inseption_ and design [here](https://github.com/paritytech/polkadot-sdk/issues/4230).
As brief introduction, the idea is to fork _live networks_ by creating a node that  _override_ the needed keys to
spawn a new network and continue with the block production (and keep the whole state). For that purpouse this node implements
a custom _block-import_ on top of babe/aura.

## Build

You can build all the binaries (doppelganger, doppelganger-parachain, polkadot-execute-worker, polkadot-prepare-worker) with this command:

```sh
SKIP_WASM_BUILD=1 cargo build --release
```

## Usage

At this moment this nodes are meant to be used as part of the [zombie-bite](https://github.com/pepoviola/zombie-bite) project.

