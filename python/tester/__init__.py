from .tester import *

__doc__ = tester.__doc__
if hasattr(tester, "__all__"):
    __all__ = tester.__all__

from .hook import HookGlobals
from .direct_script import DirectScript