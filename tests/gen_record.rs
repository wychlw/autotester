// use std::fs;

// use tester::{devhost::sdwirec::{SdwirecChooser, SdwirecProd}, term::{asciicast::Asciicast, recorder::{Recorder, SimpleRecorder}, serial::Serial, shell::Shell, tty::WrapperTty}};

// #[test]
// fn gen_record() {

//     let sd = SdwirecProd::new(SdwirecChooser::Id(0));
//     let ts = Shell::build(Some("/bin/bash")).unwrap();
//     // 获取镜像 ...
//     let mut rec = Asciicast::build(ts); // 录屏从此处开始
//     rec.begin().unwrap();
//     sd.to_ts().unwrap();
//     // 刷写镜像 ...
//     sd.to_dut().unwrap();
//     // 刷写完成 ... 让我们把镜头切到 DUT 上
//     let dut = Serial::build("/dev/ttyUSB0", 115200).unwrap();
//     let ts = rec.swap(dut).unwrap(); // 录屏切换到 DUT
//     // 等它到登录后，我们需要文字 log 了
//     let mut logger = SimpleRecorder::build(rec);
//     logger.begin().unwrap();
//     // 记录需要的信息，完毕
//     let dut_log = logger.end().unwrap();
//     let mut rec = logger.exit();
//     // 结束了，保存录屏
//     let rec_cast = rec.end().unwrap();
//     // 关闭
//     let dut = rec.exit();
//     dut.stop();
//     ts.stop();
//     // 保存文件
//     fs::write("dut.log", dut_log).unwrap();
//     fs::write("rec.cast", rec_cast).unwrap();


// }