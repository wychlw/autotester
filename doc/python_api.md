# Python API

## 简介

目前导出了以下几个类作为 API：

- PyTty：一个非常大的类，传入一个 toml 字符串，根据配置动态生成需要的类。对于外接的 API，其同样也是会在内部动态转换到相应的类来执行。
- PyShell：初步尝试拆分 PyTty 到各个子类（但 exit 返回 inner wrapper 导致不太好完全实现…正在寻找解决方案）
- PyExec：执行类，导出 os-autoinst API

## PyTty

该类实际包含很多不同的类和功能，使用时请注意实际用的什么功能

PyTty 方式如下：

### __init__

```python
__init__(conf: str, be_wrapped: PyTty = None)
```

- conf：toml 配置字符串，详解于下
- be_wrapped：传入的套娃参数，在 warp 被配置时生效。

#### toml config

- wrap: bool? 是否包裹另一个 PyTty
- shell: object？创建一个可执行的本地 Shell
    - shell: str? 使用的 shell，默认为 `/bin/sh`
- simple_recorder: bool? 创建一个直接记录 raw 的 recorder *wrap*
- asciicast: bool? 创建一个记录 asciicast 格式的 recorder *wrap*
- exec: object? 创建一个 API 执行器 *wrap*
    - sudo: bool? 是否支持 sudo，默认为 true 

### 其余 API

其余 API 直接导出相应的 rust 接口。可以见各个 trait，包括：
- Tty
- WrapperTty
- Recorder

*CliexecApi 导出在 PyExec 中*

## PyShell

### __init__

```python
__init__(shell: str = None)
```

- str 使用的 shell，默认 `/bin/sh`

### 其余 API

见 rust 中的 Tty trait

## PyExec

### __init__

```python
__init__(be_wrapped: PtTty, sudo: bool = None)
```

- be_wrapped：执行器内部包裹的 tty

### 其余 API

实现以下 trait 导出：
```rust
pub trait CliTestApi: ExecBase {
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
    fn assert_script_run(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn Error>>;
    fn writeln(&mut self, script: &str) -> Result<(), Box<dyn Error>>;
    fn wait_serial(&mut self, expected: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
}

pub trait SudoCliTestApi: CliTestApi {
    fn script_sudo(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
    fn assert_script_sudo(&mut self, script: &str, timeout: u32) -> Result<(), Box<dyn Error>>;
}
```

