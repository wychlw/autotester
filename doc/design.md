# 设计思路

## CLI

### Tty-like 

对于 Cli 侧的设备，我们可以将其视为一群 Tty 的集合。

一个 Tty 被认为是：具有输入 bytearray、输出 bytearray 的 class。因此，Tty 需要实现以下 trait：
```rust
pub trait Tty {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>>;
}
```

和管道一样，Tty 的输入当然可以是另一个 Tty 啦。比如各类 recorder 这些。当然，由于没有“管道”这种直接的东西可以用，实际方案是将输入设备插入到处理的 Tty 之中。因此，我们需要一个方式在释放的时候取回里面的东西。这种操作叫做：
```rust
pub trait WrapperTty: Tty {
    fn exit(self) -> DynTty;
}
```

有些东西会把它们的数据处理的面目全非……比如各种提供 os-autoinst API 的执行器；有些时候我们又需要处理中间的某个开关。对此，需要一种方式提供被它“包裹”的东西的引用：
```rust
pub trait InnerTty: WrapperTty {
    fn inner_ref(&self) -> &DynTty;
    fn inner_mut(&mut self) -> &mut DynTty;
}
```
