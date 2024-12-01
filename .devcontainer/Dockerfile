FROM ubuntu:latest

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get purge -y imagemagick imagemagick-6-common \
    && apt install -y git build-essential libudev-dev libssl-dev musl-dev pkg-config sudo wget zsh \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*


# ARG USERNAME="ubuntu"
# ARG USER_UID=1000
# ARG USER_GID=${USER_UID}


# RUN groupadd --gid ${USER_GID} ${USERNAME} && \
#     useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} && \
#     usermod -a -G dialout "${USERNAME}"

# USER "${USERNAME}"

RUN sh -c "$(wget https://raw.github.com/robbyrussell/oh-my-zsh/master/tools/install.sh -O -)"  && \
    git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

COPY .p10k.zsh ${HOME}

