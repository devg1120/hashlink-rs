#rustc --crate-name hashlink --edition=2018 hashlink-sys/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=9999ccab1827d22e -C extra-filename=-9999ccab1827d22e --out-dir /home/gusa1120/_NEWDEV/hashlink-rs/target/debug/deps -C incremental=/home/gusa1120/_NEWDEV/hashlink-rs/target/debug/incremental -L dependency=/home/gusa1120/_NEWDEV/hashlink-rs/target/debug/deps -L native=./build/_deploy -L ./build/_deploy -l static=hlfull -l stdc++ -l hlfull -l dylib=ws2_32 -l dylib=user32
rustc --crate-name hashlink --edition=2018 hashlink-sys/src/lib.rs --error-format=human  --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=9999ccab1827d22e -C extra-filename=-9999ccab1827d22e --out-dir /home/gusa1120/_NEWDEV/hashlink-rs/target/debug/deps -C incremental=/home/gusa1120/_NEWDEV/hashlink-rs/target/debug/incremental -L dependency=/home/gusa1120/_NEWDEV/hashlink-rs/target/debug/deps -L native=./hashlink-sys/build/_deploy -L ./hashlink-sys/build/_deploy -l static=hlfull -l stdc++ -l hlfull -l dylib=ws2_32 -l dylib=user32
