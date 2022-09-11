FROM ubuntu:jammy@sha256:2d7ecc9c5e08953d586a6e50c29b91479a48f69ac1ba1f9dc0420d18a728dfc5

RUN apt update \
 && DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends \
    sudo \
    build-essential \
    jq \
    curl \
    wget \
    git \
    ca-certificates \
    gnupg \
    lsb-release \
    less \
    unzip \
    shellcheck \
 && apt-get autoremove -y \
 && apt-get clean -y

RUN curl https://apt.releases.hashicorp.com/gpg                | apt-key --keyring /usr/share/keyrings/hashicorp-archive-keyring.gpg add - \
 && curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key --keyring /usr/share/keyrings/cloud.google.gpg add - \
 && curl https://download.docker.com/linux/ubuntu/gpg          | apt-key --keyring /usr/share/keyrings/docker.gpg add - \
 && curl -fsSL https://deb.nodesource.com/setup_16.x | bash - \
 && echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | tee /etc/apt/sources.list.d/hashicorp.list \
 && echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] https://packages.cloud.google.com/apt cloud-sdk main" | tee /etc/apt/sources.list.d/google-cloud-sdk.list \
 && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list \
 && apt update \
 && DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends \
    terraform \
    google-cloud-sdk \
    nodejs \
    docker-ce docker-ce-cli containerd.io docker-compose-plugin \
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

# install sver
ARG SVER_VERSION="v0.1.5"
RUN mkdir -p "$HOME/.local/bin" \
 && TEMPDIR=$(mktemp -d) \
 && FILE_NAME="sver_${SVER_VERSION}_linux_amd64.zip" \
 && FILE_URL="https://github.com/mitoma/sver/releases/download/${SVER_VERSION}/${FILE_NAME}" \
 && cd "$TEMPDIR" \
 && curl -L "$FILE_URL" -o "$FILE_NAME" \
 && unzip "$FILE_NAME" \
 && mv sver "$HOME/.local/bin" \
 && echo 'export PATH=$PATH:$HOME/.local/bin' >> $HOME/.bashrc \
 && rm -rf $TEMPDIR
