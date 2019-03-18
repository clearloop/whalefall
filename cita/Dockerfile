FROM cita/cita-build:ubuntu-18.04-20190304

WORKDIR /cita

RUN git clone https://github.com/cryptape/cita.git \
        && cd cita \
        && git checkout -b master \
        && git submodule init \
        && git submodule update \
        && make release

ADD ./target/install /cita

EXPOSE 1439
