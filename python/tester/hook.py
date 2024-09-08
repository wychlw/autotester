from typing import Generator, Tuple
from bytecode import *
import inspect

"""
对于这里我们想要 hook 的东西，其大概会长成这个样子：
```python
def func():
    ping("127.0.0.1")
```
里面的 ping 部分 dis 出来的 bytecode 大概是这样的：
```
<LOAD_GLOBAL arg=(True, 'ping') location=InstrLocation(lineno=2, end_lineno=2, col_offset=1, end_col_offset=5)>
<LOAD_CONST arg='192.168.1.1' location=InstrLocation(lineno=2, end_lineno=2, col_offset=6, end_col_offset=19)>
<CALL arg=1 location=InstrLocation(lineno=2, end_lineno=2, col_offset=1, end_col_offset=20)>
<POP_TOP location=InstrLocation(lineno=2, end_lineno=2, col_offset=1, end_col_offset=20)>
```

对于那些在 global 里面的东西，我们不管它；而不在 global 里面的东西，我们就要 hook 了。
hook 函数的形式大概会长这样：
```python
def hook_func('name', *args, **kwargs):
    pass
```
在里面我们再去查找什么 globals 啊之类的。

也就是说，我们需要去找这样一个 LOAD_GLOBAL 和 CALL 的组合，并且中间的所有 LOADxxx 和 arg 数量一致。而后，修改该段字节码，使得其实际调用的是我们的 hook_func。
注意需要考虑多个 ARGS 和 KWARGS 的问题...
"""

class HookGlobals:
    def find_paired_load_global(self, code: Bytecode, call_pos: int) -> int:
        instr = code[call_pos]
        arg_count = instr.arg
        if call_pos - 1 < 0:
            return -1
        if code[call_pos - 1].name == "KW_NAMES":
            call_pos -= 1
        if call_pos - arg_count < 0:
            return -1
        for j in range(1, arg_count + 1):
            instr = code[call_pos - j]
            if type(instr) != Instr:
                return -1
            if instr.name != "LOAD_GLOBAL" and \
                instr.name != "LOAD_CONST" and \
                instr.name != "LOAD_FAST" and \
                instr.name != "LOAD_DEREF" and \
                instr.name != "LOAD_NAME":
                return -1
        if code[call_pos - arg_count - 1].name != "LOAD_GLOBAL":
            return -1
        return call_pos - arg_count - 1
    
    def find_paired_call(self, code: Bytecode) -> Generator[Tuple[int, int], None, None]:
        for i in reversed(range(len(code))):
            instr = code[i]
            if type(instr) == Instr and instr.name == "CALL":
                j = self.find_paired_load_global(code, i)
                if j != -1:
                    yield j, i
    
    def replace_global_call(self, func):
        codes = Bytecode.from_code(func.__code__)
        for i, j in self.find_paired_call(codes):
            orig_func = codes[i].arg
            replace_code = [
                Instr('LOAD_GLOBAL', (True, '__hook_func__')),
                Instr('LOAD_CONST', (True, orig_func)),
            ]
            codes[i:i + 1] = replace_code
            arg_count = codes[j + 1].arg
            replace_code_2 = [
                Instr('CALL', arg_count + 1)
            ]
            codes[j + 1:j + 2] = replace_code_2
        func.__code__ = codes.to_code()
        return func
    
    def hook_call(func_name, *args, **kwargs):
        print(func_name)
        print(args)
        print(kwargs)
        print(f"Hooked {func_name} with {args} and {kwargs}")
    
    def set_global(self, name, attr):
        # This is for using it in every global scope
        for frame in inspect.stack():
            frame[0].f_globals[name] = attr
    
    def __init__(self, func):
        self.set_global("__hook_func__", HookGlobals.hook_call)
        if not hasattr(func, 'hooked__'):
            self.replace_global_call(func)
            func.__hooked__ = True
