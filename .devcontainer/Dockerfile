FROM cimg/rust:1.62-node

RUN curl https://apt.releases.hashicorp.com/gpg | sudo apt-key --keyring /usr/share/keyrings/hashicorp-archive-keyring.gpg add - \
 && curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo apt-key --keyring /usr/share/keyrings/cloud.google.gpg add - \
 && echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/hashicorp.list \
 && echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] https://packages.cloud.google.com/apt cloud-sdk main" | sudo tee /etc/apt/sources.list.d/google-cloud-sdk.list \
 && sudo apt update \
 && sudo apt install -y \
    terraform \
    google-cloud-sdk \
 && sudo apt-get autoremove -y \
 && sudo apt-get clean -y

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN sudo groupadd --gid $USER_GID $USERNAME \
    && sudo useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && sudo sh -c "echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME" \
    && sudo chmod 0440 /etc/sudoers.d/$USERNAME

RUN sudo mv /home/circleci/.rustup /home/$USERNAME/.rustup \
 && sudo mv /home/circleci/.cargo /home/$USERNAME/.cargo \
 && sudo sh -c "echo \"source $HOME/.cargo/env\" >> /home/$USERNAME/.bashrc" \
 && sudo chown -R $USERNAME:$USERNAME /home/$USERNAME/.rustup \
 && sudo chown -R $USERNAME:$USERNAME /home/$USERNAME/.cargo

USER $USERNAME
