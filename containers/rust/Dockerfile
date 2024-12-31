# ==================== Builderステージ ====================
FROM rust:1.83-slim AS builder

# 必要なビルドツールとライブラリをインストール
RUN apt-get update \
    && apt-get install -y \
        build-essential \
        libssl-dev \
        pkg-config

WORKDIR /usr/src/app

# アプリケーションをビルド
COPY . . 
RUN cargo build --release

# ==================== 最終ステージ ====================
FROM debian:bookworm-slim

WORKDIR /usr/src/app

# 必要なツールとライブラリをインストール
RUN apt-get update \
    && apt-get install -y \
        build-essential \
        libssl3 \
        libssl-dev \
        pkg-config \
        curl

# RustとCargoのセットアップ
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y \
    && export PATH="/root/.cargo/bin:${PATH}" \
    && rustup default stable

# sea-orm-cliをインストール
RUN /root/.cargo/bin/cargo install sea-orm-cli

# PATHを更新
ENV PATH="/root/.cargo/bin:${PATH}"

# マイグレーション関連ファイルをコピー
COPY migration ./migration

# バイナリファイルをコピー
COPY --from=builder /usr/src/app/target/release/backend .

# インストール確認
RUN sea-orm-cli --version

# Webサーバーをデフォルトのエントリーポイントにする
CMD ["/usr/src/app/backend"]
