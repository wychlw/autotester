use std::{
    error::Error,
    io::Write,
    sync::Arc,
    thread::{sleep, spawn, JoinHandle},
    time::{Duration, Instant},
};

use eframe::egui::mutex::Mutex;
use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream, ToNsName};
use pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyRefMut, PyResult};
use serde::{Deserialize, Serialize};

use crate::{
    cli::tty::{DynTty, Tty, WrapperTty},
    consts::DURATION,
    err,
    exec::cli_api::{CliTestApi, SudoCliTestApi},
    impl_any, info, log,
    pythonapi::shell_like::{handle_wrap, py_tty_inner, PyTty, PyTtyInner},
    ui::{ipc::parse_sock_id, util::get_sub_virt},
    util::{anybase::heap_raw, util::rand_string},
};

use super::ipc::{get_sock_name, sub_send_msg, sub_send_msg_wait_msg, WindowIpcMessage};

pub struct UiCliTester {
    inner: Arc<Mutex<Option<DynTty>>>,
    stream: Arc<Mutex<Option<String>>>,
    buf: Arc<Mutex<Vec<u8>>>,
    handle: Option<JoinHandle<()>>,
    exit: Arc<Mutex<bool>>,
}

#[derive(Serialize, Deserialize)]
pub enum UiCliIpcMsg {
    BUILD(String),    // build ipc stream with given socket
    REBUILD(String),  // rebuild ipc stream with given socket, so why pipe will broken?
    CONSOLE(Vec<u8>), // transfer a string
    EXIT,             // end ipc
}

fn send_once(sock_name: &str, msg: &UiCliIpcMsg) -> Result<(), Box<dyn Error>> {
    log!("Send to term: {}", serde_json::to_string(msg)?);
    let sock_name = sock_name.to_ns_name::<GenericNamespaced>()?;
    let mut conn = Stream::connect(sock_name)?;
    serde_json::to_writer(&conn, msg)?;
    conn.write(b"\n")?;
    Ok(())
}

pub fn rebuild_ipc(ori_name: String) -> Result<String, Box<dyn Error>> {
    let window_id = parse_sock_id(&ori_name);
    let window_id = match window_id {
        Some(x) => x,
        None => return Err("Invalid window id".into()),
    };
    let sock = get_sock_name(get_sub_virt() + &rand_string(5) + "cliui", Some(window_id));
    let handshake = sub_send_msg_wait_msg(WindowIpcMessage {
        window_id,
        message: serde_json::to_string(&UiCliIpcMsg::REBUILD(sock))?,
    })?;
    let handshake: UiCliIpcMsg = serde_json::from_str(&handshake.message)?;
    let sock = match handshake {
        UiCliIpcMsg::REBUILD(sock) => sock,
        _ => return Err("Invalid handshake".into()),
    };
    log!("IPC rebuilt: {:?}", sock);
    Ok(sock)
}

impl UiCliTester {
    pub fn begin_ipc(&mut self, window_id: u64, sock: String) -> Result<(), Box<dyn Error>> {
        let self_stream = self.stream.clone();
        let mut self_stream = self_stream.lock();
        if self_stream.is_some() {
            return Err("Already has IPC".into());
        }
        let handshake = sub_send_msg_wait_msg(WindowIpcMessage {
            window_id,
            message: serde_json::to_string(&UiCliIpcMsg::BUILD(sock))?,
        })?;
        let handshake: UiCliIpcMsg = serde_json::from_str(&handshake.message)?;
        let sock = match handshake {
            UiCliIpcMsg::BUILD(sock) => sock,
            _ => return Err("Invalid handshake".into()),
        };
        info!("IPC connected: {:?}", sock);
        *self_stream = Some(sock);
        Ok(())
    }
    pub fn end_ipc(&mut self) {
        let self_stream = self.stream.clone();
        let mut self_stream = self_stream.lock();
        let stream = self_stream.take();
        let stream = match stream {
            Some(x) => x,
            None => return,
        };
        send_once(&stream, &UiCliIpcMsg::EXIT).unwrap();
    }
    pub fn build(inner: DynTty, term_id: u64) -> Result<Self, Box<dyn Error>> {
        let mut res = Self {
            inner: Arc::new(Mutex::new(Some(inner))),
            stream: Arc::new(Mutex::new(None)),
            buf: Arc::new(Mutex::new(Vec::new())),
            handle: None,
            exit: Arc::new(Mutex::new(false)),
        };

        let sock_name = get_sub_virt() + &rand_string(5) + "cliui";
        let sock_name = get_sock_name(sock_name, Some(term_id));
        res.begin_ipc(term_id, sock_name)?;

        let inner = res.inner.clone();
        let stream = res.stream.clone();
        let buf = res.buf.clone();
        let exit = res.exit.clone();
        let handle = spawn(move || loop {
            sleep(Duration::from_millis(DURATION));
            {
                let exit = exit.lock();
                if *exit {
                    break;
                }
            }
            let data;
            {
                let mut inner = inner.lock();
                if inner.is_none() {
                    continue;
                }
                let inner = inner.as_mut().unwrap();
                let d = inner.read();
                if let Err(e) = d {
                    err!("read error: {}", e);
                    break;
                }
                data = d.unwrap();
                let mut buf = buf.lock();
                buf.extend(data.clone());
            }
            {
                let mut _stream = stream.lock();
                if _stream.is_none() {
                    continue;
                }
                if data.is_empty() {
                    continue;
                }
                // let mut retry = 0;
                sub_send_msg(WindowIpcMessage {
                    window_id: term_id,
                    message: serde_json::to_string(&UiCliIpcMsg::CONSOLE(data.clone())).unwrap(),
                });
                // loop {
                //     sleep(Duration::from_millis(DURATION));
                //     let stream = _stream.as_mut().unwrap();
                //     let e = match send_once(&stream, &UiCliIpcMsg::CONSOLE(data.clone())) {
                //         Ok(_) => break,
                //         Err(e) => e,
                //     };
                //     err!("IPC Send error: {}", e);
                //     if retry > 3 {
                //         break;
                //     }
                //     warn!("IPC broken, try to rebuild. Times: {}", retry);
                //     let ori_name = _stream.take();
                //     let ori_name = match ori_name {
                //         Some(x) => x,
                //         None => {
                //             err!("No IPC");
                //             break;
                //         }
                //     };
                //     *_stream = Some(rebuild_ipc(ori_name).unwrap());
                //     retry += 1;
                //     continue;
                // }
            }
        });

        res.handle = Some(handle);

        Ok(res)
    }
}

impl UiCliTester {
    fn run_command(&mut self, command: &String) -> Result<(), Box<dyn Error>> {
        info!("Write to shell: {}", command);
        sleep(Duration::from_millis(DURATION));
        let inner = self.inner.clone();
        let mut inner = inner.lock();
        let inner = inner.as_mut().unwrap();
        inner.write(command.as_bytes())
    }
    fn __exit(&mut self) {
        self.end_ipc();
        let mut exit = self.exit.lock();
        *exit = true;
    }
}
// impl Drop for UiCliTester {
//     fn drop(&mut self) {
//         self.__exit();
//     }
// }
impl_any!(UiCliTester);
impl Tty for UiCliTester {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = self.buf.lock();
        let res = buf.clone();
        buf.clear();
        Ok(res)
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = self.buf.lock();
        let res = buf.clone();
        buf.clear();
        Ok(res)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let inner = self.inner.clone();
        let mut inner = inner.lock();
        let inner = inner.as_mut().unwrap();
        inner.write(data)
    }
}
impl WrapperTty for UiCliTester {
    fn exit(mut self) -> DynTty {
        self.__exit();
        self.inner.lock().take().unwrap()
    }

    fn inner_ref(&self) -> &DynTty {
        // &self.inner
        panic!("You should not call this method");
    }

    fn inner_mut(&mut self) -> &mut DynTty {
        // &mut self.inner
        panic!("You should not call this method");
    }
}

impl UiCliTester {
    fn filter_assert_echo(&self, expected: &str, buf: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        let expected = "echo ".to_owned() + expected;
        let expected = expected.as_bytes();
        for (pos, window) in buf.windows(expected.len()).enumerate() {
            if window == expected {
                let i = pos + expected.len();
                buf.drain(0..=i);
                break;
            }
        }
        Ok(())
    }

    fn kmp_next(&self, target: &Vec<u8>) -> Vec<usize> {
        let mut next = vec![0usize; target.len()];
        let mut i = 1;
        let mut j = 0;
        while i < target.len() - 1 {
            if target[i] == target[j] {
                next[i] = j + 1;
                i += 1;
                j += 1;
            } else {
                if j == 0 {
                    next[i] = 0;
                    i += 1;
                } else {
                    j = next[j - 1] as usize;
                }
            }
        }
        next
    }

    fn kmp_search(&self, content: &Vec<u8>, target: &Vec<u8>) -> Option<usize> {
        let next = self.kmp_next(target);
        let mut i = 0;
        let mut j = 0;
        let mut res = None;
        while i < content.len() && j < target.len() {
            if content[i] == target[j] {
                if res.is_none() {
                    res = Some(i);
                }
                i += 1;
                j += 1;
                if j >= target.len() {
                    break;
                }
            } else {
                if j == 0 {
                    i += 1;
                } else {
                    j = next[j - 1];
                }
                res = None;
            }
        }
        res
    }

    fn do_wait_serial(
        &mut self,
        expected: &str,
        timeout: u32,
        filter_echo_back: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let begin = Instant::now();
        info!("Waiting for string {{{}}}", expected);
        loop {
            sleep(Duration::from_millis(DURATION));
            let buf = self.buf.lock();
            let mut buf = buf.clone();
            if let Some(filter) = filter_echo_back {
                self.filter_assert_echo(filter, &mut buf)?;
            }
            // The reason we compare raw u8 is... What if the data is corrupted?
            let target = expected.as_bytes();
            if let Some(pos) = self.kmp_search(&buf, &target.to_vec()) {
                info!("Matched string {{{}}}", expected);
                let res = buf.split_off(pos + target.len());
                let res = String::from_utf8(res)?;
                buf.drain(0..pos + target.len());
                return Ok(res);
            }
            if begin.elapsed().as_secs() > timeout as u64 {
                err!(
                    "Timeout! Expected: {}, Actual: {}",
                    expected,
                    String::from_utf8(buf.clone()).unwrap()
                );
                return Err(Box::<dyn Error>::from("Timeout"));
            }
        }
    }
}
impl CliTestApi for UiCliTester {
    fn wait_serial(&mut self, expected: &str, timeout: u32) -> Result<String, Box<dyn Error>> {
        self.do_wait_serial(expected, timeout, None)
    }
    fn script_run(&mut self, script: &str, timeout: u32) -> Result<String, Box<dyn Error>> {
        let mut cmd = script.to_owned();
        let echo_content_rand = rand_string(8);

        cmd += " && echo ";
        cmd += &echo_content_rand;
        cmd += " \n";

        self.run_command(&cmd)?;

        self.do_wait_serial(&echo_content_rand, timeout, Some(&echo_content_rand))
    }
    fn background_script_run(&mut self, script: &str) -> Result<(), Box<dyn Error>> {
        let mut cmd = script.to_owned();
        cmd += " &\n";
        self.run_command(&cmd)
    }
    fn writeln(&mut self, script: &str) -> Result<(), Box<dyn Error>> {
        let mut cmd = script.to_owned();
        cmd += "\n";
        self.run_command(&cmd)
    }
}
impl SudoCliTestApi for UiCliTester {
    fn script_sudo(
        &mut self,
        script: &str,
        timeout: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = String::from("sudo ");
        cmd += script;
        cmd += " ";
        self.script_run(&cmd, timeout)
    }
}

pub fn handle_uiclitester(inner: &mut Option<PyTtyInner>, term_id: u64) -> PyResult<()> {
    if inner.is_none() {
        return Err(PyRuntimeError::new_err(
            "You must define at least one valid object",
        ));
    }
    let mut be_wrapped = inner.take().unwrap();
    let tty = be_wrapped.safe_take()?;
    let tty = Box::into_inner(tty);
    let res =
        UiCliTester::build(tty, term_id).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let res = Box::new(res);
    *inner = Some(py_tty_inner(heap_raw(res)));
    return Ok(());
}

#[pyclass(extends=PyTty, subclass)]
pub struct UiExec {}

#[pymethods]
impl UiExec {
    #[new]
    #[pyo3(signature = (be_wrapped, term_id))]
    fn py_new(be_wrapped: &mut PyTty, term_id: u64) -> PyResult<(Self, PyTty)> {
        let mut inner = None;

        handle_wrap(&mut inner, Some(be_wrapped))?;
        handle_uiclitester(&mut inner, term_id)?;

        Ok((UiExec {}, PyTty::build(inner.unwrap())))
    }
    #[pyo3(signature = (script, timeout=None))]
    fn script_run(
        mut self_: PyRefMut<'_, Self>,
        script: &str,
        timeout: Option<u32>,
    ) -> PyResult<String> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        let timeout = timeout.unwrap_or(30);

        if inner.downcast_ref::<UiCliTester>().is_some() {
            let inner = inner.downcast_mut::<UiCliTester>().unwrap();
            let res = inner
                .script_run(script, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
            Ok(res)
        } else {
            Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ))
        }
    }

    fn background_script_run(mut self_: PyRefMut<'_, Self>, script: &str) -> PyResult<()> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if inner.downcast_ref::<UiCliTester>().is_some() {
            let inner = inner.downcast_mut::<UiCliTester>().unwrap();
            inner
                .background_script_run(script)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else {
            return Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ));
        }
        Ok(())
    }

    fn writeln(mut self_: PyRefMut<'_, Self>, script: &str) -> PyResult<()> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        if inner.downcast_ref::<UiCliTester>().is_some() {
            let inner = inner.downcast_mut::<UiCliTester>().unwrap();
            inner
                .writeln(script)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        } else {
            return Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ));
        }
        Ok(())
    }

    #[pyo3(signature = (expected, timeout=None))]
    fn wait_serial(
        mut self_: PyRefMut<'_, Self>,
        expected: &str,
        timeout: Option<u32>,
    ) -> PyResult<String> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        let timeout = timeout.unwrap_or(30);

        if inner.downcast_ref::<UiCliTester>().is_some() {
            let inner = inner.downcast_mut::<UiCliTester>().unwrap();
            let res = inner
                .wait_serial(expected, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
            Ok(res)
        } else {
            Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ))
        }
    }

    #[pyo3(signature = (script, timeout=None))]
    fn script_sudo(
        mut self_: PyRefMut<'_, Self>,
        script: &str,
        timeout: Option<u32>,
    ) -> PyResult<String> {
        let self_ = self_.as_mut();
        let inner = self_.inner.get_mut()?;
        let inner = inner.as_any_mut();

        let timeout = timeout.unwrap_or(30);

        if inner.downcast_ref::<UiCliTester>().is_some() {
            let inner = inner.downcast_mut::<UiCliTester>().unwrap();
            let res = inner
                .script_sudo(script, timeout)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
            Ok(res)
        } else {
            Err(PyRuntimeError::new_err(
                "Can't find the right object to run the script",
            ))
        }
    }
}
