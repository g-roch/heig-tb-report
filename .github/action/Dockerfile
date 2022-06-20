# Container image that runs your code
FROM texlive/texlive:latest

RUN apt-get update && apt-get install -y zip cargo && apt-get clean

ENV HOME "/home/tmp"
RUN mkdir /home/tmp
RUN chmod 0777 /home/tmp

#ADD https://sh.rustup.rs rustupinstaller
#RUN sh rustupinstaller -y
#ENV PATH="/root/.cargo/bin:$PATH"
#RUN cargo --version
#RUN chmod ugo+x /root
#RUN chmod -R ugo+rwx /root/.cargo
#RUN chmod ugo+rwx /root/.rustup
#RUN ls -la /root/.rustup
#RUN ln -s /root/.cargo /root/.rustup .

## Copies your code file from your action repository to the filesystem path `/` of the container
#COPY entrypoint.sh /entrypoint.sh

## Code file to execute when the docker container starts up (`entrypoint.sh`)
#ENTRYPOINT ["/entrypoint.sh"]
