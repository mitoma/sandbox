FROM gcr.io/distroless/cc:nonroot

ARG ARTIFACT_DIR
COPY ${ARTIFACT_DIR}/backend /backend
COPY ${ARTIFACT_DIR}/frontend /frontend
COPY contents /contents

ENTRYPOINT [ "/backend/backend", \
             "--contents-file-path=/contents", \
             "--static-file-path=/frontend", \
             "--address=0.0.0.0"]