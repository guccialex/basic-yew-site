# basic-yew-site

Base template for a full stack server entirely in rust

Run the ./build.sh in order to build the wasm files, before you push to github if you're going to deploy it with the docker file



How it works:

In the the root directory, there are three rust projects

Yew, which is compiled with trunk to render a dist folder that contains the files needed to serve the frontend of the application on the browser

Server, which serves the files in the dist folder, and any other backend services

Shared, which contains serializable structures visible to both the yew and server, so that they can send and receive data between each other


build.sh

Builds the files needed for distribution in /yew/dist

Copies that dist folder to the server

Then builds the server and runs it and serves it on port 8080





To build and deploying on Google cloud run:

1. Copy the code here, put it in a repository connected to your github account
2. After editing the code, run build.sh before commiting it to your repo to use, to update the dist contents in the server directory with the up to date generated wasm code and static files of the yew application
3. Create a google cloud account and a new project
4. Enable the "cloud build api" for that project
5. Create a new trigger in cloud build, referencing that project, set it to build with the dockerfile, set the build timeout to 1800 seconds, set the trigger invocation to manual
6. Run the trigger and wait ~20 minutes for it to build
7. Go to cloud run
8. Create a service using the image created at the run step, click "allow unauthenticated invocations"
9. Give it a minute, click on the link the service exposes to find the app on the web

To connect a service to a domain name:

1. Buy a domain name on the google account associated with the gcp account
2. On the cloud build click "manage custom domains"
3. Add a mapping, add the service, the domains you own that google can see should be there, select the one you want
4. Go to https://domains.google.com/registrar/ click "manage" for the domain you mapped the service to
5. Go to the dns page and add the custom resource records it tells you to add
6. Wait like an hour, the domain you connected it to will serve the app