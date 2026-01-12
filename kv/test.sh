#!/bin/bash

# 测试脚本
# 用于启动服务器并运行客户端测试

set -e

echo "=== KV Server Test Script ==="
echo ""

# 检查是否已经有一个服务器在运行
if lsof -Pi :9876 -sTCP:LISTEN -t >/dev/null 2>&1 ; then
    echo "Port 9876 is already in use. Please stop any running server first."
    exit 1
fi

# 编译项目
echo "1. Building project..."
cargo build --examples
echo "   ✓ Build completed"
echo ""

# 启动服务器
echo "2. Starting server..."
cargo run --example server > /tmp/kv-server.log 2>&1 &
SERVER_PID=$!
echo "   Server started with PID: $SERVER_PID"
echo "   Server log: /tmp/kv-server.log"
echo ""

# 等待服务器启动
echo "3. Waiting for server to be ready..."
MAX_RETRIES=30
RETRY_COUNT=0
while ! nc -z localhost 9876 2>/dev/null; do
    if [ $RETRY_COUNT -ge $MAX_RETRIES ]; then
        echo "   ✗ Server failed to start within expected time"
        cat /tmp/kv-server.log
        kill $SERVER_PID 2>/dev/null || true
        exit 1
    fi
    sleep 0.1
    RETRY_COUNT=$((RETRY_COUNT + 1))
done
echo "   ✓ Server is ready"
echo ""

# 运行客户端测试
echo "4. Running client tests..."
cargo run --example client
echo "   ✓ Client tests completed"
echo ""

# 清理
echo "5. Cleaning up..."
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true
echo "   ✓ Server stopped"
echo ""

echo "=== All tests completed successfully! ==="

# 显示服务器日志（可选）
if [ -f /tmp/kv-server.log ]; then
    echo ""
    echo "Server log:"
    echo "---"
    cat /tmp/kv-server.log
    echo "---"
fi
