from .hook import GlobalCallHook
from . import tester

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
        __hook_exec__.script_run(cmd)
        
    def __init__(self, func, exec):
        super().__init__(func)
        self.set_global("__hook_func__", DirectScript.hook_func)
        self.set_global("__hook_exec__", exec)

