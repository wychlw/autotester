# autotester

Python 部分文档见 [doc](./doc)
Rust 部分代码包含 rustdoc

## 文件夹释义

- doc：文档
- python：Python 侧代码
- src：Rust 侧代码
- tests：应用测试及示例
- apps：Python 实际测试脚本

## 目前完成状况

Rust CLI: 基本稳定，已完成的 API 不会有大的改动，可能会有新增中间处理层
Rust CLI Exec：稳定，不太可能有更多变动（其它兼容/功能请在 FFI/外部实现）

Rust GUI：不稳定，API 不太可能有太大变动不过
Rust GUI Exec：不稳定，API 变动可能（根据新功能实验更改）

Rust Python Api：我保证它能用…… **计划改动，我尽量做到不破坏现有 API**
UI：~~一团空气~~
