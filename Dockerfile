FROM ubuntu:trusty
RUN sudo apt-get -y update
RUN sudo apt-get -y upgrade
RUN sudo apt-get install -y sqlite3 libsqlite3-dev curl
WORKDIR /usr/src/backend
SHELL ["/bin/bash", "-c"]
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN ls -a /root
RUN source "/root/.cargo/env"
RUN /root/.cargo/bin/rustc -V
RUN mkdir -p ./media/images
RUN mkdir -p ./media/profile-pictures
COPY . .
RUN /usr/bin/sqlite3 projekt-db < setup.sql
RUN ls -a
RUN /root/.cargo/bin/cargo build --release
CMD /root/.cargo/bin/cargo run
