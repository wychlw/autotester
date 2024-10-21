from .tester import *
from .hook import GlobalCallHook
from .direct_script import DirectScript
from .tty_abst import TtyAbst

__doc__ = None
if hasattr(tester, "__all__"):
    __all__ = tester.__all__

ui.__init_sub_virt__("not_in_GUI_env")
