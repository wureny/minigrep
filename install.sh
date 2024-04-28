#!/bin/bash

# 获取 minigrep 可执行文件的绝对路径
MINIGREP_PATH="$(cd "$(dirname "$0")" && pwd)/target/release/minigrep"

# 复制可执行文件到 /usr/local/bin
sudo cp "$MINIGREP_PATH" /usr/local/bin/

echo "minigrep has been installed to /usr/local/bin/"
