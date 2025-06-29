# Rustのビルド環境を含んだイメージをベースにする
FROM rust:1-slim

# 作業ディレクトリを設定
WORKDIR /usr/src/app

# プロジェクトのファイルをすべてコンテナにコピー
COPY . .

# イメージビルド時に一度だけリリースビルドを実行
# これで実行ファイルが target/release/ に作成される
RUN cargo build --release

# 公開するポートを指定
EXPOSE 3000

# コンテナ起動時は、ビルド済みの実行可能ファイルを実行するだけ
# <your_app_name> はご自身のプロジェクト名に書き換えてください
CMD ["./target/release/MinWeb2025-blogging-platform-backend"]
