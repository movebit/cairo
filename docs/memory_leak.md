# cairo-language-server memory leak
先要编译目标: cargo build --bin cairo-language-server

## 1.泄漏外层位置：
```rust
// crates/cairo-lang-language-server/src/lib.rs: refresh_diagnostics()
let new_file_diagnostics = FileDiagnostics {
    parser: db.file_syntax_diagnostics(file_id),
    semantic: db.file_semantic_diagnostics(file_id).unwrap_or_default(),
    lowering: db.file_lowering_diagnostics(file_id).unwrap_or_default(),
};
```

## 2.打印内存耗用代码：

```rust
// 依赖这个子模块 crates/cairo-lang-language-server/src/sys
sys::record_global_memory_usage();  // 1.先记录
eprintln!(">> refresh_diagnostics current_mem = {}Kb, CurTime = {}", sys::get_global_memory_usage(), printCurTimeStr());  // 2.再读取
```

## 3.打开一份 cairo 代码, 触发内存泄漏
参考: https://github.com/starkware-libs/cairo/issues/4062
