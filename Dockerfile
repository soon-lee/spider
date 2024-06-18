# 使用官方的Rust Docker镜像作为基础镜像，这里使用latest标签，也可以指定具体版本，如 rust:1.70
FROM rust:latest as builder

# 设置工作目录
WORKDIR /usr/src/app

# 将Cargo.lock和Cargo.toml复制到容器中，以便Cargo能解析依赖
COPY Cargo.lock Cargo.toml ./

# 使用Cargo fetch 下载依赖，这一步可以在多阶段构建中复用
RUN cargo fetch 

# 将整个项目源代码复制到容器中
COPY . .

# 使用Cargo build --release 构建优化后的可执行文件
RUN cargo build --release

# 使用另一个基础镜像，比如Alpine Linux，用于运行构建好的应用
FROM alpine:latest

# 安装必要的动态链接库，如果有的话。对于Axum应用，通常需要安装libc6
RUN apk add --no-cache libc6-compat

# 从构建阶段拷贝编译好的可执行文件到新的镜像中
COPY --from=builder /usr/src/app/target/release/my_axum_app /usr/local/bin/

# 暴露应用需要监听的端口
EXPOSE 3000

# 定义启动容器时运行的命令
CMD ["./my_axum_app"]

# 可选：给镜像打标签
LABEL maintainer="your-email@example.com"
