ARG arch x64

### Build Rust crates
FROM rust:alpine3.16 as rust-builder

WORKDIR /rust-builder

RUN apk add --no-cache musl-dev

COPY Cargo.lock Cargo.toml ./
COPY crates/ ./crates

RUN cargo build --release

RUN strip target/release/flagger

### Build JavaScript project
FROM node:18.6-alpine3.16 as node-builder

WORKDIR /node-builder

COPY package.json yarn.lock tailwind.config.js postcss.config.js ./

RUN yarn -s --frozen-lockfile

COPY tsconfig.json next.config.js ./
COPY www/ ./www
COPY lib/ ./lib

RUN yarn -s build

RUN ls -la

RUN yarn -s compile --targets node18-alpine-${arch}

### End platform
FROM alpine:3.16

WORKDIR /flagger

ENV NODE_ENV production

COPY --from=node-builder /node-builder/out/flagger-serve-www /usr/local/bin/flagger-serve-www
COPY --from=rust-builder /rust-builder/target/release/flagger /usr/local/bin/flagger

ENTRYPOINT ["flagger"]
CMD ["flagger", "--help"]
