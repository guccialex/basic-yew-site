cd server

rm -rf ./dist

cd ..

cd yew

trunk build --release

cd ..
cp -r ./yew/dist ./server/dist


cd server

cargo build --release

cargo run --release