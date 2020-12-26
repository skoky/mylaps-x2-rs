FROM debian:buster

RUN apt-get update \
    && apt-get -y install autoconf cmake make dmidecode \
    build-essential libc6 g++-multilib
