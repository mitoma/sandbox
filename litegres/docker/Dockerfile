FROM cimg/rust:1.84 AS build-http-proxy

RUN cargo install http-tunnel

FROM postgres:17

RUN apt update \
 && DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends \
    supervisor \
 && apt-get autoremove -y \
 && apt-get clean -y

COPY --from=build-http-proxy /home/circleci/.cargo/bin/http-tunnel /usr/local/bin/http-tunnel
COPY supervisord/supervisord.conf /etc/supervisor/conf.d/supervisord.conf

ENTRYPOINT ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
