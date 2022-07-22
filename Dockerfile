### Build Rust crates
FROM rust:alpine3.16 as rust-builder

WORKDIR /rust-builder

RUN apk add --no-cache musl-dev

COPY Cargo.lock Cargo.toml ./
COPY crates/ ./crates

RUN cargo build --release

### Build JavaScript project
FROM node:18.6-alpine3.16 as node-builder

WORKDIR /node-builder

COPY package.json yarn.lock ./

RUN yarn -s --frozen-lockfile

COPY tsconfig.json .
COPY www/ ./www

RUN yarn run -s build

RUN yarn -s compile --targets node18-alpine-x64

### End platform
FROM alpine:3.16

WORKDIR /flagger

ENV NODE_ENV production

COPY --from=node-builder /node-builder/out/flagger-serve-www /usr/local/bin/flagger-serve-www
COPY --from=rust-builder /rust-builder/target/release/flagger /usr/local/bin/flagger

CMD ["flagger", "--help"]
