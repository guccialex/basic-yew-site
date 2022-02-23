#this builds the program in /yew using trunk, creating the dist directory with the web code and assets
#copies it to /server
#then builds and runs the server

cd server

rm -rf ./dist

cd ..

cd yew

trunk build --release

cd ..
cp -r ./yew/dist ./server/dist


cd server

cargo build --release

#cp ./target/release/server .

cargo run --release