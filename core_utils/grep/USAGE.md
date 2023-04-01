`printf '%s\0' $(cat /path/to/your/file) | xargs -0 -I{} ./target/release/grep "{}" findstr`
