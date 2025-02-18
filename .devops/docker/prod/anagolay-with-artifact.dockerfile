ARG GIT_LATEST_REVISION
FROM docker.io/bitnami/minideb

LABEL description="Production ready image for Anagolay: a platform for web3 Rights Management" \
	network.anagolay.image.type="main" \
	network.anagolay.image.authors="daniel@woss.io, devops-team@anagolay.network" \
	maintainer="daniel@woss.io" \
	network.anagolay.image.vendor="Anagolay Network" \
	network.anagolay.image.description="Anagolay: a platform for web3 Rights Management" \
	network.anagolay.image.source="https://gitlab.com/anagolay/anagolay/blob/${GIT_LATEST_REVISION}/.devops/prod/anagolay-with-artifact.dockerfile" \
	network.anagolay.image.documentation="https://gitlab.com/anagolay/anagolay"

RUN apt-get update \ 
&& apt-get install -y \ 
ca-certificates

COPY ./anagolay /usr/bin/anagolay
COPY ./anagolay.sha256 /
COPY ./LICENSE /

EXPOSE 30333 9933 9944

VOLUME ["/data"]

# check if executable works in this container
RUN anagolay --version

ENTRYPOINT  ["/usr/bin/anagolay"]
CMD  ["/usr/bin/anagolay"]


