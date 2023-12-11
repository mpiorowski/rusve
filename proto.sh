# Client
cd client
rm -rf src/lib/proto
mkdir src/lib/proto
npm run proto

# Service Users Rust
cd ../service-users
cargo run --bin proto

# Service Utils Rust
cd ../service-utils
cargo run --bin proto

# Service Notes Rust
cd ../service-notes
cargo run --bin proto
