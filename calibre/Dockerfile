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
# 没有这些环境变量就会从系统重查到依赖，版本会对不上
ENV SW /sw/sw/
ENV PATH $SW/bin:$PATH
ENV CFLAGS -I$SW/include
ENV LDFLAGS -L$SW/lib
ENV LD_LIBRARY_PATH $SW/qt/lib:$SW/lib
ENV PKG_CONFIG_PATH $SW/lib/pkgconfig
ENV QMAKE $SW/qt/bin/qmake
ENV CALIBRE_QT_PREFIX $SW/qt
RUN ls $SW && python ./setup.py bootstrap
# 因为修改了环境变量，所以使用python已经不是镜像里原本的那个了，pip也就对不上，需要重新安装一份
RUN curl -sL https://bootstrap.pypa.io/get-pip.py -o get-pip.py && python get-pip.py && rm -f get-pip.py && python -m pip install pyyaml
RUN echo "import ssl" >> s.py && echo "ssl._create_default_https_context = ssl._create_unverified_context" >> s.py && cat ../bypy/bypy/chroot.py >> s.py && mv s.py ../bypy/bypy/chroot.py && cat ../bypy/bypy/chroot.py
RUN apt install -y genisoimage qemu-utils
#RUN pip install yaml && cd ../bypy && pip install yaml
RUN python ../bypy/ linux --arch 64 vm
RUN python setup.py build_dep linux && ls bypy/b/linux/
RUN python setup.py linux && ls dist