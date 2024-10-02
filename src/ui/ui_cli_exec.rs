use std::{
    error::Error,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::{sleep, spawn, JoinHandle},
    time::{Duration, Instant},
};

use eframe::egui::mutex::Mutex;
use pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyRefMut, PyResult};

use crate::{
    cli::tty::{DynTty, Tty, WrapperTty}, consts::DURATION, err, exec::cli_api::{CliTestApi, SudoCliTestApi}, impl_any, info, pythonapi::shell_like::{handle_wrap, py_tty_inner, PyTty, PyTtyInner}, ui::util::get_sub_virt, util::{anybase::heap_raw, util::rand_string}
};

use super::terminal::{Terminal, TerminalMessage};

pub struct UiCliTester {
    inner: Arc<Mutex<Option<DynTty>>>,
    send: Arc<Mutex<Option<Sender<TerminalMessage>>>>,
    recv: Arc<Mutex<Option<Receiver<TerminalMessage>>>>,
    buf: Arc<Mutex<Vec<u8>>>,
    handle: Option<JoinHandle<()>>,
    exit: Arc<Mutex<bool>>,
}

impl UiCliTester {
    pub fn try_hook(&mut self, term: &mut Terminal) -> Result<(), Box<dyn Error>> {
        let send = self.send.clone();
        let recv = self.recv.clone();
        let mut send = send.lock();
        let mut recv = recv.lock();
        if send.is_some() || recv.is_some() {
            err!("Already hooked");
            return Err("Already hooked".into());
        }
        let (tx, rx) = mpsc::channel();
        *send = Some(tx);
        let o_rx = term.try_hook(rx)?;
        *recv = Some(o_rx);
        Ok(())
    }
    pub fn build(inner: DynTty, term: &mut Terminal) -> Result<Self, Box<dyn Error>> {
        println!("{}", get_sub_virt());
        let mut res = Self {
            inner: Arc::new(Mutex::new(Some(inner))),
            send: Arc::new(Mutex::new(None)),
            recv: Arc::new(Mutex::new(None)),
            buf: Arc::new(Mutex::new(Vec::new())),
            handle: None,
            exit: Arc::new(Mutex::new(false)),
        };
        res.try_hook(term)?;

        let inner = res.inner.clone();
        let send = res.send.clone();
        let recv = res.recv.clone();
        let buf = res.buf.clone();
        let exit = res.exit.clone();
        let handle = spawn(move || loop {
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
                let mut recv_o = recv.lock();
                if recv_o.is_none() {
                    continue;
                }
                let recv = recv_o.as_mut().unwrap();
                match recv.try_recv() {
                    Ok(TerminalMessage::Data(_)) => {
                        unreachable!()
                    }
                    Ok(TerminalMessage::Close) => {
                        let mut send_o = send.lock();
                        let send = send_o.as_ref().unwrap();
                        send.send(TerminalMessage::Close).unwrap();
                        recv_o.take();
                        send_o.take();
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
            let send_o = send.lock();
            let send = send_o.as_ref().unwrap();
            send.send(TerminalMessage::Data(data)).unwrap();
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
        let mut send_o = self.send.lock();
        if send_o.is_some() {
            let send = send_o.as_ref().unwrap();
            send.send(TerminalMessage::Close).unwrap();
        }
        send_o.take();
        let mut recv_o = self.recv.lock();
        recv_o.take();
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
        let echo_content_rand = String::from_utf8(rand_string(8)).unwrap();

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

pub fn handle_uiclitester(inner: &mut Option<PyTtyInner>, term_id: usize) -> PyResult<()> {
    if inner.is_none() {
        return Err(PyRuntimeError::new_err(
            "You must define at least one valid object",
        ));
    }
    let mut be_wrapped = inner.take().unwrap();
    let tty = be_wrapped.safe_take()?;
    let tty = Box::into_inner(tty);
    {
        // for window in sub_windows.iter_mut() {
        //     info!("Checking window: {} {} {}", window.title, window.id.value(), term_id);
        //     if window.id.value() != term_id as u64 {
        //         continue;
        //     }
        //     let window = &mut window.window;
        //     let term = window.as_any_mut().downcast_mut::<Terminal>();
        //     if term.is_none() {
        //         return Err(PyRuntimeError::new_err("Can't find the terminal"));
        //     }
        //     let term = term.unwrap();
        //     let res = UiCliTester::build(tty, term)
        //         .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        //     let res = Box::new(res);
        //     *inner = Some(py_tty_inner(heap_raw(res)));
        //     return Ok(());
        // }
        return Err(PyRuntimeError::new_err("Can't find the terminal"));
    }
}

#[pyclass(extends=PyTty, subclass)]
pub struct UiExec {}

#[pymethods]
impl UiExec {
    #[new]
    #[pyo3(signature = (be_wrapped, term_id))]
    fn py_new(be_wrapped: &mut PyTty, term_id: usize) -> PyResult<(Self, PyTty)> {
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
