# Dockerfile References: https://docs.docker.com/engine/reference/builder/
FROM rust:1.43 as builder
LABEL maintainer="Gary Moore <littlebunch@gmail.com>"

# Stage 1
RUN USER=root cargo new --bin graphql-rs
WORKDIR ./graphql-rs
ARG LOG_DIR=/var/logs/graphqlrs
RUN mkdir -p ${LOG_DIR}
ENV LOG_FILE_LOCATION=${LOG_DIR}/graphql.log
# Attached a volume for logging
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN cargo build --release

#### Stage 2
FROM debian:buster-slim
ARG APP=/usr/src/app
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && apt install -y \
libmariadb-dev \
libmariadbclient-dev \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
	APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /graphql-rs/target/release/graphql-rs ${APP}/graphql-rs

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}
#COPY --from=builder /app/graphqlrs .
#ADD api/dist/ ./dist
EXPOSE 8080
VOLUME [${LOG_DIR}]
CMD ["./graphql-rs"]

