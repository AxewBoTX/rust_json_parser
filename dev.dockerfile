FROM docker.io/rust:1.77.0

# set the working directory
WORKDIR /usr/src/app

# copy all the project files onto the working directory
COPY . .

# basic container setup (according to my liking)
RUN apt-get update
RUN apt-get install -y curl
RUN apt-get install -y tmux psmisc
RUN bash -c "echo 'PATH="/usr/local/cargo/bin:$PATH"' >> ~/.bashrc"
RUN bash -c "source ~/.bashrc"

# run the tail command to keep the container running
CMD ["tail", "-f", "/dev/null"]
