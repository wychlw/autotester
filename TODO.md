- [ ] 更多的连接方式
    - [x] 更完善的 SSH
    - [ ] 通过 tunnel 连接

- [ ] 外设支持
    - [ ] 外设抽象 : mod devhost
    - [ ] 外设编写
        - [x] SdWireC

- [ ] 设备抽象 : mod device

- [x] 更加的多态支持 : where T: Tty -> Box<dyn Tty>

- [ ] 导出的 API
    - [x] 实现 cli-like 面向外界的哪一个巨型 wrapper
        - [?] 从 dyn Tty 中区分出这个巨型 wrapper，并分开实现（可以在每次开头前都试一试？）
    - 执行器

- [ ] 与下一步测试软件的进一步集成