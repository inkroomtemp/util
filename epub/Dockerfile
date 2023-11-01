FROM ubuntu:20.04
WORKDIR /app
RUN sed -i "s@http://.*archive.ubuntu.com@http://mirrors.huaweicloud.com@g" /etc/apt/sources.list  \
    && sed -i "s@http://.*security.ubuntu.com@http://mirrors.huaweicloud.com@g" /etc/apt/sources.list
RUN export DEBIAN_FRONTEND=noninteractive \
    && apt-get update \
    && apt-get install -y tzdata \
    && ln -fs /usr/share/zoneinfo/Asia/Shanghai /etc/localtime \
    && dpkg-reconfigure --frontend noninteractive tzdata
#COPY source.list /etc/apt/sources.list
RUN #export DEBIAN_FRONTEND=noninteractive && apt update -y && apt upgrade -y
RUN #sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories && apk add vim git zsh openssh-server openssh-client bash wget curl
RUN apt install -y ssh openssh-client openssh-server vim git zsh pip
RUN apt install -y curl
RUN REPO=sjtug/ohmyzsh REMOTE=https://git.sjtu.edu.cn/${REPO}.git sh -c "$(curl -fsSL https://git.sjtu.edu.cn/sjtug/ohmyzsh/-/raw/master/tools/install.sh\?inline\=false)" \
     && sed -i "s@/bin/bash@/bin/zsh@" /etc/passwd \
      && git config --global i18n.logoutputencoding utf-8 \
      && git config --global i18n.commitencoding utf-8 \
      && git config --global core.quotepath false

ENV LANG C.UTF-8
#RUN adduser --disabled-password cpp  &&  \
RUN sed -i "s@#PermitRootLogin prohibit-password@PermitRootLogin yes@g" /etc/ssh/sshd_config & sed -i "s/#PermitRootLogin.*/PermitRootLogin yes/g" /etc/ssh/sshd_config
RUN echo "cpp ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
#RUN echo "cpp:cpp" | chpasswd
RUN echo "root:cpp" | chpasswd
RUN cd /etc/ssh && ssh-keygen -A

RUN cd /app &&git clone https://github.com/lightnovel-center/linovelib2epub.git
#RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys  EA8CACC073C3DB2A
#RUN echo "deb https://launchpad.proxy.ustclug.org/linuxuprising/shutter/ubuntu focal main" >> /etc/apt/sources.list && echo "deb-src https://launchpad.proxy.ustclug.org/linuxuprising/shutter/ubuntu focal main" >> /etc/apt/sources.list && cat /etc/apt/sources.list && apt update -y && apt upgrade -y
RUN apt install -y software-properties-common
RUN add-apt-repository ppa:deadsnakes/ppa && echo ok2
RUN apt install -y python3.12
RUN update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.12 1 && update-alternatives --config python3 && python3 --version
RUN apt install -y python3.12-venv
RUN apt install -y python3.12-distutils
RUN cd /app/linovelib2epub/ && python3.12 -m venv venv && chmod +x ./venv/bin/activate && ./venv/bin/activate &&  python3.12 -m pip install -r requirements_dev.txt && python3.12 -m pip install -e .

# /usr/sbin/sshd -p 22
#CMD tail -f /dev/null
 CMD /etc/init.d/ssh restart  && /bin/zsh
# python3 -m venv venv && chmod +x ./venv/bin/activate && ./venv/bin/activate && python3 -m pip install -r requirements.txt -i https://mirrors.aliyun.com/pypi/simple && python3 -m pip install -e . && pip3 install argparse -i https://mirrors.aliyun.com/pypi/simple