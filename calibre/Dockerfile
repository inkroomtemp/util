FROM python
RUN apt update -y && apt install -y git curl wget gcc g++ make pkg-config sudo cmake  gettext 
ENV USER root
WORKDIR /app
RUN git clone https://github.com/kovidgoyal/bypy.git
RUN git clone https://github.com/kovidgoyal/calibre.git
WORKDIR /app/calibre
RUN python ./setup/unix-ci.py install
RUN python ./setup/unix-ci.py bootstrap && env
RUN python ./setup.py bootstrap