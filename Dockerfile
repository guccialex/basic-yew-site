FROM rust:latest

WORKDIR /yew-app

#copy the server and shared directories into the container
COPY ./ .

WORKDIR /yew-app/server

RUN cargo install --path .

EXPOSE 8080

CMD server

#to build this, though you can also run the ./build
#run
#sudo (don't do sudo if you installed docker correctly) docker build .
#It'll say Successfully built <some number>
#and to run, exposing on your port 8080, so you can access at 0.0.0.0:8080 run
#sudo docker run -p 8080:8080 -it <the first 4 or so digits of that some number>