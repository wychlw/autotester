use std::{fs, thread::sleep, time::Duration};

use tester::{
    devhost::sdwirec::{SdwirecChooser, SdwirecProd},
    dyn_cast_mut, dyn_into,
    exec::{
        cli_api::{CliTestApi, ExecBase, SudoCliTestApi},
        cli_exec::CliTester,
        cli_exec_sudo::SudoCliTester,
    },
    term::{
        asciicast::Asciicast,
        recorder::{Recorder, SimpleRecorder},
        serial::Serial,
        shell::Shell,
        tty::WrapperTty,
    },
};

#[test]
fn gen_record() {
    let sd = SdwirecProd::new(SdwirecChooser::Id(0));
    let ts = Shell::build(Some("/bin/bash")).unwrap();
    let rec = Asciicast::build(Box::new(ts));

    // 获取镜像 ...
    let mut exec = SudoCliTester::build(Box::new(rec));
    exec.script_run("cd ~/Work/plct/boards/d1/debian", 2)
        .unwrap();
    exec.assert_script_run("unzip RVBoards_D1_Debian_lxde_img_linux_v0.4.1.img.zip", 60)
        .unwrap();

    // 录屏从此处开始
    let rec = dyn_cast_mut!(exec.inner_mut(), Asciicast).unwrap();
    rec.begin().unwrap();
    sd.to_ts().unwrap();
    // 刷写镜像 ...
    exec.assert_script_sudo(
        "dd if=RVBoards_D1_Debian_lxde_img_linux_v0.4.1.img of=/dev/sda bs=4M status=progress",
        600,
    )
    .unwrap();

    sd.to_dut().unwrap();
    // 刷写完成 ... 让我们把镜头切到 DUT 上
    let dut = Serial::build("/dev/ttyUSB0", 115200).unwrap();
    let rec = dyn_cast_mut!(exec.inner_mut(), Asciicast).unwrap();
    let ts = rec.swap(Box::new(dut)).unwrap(); // 录屏切换到 DUT

    // 等它到登录后，我们需要文字 log 了
    exec.wait_serial("login:", 60).unwrap();

    let rec = dyn_into!(exec.exit(), Asciicast).unwrap();
    let mut logger = SimpleRecorder::build(rec);
    logger.begin().unwrap();
    let mut exec = CliTester::build(Box::new(logger));

    // 记录需要的信息，完毕
    exec.writeln("root").unwrap();
    exec.wait_serial("password", 5).unwrap();
    exec.writeln("rvboards").unwrap();
    sleep(Duration::from_secs(3));
    exec.writeln("cat /etc/os-release").unwrap();
    sleep(Duration::from_secs(5));
    exec.writeln("uname -a").unwrap();
    sleep(Duration::from_secs(5));
    exec.script_run("whoami", 2).unwrap();

    let mut logger = dyn_into!(exec.exit(), SimpleRecorder).unwrap();
    let dut_log = logger.end().unwrap();
    let mut rec = dyn_into!(logger.exit(), Asciicast).unwrap();
    // 结束了，保存录屏
    let rec_cast = rec.end().unwrap();
    // 关闭
    let dut = dyn_into!(rec.exit(), Serial).unwrap();
    let ts = dyn_into!(ts, Shell).unwrap();
    // dut.stop();
    ts.stop();
    // 保存文件
    fs::write("dut.log", dut_log).unwrap();
    fs::write("rec.cast", rec_cast).unwrap();
}
