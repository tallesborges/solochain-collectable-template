License: MIT-0


## Release

Polkadot SDK stable2409


```
cargo build --features=runtime-benchmarks
./target/debug/solochain-template-node benchmark pallet --pallet pallet_collectables --extrinsic="*" --chain=dev --output pallets/collectables/src/weights.rs --template ./.maintain/frame-weight-template.hbs;
```

```
cargo build --release --features=runtime-benchmarks
./target/release/solochain-template-node benchmark pallet --pallet pallet_collectables --extrinsic="*" --chain=dev --output pallets/collectables/src/weights.rs --template ./.maintain/frame-weight-template.hbs
```
