FROM ubuntu:alpine

WORKDIR /usr/src
ENTRYPOINT [ "cargo", "run" ]
CMD [ "8080" ]

LABEL author="west"
LABEL maintainer="west.sh.mail@gmail.com"