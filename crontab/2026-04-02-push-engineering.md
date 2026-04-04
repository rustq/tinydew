# Push tinydew engineering branch at 13:12 UTC (3h from 10:12 UTC)
12 13 * * * cd /root/tinydew && export PATH="$HOME/.cargo/bin:$PATH" && cargo build --features interactive 2>&1 && cargo test 2>&1 && git add -A && git status
