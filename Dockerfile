#Using clearlinux as the base image
FROM clearlinux:latest

#Install Python 3 and pip
RUN swupd bundle-add python3-basic
RUN python3 -m ensurepip

WORKDIR /gitno

COPY . /gitno

#Run and build gitno
RUN python3 /gitno/setup.py build
RUN python3 /gitno/setup.py install

#Requirements for running gitno
ARG GITHUB_ACCESS_TOKEN
ENV GITHUB_ACCESS_TOKEN=${GITHUB_ACCESS_TOKEN}

