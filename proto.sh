# Client
cd client
rm -rf src/lib/proto
mkdir src/lib/proto
npm run proto

# Service Users Rust
cd ../service-users-rust
cargo run --bin proto

# Service Utils Rust
cd ../service-utils-rust
cargo run --bin proto

# Service Notes Rust
cd ../service-notes-rust
cargo run --bin proto

# Service Users Go
cd ../service-users-go
rm -rf proto
mkdir proto
protoc --go_out=./proto --go_opt=paths=source_relative \
    --go-grpc_out=./proto --go-grpc_opt=paths=source_relative \
    --proto_path=../proto \
    ../proto/*.proto

# Service Utils Go
cd ../service-utils-go
rm -rf proto
mkdir proto
protoc --go_out=./proto --go_opt=paths=source_relative \
    --go-grpc_out=./proto --go-grpc_opt=paths=source_relative \
    --proto_path=../proto \
    ../proto/*.proto

# Service Notes Go
cd ../service-notes-go
rm -rf proto
mkdir proto
protoc --go_out=./proto --go_opt=paths=source_relative \
    --go-grpc_out=./proto --go-grpc_opt=paths=source_relative \
    --proto_path=../proto \
    ../proto/*.proto

