- [ ]  添加 rustdoc
    - [ ]  cli
        - [x]  mod
        - [ ]  asciicast
        - [x]  deansi
        - [ ]  recorder
        - [ ]  serial
        - [ ]  shell
        - [ ]  ssh
        - [x]  tee
        - [x]  tty

- [ ] **可维护性：想办法把那个 PyTty 缩小掉……**

- [ ] CLI
    - [ ] 获取内层某个层级的 ref/mut 然后更改？不知道会不会破坏借用检查器不过…会的话可能得从 Box 换成 Rc 了

- [ ] 更多的连接方式
    - [x] 更完善的 SSH
    - [ ] 通过 tunnel 连接

- [ ] 外设支持
    - [ ] 外设抽象 : mod devhost
    - [ ] 外设编写
        - [x] SdWireC

- [ ] 设备抽象 : mod device

- [x] 更加的多态支持 : where T: Tty -> Box<dyn Tty>
    - [ ] Trait Cast

- [ ] 导出的 API
    - [x] 实现 cli-like 面向外界的哪一个巨型 wrapper
        - [?] 从 dyn Tty 中区分出这个巨型 wrapper，并分开实现（可以在每次开头前都试一试？）
            - [ ] 
    - [x] 执行器
    - [ ] Python API 多个层级之间不太能互通…（inner_ref 和 inner_mut 不能直接用）需要想个办法处理下

- [ ] 与下一步测试软件的进一步集成
    - [ ] GUI 部分框架
    - [ ] UI 部分
        - [ ] UI 设计
            - 目前考虑多窗口，然后在上面放上一个编辑器和执行器什么的…这样可以实时编辑、打 needle 啥的？
            - 然后由于 Wrapper 的层级越来越高…要不要写个啥展示下当前层级的样子呢（思考）

- [ ] 实际应用
