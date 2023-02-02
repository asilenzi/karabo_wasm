rm default_*.profraw
rm *profdata
export RUSTFLAGS="-C instrument-coverage"
export TARGET=$(cargo test 2>&1 | grep Running | sed -E 's# +Running .+\((.+)\)#\1#')
rust-profdata merge -sparse default_*.profraw -o test.profdata
cargo-cov -- report --ignore-filename-regex='/.cargo/registry' --ignore-filename-regex='/tests' --ignore-filename-regex='/rustc' --object $TARGET --instr-profile=./test.profdata