#!/bin/bash

# Kinesis 环境检测脚本
# 用法: ./detect.sh

set -e

# 颜色
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检测结果
ERRORS=()
WARNINGS=()
SUCCESS=()

echo "========================================"
echo "Kinesis 环境检测"
echo "========================================"
echo ""

# 1. 检测二进制
echo "1. 检测二进制文件..."
if [ -x "$HOME/.kinesis/kinesis-rs" ]; then
    VERSION=$($HOME/.kinesis/kinesis-rs --version 2>/dev/null || echo "unknown")
    SUCCESS+=("二进制: $HOME/.kinesis/kinesis-rs (${VERSION})")
    echo -e "${GREEN}✓${NC} 二进制已安装: ${VERSION}"
else
    ERRORS+=("二进制未安装")
    echo -e "${RED}✗${NC} 二进制未安装"
    echo "  运行: curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash"
fi
echo ""

# 2. 检测平台
echo "2. 检测平台..."
PLATFORM=$(uname -s)
ARCH=$(uname -m)
echo -e "${GREEN}✓${NC} 平台: ${PLATFORM} ${ARCH}"
echo ""

# 3. 检测环境变量
echo "3. 检测环境变量..."

# SOL_RPC_URL
if [ -n "$SOL_RPC_URL" ]; then
    SUCCESS+=("SOL_RPC_URL: $SOL_RPC_URL")
    echo -e "${GREEN}✓${NC} SOL_RPC_URL: $SOL_RPC_URL"
else
    WARNINGS+=("SOL_RPC_URL 未设置")
    echo -e "${YELLOW}!${NC} SOL_RPC_URL 未设置 (将使用默认值)"
fi

# BSC_RPC_URL
if [ -n "$BSC_RPC_URL" ]; then
    SUCCESS+=("BSC_RPC_URL: $BSC_RPC_URL")
    echo -e "${GREEN}✓${NC} BSC_RPC_URL: $BSC_RPC_URL"
else
    WARNINGS+=("BSC_RPC_URL 未设置")
    echo -e "${YELLOW}!${NC} BSC_RPC_URL 未设置 (将使用默认值)"
fi
echo ""

# 4. 检测网络连接
echo "4. 检测网络连接..."

# Solana RPC
if [ -n "$SOL_RPC_URL" ]; then
    if curl -s -X POST "$SOL_RPC_URL" -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' > /dev/null 2>&1; then
        SUCCESS+=("Solana RPC: 可用")
        echo -e "${GREEN}✓${NC} Solana RPC: 可用"
    else
        ERRORS+=("Solana RPC: 不可用")
        echo -e "${RED}✗${NC} Solana RPC: 不可用"
    fi
else
    WARNINGS+=("跳过 Solana RPC 检测 (未设置)")
    echo -e "${YELLOW}!${NC} 跳过 Solana RPC 检测 (未设置)"
fi

# BSC RPC
if [ -n "$BSC_RPC_URL" ]; then
    if curl -s -X POST "$BSC_RPC_URL" -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber"}' > /dev/null 2>&1; then
        SUCCESS+=("BSC RPC: 可用")
        echo -e "${GREEN}✓${NC} BSC RPC: 可用"
    else
        ERRORS+=("BSC RPC: 不可用")
        echo -e "${RED}✗${NC} BSC RPC: 不可用"
    fi
else
    WARNINGS+=("跳过 BSC RPC 检测 (未设置)")
    echo -e "${YELLOW}!${NC} 跳过 BSC RPC 检测 (未设置)"
fi
echo ""

# 5. 检测钱包
echo "5. 检测钱包..."
if [ -x "$HOME/.kinesis/kinesis-rs" ]; then
    if [ -n "$SOL_RPC_URL" ]; then
        WALLET=$($HOME/.kinesis/kinesis-rs wallet 2>/dev/null | grep -oP '88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv' || echo "")
        if [ -n "$WALLET" ]; then
            SUCCESS+=("钱包: $WALLET")
            echo -e "${GREEN}✓${NC} 钱包: $WALLET"
        else
            echo -e "${YELLOW}!${NC} 钱包: $WALLET"
        fi
    fi
fi
echo ""

# 总结
echo "========================================"
echo "检测结果"
echo "========================================"

if [ ${#ERRORS[@]} -gt 0 ]; then
    echo -e "${RED}错误:${NC}"
    for err in "${ERRORS[@]}"; do
        echo "  - $err"
    done
    echo ""
fi

if [ ${#WARNINGS[@]} -gt 0 ]; then
    echo -e "${YELLOW}警告:${NC}"
    for warn in "${WARNINGS[@]}"; do
        echo "  - $warn"
    done
    echo ""
fi

if [ ${#SUCCESS[@]} -gt 0 ]; then
    echo -e "${GREEN}成功:${NC}"
    for succ in "${SUCCESS[@]}"; do
        echo "  + $succ"
    done
    echo ""
fi

# 最终状态
if [ ${#ERRORS[@]} -gt 0 ]; then
    echo -e "${RED}状态: 失败${NC}"
    exit 1
elif [ ${#WARNINGS[@]} -gt 0 ]; then
    echo -e "${YELLOW}状态: 警告${NC}"
    exit 0
else
    echo -e "${GREEN}状态: 通过${NC}"
    exit 0
fi
