FROM docker.io/rust:1.77.0

WORKDIR /usr/src/app

COPY . .

RUN apt-get update
RUN apt-get install -y curl tmux psmisc
RUN bash -c "source ~/.bashrc"

CMD ["tail", "-f", "/dev/null"]
