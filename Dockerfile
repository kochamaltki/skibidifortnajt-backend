FROM ubuntu:trusty
RUN sudo apt-get -y update
RUN sudo apt-get -y upgrade
RUN sudo apt-get install -y sqlite3 libsqlite3-dev curl
WORKDIR /usr/src/backend
SHELL ["/bin/bash", "-c"]
RUN ls -a /
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source "/.cargo/env"
RUN rustc -V
RUN mkdir -p /media/images
COPY . .
RUN /usr/bin/sqlite3 projekt-db < setup.sql
RUN ls -a
RUN cargo build
CMD cargo run
