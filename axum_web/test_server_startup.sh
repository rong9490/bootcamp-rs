#!/bin/bash
# 测试服务器启动和日志输出

echo "=== 启动服务器 ==="
cargo run --bin axum_web > /tmp/axum_output.log 2>&1 &
SERVER_PID=$!

# 等待服务器启动
sleep 2

echo ""
echo "=== 捕获的日志输出 ==="
cat /tmp/axum_output.log | head -30

echo ""
echo "=== 测试服务器是否响应 ==="
curl -s http://127.0.0.1:4000/health
echo ""

echo ""
echo "=== 停止服务器 ==="
kill $SERVER_PID 2>/dev/null
wait $SERVER_PID 2>/dev/null

echo ""
echo "=== 完整日志（最后20行）==="
tail -20 /tmp/axum_output.log
