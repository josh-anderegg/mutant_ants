# Only run this on release, otherwise it's horrificly slow
cargo test --release --test draw_tests -- --include-ignored