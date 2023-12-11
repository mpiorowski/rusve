rm -rf ./client/src/proto
mkdir ./client/src/proto
rm -rf ./service-users/src/proto
rm -rf ./service-notes/src/proto
rm -rf ./service-utils/src/proto

cd ./proto

# Client
npm run proto

# Server
cargo run
