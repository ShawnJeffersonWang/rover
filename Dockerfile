# ---- Builder Stage ----
# 使用官方的 Rust 镜像作为构建环境
FROM rust:1-slim AS builder

# 创建一个非 root 用户
RUN useradd --user-group --create-home --shell /bin/bash app

WORKDIR /app

# 复制依赖清单文件
COPY Cargo.toml Cargo.lock ./

# 创建一个虚拟的 lib.rs 来缓存依赖项
# 只有当 Cargo.toml/lock 改变时，这一层才会重新构建
RUN mkdir src && echo "fn main() {}" > src/lib.rs
RUN cargo build --release --locked

# 复制完整的源代码
COPY . .

# 删除虚拟的 lib.rs 并构建真正的应用
RUN rm -f target/release/deps/agricultural_vision_rust*
RUN cargo build --release --locked

# ---- Runner Stage ----
# 使用一个非常小的基础镜像来运行程序
FROM debian:slim

# 再次创建非 root 用户
RUN useradd --user-group --create-home --shell /bin/bash app
USER app
WORKDIR /app

# 从 builder 阶段复制编译好的二进制文件和配置文件
COPY --from=builder /app/target/release/agricultural_vision_rust .
COPY --chown=app:app config ./config

# 暴露应用程序的端口
EXPOSE 8000

# 容器启动时运行的命令
CMD ["./agricultural_vision_rust"]