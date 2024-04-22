FROM python
RUN apt update -y && apt install -y git curl wget gcc g++ make pkg-config sudo cmake  gettext 

WORKDIR /app
RUN git clone https://github.com/kovidgoyal/bypy.git
RUN git clone https://github.com/kovidgoyal/calibre.git
WORKDIR /app/calibre
RUN adduser calibre
ENV USER calibre
RUN apt install -y libnss3-dev libxcomposite-dev libxdamage-dev libx11-dev libxtst-dev libxrandr-dev libxkbfile-dev && python ./setup/unix-ci.py install
RUN python ./setup/unix-ci.py bootstrap && env
RUN python ./setup.py bootstrap