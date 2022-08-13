FROM ubuntu:focal-20220801

RUN apt update \
 && apt install -y \
    sudo \
    build-essential \
    jq \
    curl \
    wget \
    git \
 && apt-get autoremove -y \
 && apt-get clean -y

RUN curl https://apt.releases.hashicorp.com/gpg | apt-key --keyring /usr/share/keyrings/hashicorp-archive-keyring.gpg add - \
 && curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key --keyring /usr/share/keyrings/cloud.google.gpg add - \
 && curl -fsSL https://deb.nodesource.com/setup_16.x | bash - \
 && echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | tee /etc/apt/sources.list.d/hashicorp.list \
 && echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] https://packages.cloud.google.com/apt cloud-sdk main" | tee /etc/apt/sources.list.d/google-cloud-sdk.list \
 && apt update \
 && apt install -y \
    terraform \
    google-cloud-sdk \
    nodejs \
 && apt-get autoremove -y \
 && apt-get clean -y

ARG USERNAME=mitoma
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && sh -c "echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME" \
    && chmod 0440 /etc/sudoers.d/$USERNAME

USER $USERNAME

# install rust for localuser
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
