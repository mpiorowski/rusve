cd client
npm run proto
cd ../service-users
cargo run --bin proto
cd ../service-notes
cargo run --bin proto
cd ../service-utils
cargo run --bin proto
cd ../service-posts
protoc --go_out=./proto --go_opt=paths=source_relative \
    --go-grpc_out=./proto --go-grpc_opt=paths=source_relative \
    --proto_path=../proto \
    ../proto/*.proto

