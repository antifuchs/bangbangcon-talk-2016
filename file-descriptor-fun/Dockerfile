FROM ubuntu:14.04
MAINTAINER Andreas Fuchs <asf@boinkor.net>

RUN apt-get update && apt-get install -y curl
RUN curl https://static.rust-lang.org/dist/rust-1.8.0-x86_64-unknown-linux-gnu.tar.gz | tar zxf - -C /opt && /opt/rust-1.8.0-x86_64-unknown-linux-gnu/install.sh
RUN apt-get install -y build-essential

ADD . ./src
RUN cd /src ; cargo build

CMD cd /src ; cargo test
