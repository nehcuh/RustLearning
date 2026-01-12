#!/bin/bash

# KV Server 测试脚本

echo "=== KV Server Test Script ==="
echo ""

# 检查端口是否被占用
if lsof -Pi :9876 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo "错误：端口 9876 已被占用，请先停止运行中的服务器"
    exit 1
fi

# 编译项目
echo "1. 编译项目..."
cargo build --examples
echo "   ✓ 编译完成"
echo ""

# 启动服务器
echo "2. 启动服务器..."
cargo run --example server > /tmp/kv-server.log 2>&1 &
SERVER_PID=$!
echo "   服务器 PID: $SERVER_PID"
echo "   日志文件: /tmp/kv-server.log"
echo ""

# 等待服务器启动
echo "3. 等待服务器就绪..."
for i in {1..30}; do
    if nc -z localhost 9876 2>/dev/null; then
        echo "   ✓ 服务器已就绪"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "   ✗ 服务器启动超时"
        cat /tmp/kv-server.log
        kill $SERVER_PID 2>/dev/null || true
        exit 1
    fi
    sleep 0.1
done
echo ""

# 运行客户端测试
echo "4. 运行客户端测试..."
cargo run --example client
CLIENT_EXIT=$?
echo ""

# 清理
echo "5. 清理..."
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true
echo "   ✓ 服务器已停止"
echo ""

if [ $CLIENT_EXIT -eq 0 ]; then
    echo "=== 所有测试通过！==="
else
    echo "=== 测试失败 ==="
    exit $CLIENT_EXIT
fi

# 显示服务器日志
if [ -f /tmp/kv-server.log ]; then
    echo ""
    echo "服务器日志："
    echo "---"
    cat /tmp/kv-server.log
    echo "---"
fi
