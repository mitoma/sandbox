FROM cimg/rust:1.94@sha256:53299b95824e0f15f53f6057f4103177bc9bbb266edf912493e58a8fb91d5c54 AS build-http-proxy

RUN cargo install http-tunnel

FROM postgres:18@sha256:4aabea78cf39b90e834caf3af7d602a18565f6fe2508705c8d01aa63245c2e20

RUN apt update \
 && DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends \
    supervisor \
 && apt-get autoremove -y \
 && apt-get clean -y

COPY --from=build-http-proxy /home/circleci/.cargo/bin/http-tunnel /usr/local/bin/http-tunnel
COPY supervisord/supervisord.conf /etc/supervisor/conf.d/supervisord.conf

ENTRYPOINT ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
