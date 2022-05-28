#!/bin/bash -e
cargo build --release --features runtime-benchmarks
./target/release/dfinn-node benchmark --chain dev --list


install -d benchout
for i in `./target/release/dfinn-node benchmark --chain dev --list | sed s/,.*// |sort |uniq` ; do
   echo Try $i
   echo ./target/release/dfinn-node benchmark \
      --chain dev \
      --execution wasm \
      --wasm-execution compiled \
      --pallet=$i \
      --extrinsic="*" \
      --steps 50 \
      --repeat 20 \
      --output=benchout/$i.rs
done
#      --template=templates/orml-weight-template.hbs \
