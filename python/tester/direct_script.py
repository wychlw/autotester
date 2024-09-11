from .hook import GlobalCallHook
from . import tester

"""
对于直接执行的命令，会被翻译为 assert_script_run
对于 write 和 run，会被翻译为 writeln
对于 wait，会被翻译为 wait_serial
"""
class DirectScript(GlobalCallHook):
    def hook_func(func, *args, **kwargs):
        if globals().get(func) is not None:
            return globals()[func](*args, **kwargs)
        cmd = f"{func[1][1]} "
        for arg in args:
            cmd += f"{arg} "
        for key, value in kwargs.items():
            cmd += f"{key}={value} "
        global __hook_exec__
        __hook_exec__.assert_script_run(cmd)
    
    def write_func(*args, **kwargs):
        cmd = ""
        for arg in args:
            cmd += f"{arg} "
        for key, value in kwargs.items():
            cmd += f"{key}={value} "
        global __hook_exec__
        __hook_exec__.writeln(cmd)
    
    def wait_func(*args, **kwargs):
        cmd = ""
        for arg in args:
            cmd += f"{arg} "
        for key, value in kwargs.items():
            cmd += f"{key}={value} "
        global __hook_exec__
        __hook_exec__.wait_serial(cmd)
    
    def __init__(self, func, exec):
        super().__init__(func)
        self.set_global("__hook_func__", DirectScript.hook_func)
        self.set_global("__hook_exec__", exec)
        self.set_global("write", DirectScript.write_func)
        self.set_global("run", DirectScript.write_func)
        self.set_global("wait", DirectScript.wait_func)


