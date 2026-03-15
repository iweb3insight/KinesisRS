#!/bin/bash

# KinesisRS 自动安装脚本
# 用法: curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash

set -e

# 配置
VERSION="v0.6.5"
INSTALL_DIR="${HOME}/.kinesis"
BINARY_NAME="kinesis-rs"

# 检测平台
detect_platform() {
    local platform=$(uname -s)
    local arch=$(uname -m)
    
    case "$platform" in
        Darwin)
            if [ "$arch" = "arm64" ]; then
                echo "darwin-arm64"
            else
                echo "darwin-amd64"
            fi
            ;;
        Linux)
            echo "linux-amd64"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            echo "windows-amd64"
            ;;
        *)
            echo "unsupported"
            ;;
    esac
}

# 获取下载链接
get_download_url() {
    local platform=$1
    local base_url="https://github.com/iweb3insight/KinesisRS/releases/download/${VERSION}"
    
    case "$platform" in
        darwin-arm64)
            echo "${base_url}/kinesis-rs-${VERSION}-macos-arm64.tar.gz"
            ;;
        darwin-amd64)
            echo "${base_url}/kinesis-rs-${VERSION}-macos-amd64.tar.gz"
            ;;
        linux-amd64)
            echo "${base_url}/kinesis-rs-${VERSION}-linux-amd64.tar.gz"
            ;;
        windows-amd64)
            echo "${base_url}/kinesis-rs-${VERSION}-windows-amd64.zip"
            ;;
        *)
            echo ""
            ;;
    esac
}

# 主函数
main() {
    local platform=$(detect_platform)
    
    if [ "$platform" = "unsupported" ]; then
        echo "Error: 不支持的平台: $(uname -s)"
        exit 1
    fi
    
    echo "检测到平台: $platform"
    echo "版本: $VERSION"
    
    # 创建安装目录
    mkdir -p "$INSTALL_DIR"
    
    # 获取下载链接
    local url=$(get_download_url "$platform")
    
    if [ -z "$url" ]; then
        echo "Error: 无法获取下载链接"
        exit 1
    fi
    
    echo "下载链接: $url"
    
    # 下载
    local temp_file="/tmp/kinesis-install.$$"
    echo "下载中..."
    
    if command -v curl > /dev/null 2>&1; then
        curl -sL "$url" -o "$temp_file"
    elif command -v wget > /dev/null 2>&1; then
        wget -q "$url" -O "$temp_file"
    else
        echo "Error: 需要 curl 或 wget"
        exit 1
    fi
    
    # 解压
    echo "解压中..."
    case "$platform" in
        darwin-*|linux-*)
            tar -xzf "$temp_file" -C "$INSTALL_DIR"
            ;;
        windows-*)
            unzip -q "$temp_file" -d "$INSTALL_DIR"
            ;;
    esac
    
    # 清理
    rm -f "$temp_file"
    
    # 设置权限
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    
    # 验证
    if "${INSTALL_DIR}/${BINARY_NAME}" --version > /dev/null 2>&1; then
        echo "安装成功!"
        echo ""
        echo "使用方法:"
        echo "  export SOL_RPC_URL=\"https://api.mainnet-beta.solana.com\""
        echo "  ${INSTALL_DIR}/${BINARY_NAME} --json balance --chain solana"
        echo ""
        echo "添加到 ~/.bashrc 或 ~/.zshrc:"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    else
        echo "Warning: 验证失败，但文件可能已安装"
    fi
}

main "$@"
