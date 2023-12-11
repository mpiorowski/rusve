# Client
cd client
rm -rf src/lib/proto
mkdir src/lib/proto
npm run proto

# Server
cd ../proto
cargo run
