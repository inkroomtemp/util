FROM python
RUN apt update -y && apt install -y git curl wget gcc g++ make pkg-config sudo cmake  gettext 

WORKDIR /app
RUN git clone https://github.com/kovidgoyal/bypy.git
RUN git clone https://github.com/kovidgoyal/calibre.git
WORKDIR /app/calibre
# RUN adduser calibre
ENV USER root
# 解决 构建过程中要求不使用root
ENV QTWEBENGINE_DISABLE_SANDBOX 1
RUN apt install -y libnss3-dev libxcomposite-dev libxdamage-dev libx11-dev libxtst-dev libxrandr-dev libxkbfile-dev && python ./setup/unix-ci.py install
RUN python ./setup/unix-ci.py bootstrap && env

ENV SW /sw/sw/

ENV PATH $SW/bin:$PATH
ENV CFLAGS -I$SW/include
ENV LDFLAGS -L$SW/lib
ENV LD_LIBRARY_PATH $SW/qt/lib:$SW/lib
ENV PKG_CONFIG_PATH $SW/lib/pkgconfig
ENV QMAKE $SW/qt/bin/qmake
ENV CALIBRE_QT_PREFIX $SW/qt
RUN ls $SW && python ./setup.py bootstrap
RUN python setup.py build_dep linux && ls bypy/b/linux/
RUN python setup.py linux && ls dist