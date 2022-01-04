FROM rust:1.57 as builder

RUN USER=root cargo new --bin broadcaster
WORKDIR ./broadcaster
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/broadcaster*
RUN cargo build --release

FROM debian:bookworm-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /broadcaster/Rocket.toml ${APP}/Rocket.toml
COPY --from=builder /broadcaster/target/release/broadcaster ${APP}/broadcaster

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

EXPOSE 8000

CMD ["./broadcaster"]
