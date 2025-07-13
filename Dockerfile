# ---- Builder Stage ----
# 使用一个明确版本的官方 Rust 镜像作为构建环境，以确保构建的可复现性
# 推荐使用基于当前 Debian 稳定版 (bookworm) 的镜像
FROM rust:1.88-slim-bookworm AS builder

# 创建一个非 root 用户，增强安全性
RUN useradd --user-group --create-home --shell /bin/bash app

WORKDIR /app

# 复制依赖清单文件
COPY Cargo.toml Cargo.lock ./

# ====================================================================
# Bug 修复: 安装 C/C++ 编译器和必要的构建依赖
# - build-essential: 提供了 g++, gcc, make 等编译 C/C++ 代码所需的基础工具链，解决了 "c++: command not found" 的问题。
# - pkg-config: 许多 C/C++ 库使用它来帮助编译器找到依赖项。
# - libssl-dev: rdkafka 经常需要链接 OpenSSL，即使当前配置禁用了 SSL，预先安装可以避免未来开启相关 feature 时再次构建失败。
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       build-essential \
       pkg-config \
       libssl-dev \
    && rm -rf /var/lib/apt/lists/*
# ====================================================================

# 创建一个虚拟的 lib.rs 来缓存依赖项
# 只有当 Cargo.toml/lock 或构建工具改变时，这一层才会重新构建
RUN mkdir src && echo "fn main() {}" > src/lib.rs
# --locked 确保使用 Cargo.lock 中锁定的版本
RUN cargo build --release --locked

# 复制完整的源代码
# 注意：将 .dockerignore 文件配置好，避免不必要的文件被复制进来
COPY . .

# 删除虚拟的 lib.rs 并构建真正的应用
# 使用 cargo clean 可以更干净地清理，但直接删除目标文件更快
RUN rm -f target/release/deps/agricultural_vision_rust*
RUN cargo build --release --locked

# ---- Runner Stage ----
# 使用一个带明确版本的、非常小的基础镜像来运行程序
# bookworm 是 Debian 12 的代号，是当前的稳定版
FROM debian:bookworm-slim

# 安装必要的运行时依赖
# - ca-certificates: 用于进行 HTTPS/TLS 连接
# - libssl3: 如果你的程序链接了 OpenSSL，运行时也需要它。对于 bookworm，应该是 libssl3。
# --no-install-recommends: 避免安装不必要的推荐包，保持镜像小巧
# rm -rf /var/lib/apt/lists/*: 清理 apt 缓存，减小镜像体积
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 再次创建非 root 用户
RUN useradd --user-group --create-home --shell /bin/bash app
USER app
WORKDIR /app

# 从 builder 阶段复制编译好的二进制文件和配置文件
# --chown=app:app 确保复制过来的文件属于我们创建的非 root 用户
COPY --from=builder --chown=app:app /app/target/release/agricultural_vision_rust .
COPY --chown=app:app config ./config

# 暴露应用程序的端口
EXPOSE 8000

# 容器启动时运行的命令
CMD ["./agricultural_vision_rust"]