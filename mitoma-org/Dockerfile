FROM debian:stable-20220801-slim

ARG USERNAME=mitomasan
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# https://code.visualstudio.com/remote/advancedcontainers/add-nonroot-user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME
USER $USERNAME

ARG ARTIFACT_DIR
COPY ${ARTIFACT_DIR}/backend /backend
COPY ${ARTIFACT_DIR}/frontend /frontend
COPY contents /contents

ENTRYPOINT [ "/backend/backend", \
             "--contents-file-path=/contents", \
             "--static-file-path=/frontend", \
             "--address=0.0.0.0"]