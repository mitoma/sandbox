FROM gcr.io/distroless/cc:nonroot

ARG ARTIFACT_DIR
COPY ${ARTIFACT_DIR}/backend /backend
COPY ${ARTIFACT_DIR}/frontend /frontend
COPY contents /contents
COPY contents_hello /contents_hello

ENTRYPOINT [ "/backend/backend", \
             "--contents-file-path=/contents", \
             "--static-file-path=/frontend", \
             "--static-file-path-for-hello=/contents_hello", \
             "--address=0.0.0.0"]