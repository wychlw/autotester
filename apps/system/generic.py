"""
Generic means this system is out of box, no need to do anything.
"""

from time import sleep
from tester import Exec

from system.system import slp

class Generic:
    """
    System class for Generic Linux
    """

    def __init__(self, username, password, tty) -> None:
        self.username = username
        self.password = password
        self.tty = tty

    def setup(self):
        """
        Setup the system.
        """
        pass

    def loggin(self):
        """
        Loggin to the system.
        """
        self.tty.wait_serial("login:", 600)
        slp()
        self.tty.writeln(self.username)
        slp()
        slp(10) # Some system would gives you "密码" instead of "Password"... Just sleep for a while.
        slp()
        self.tty.writeln(self.password)
        slp()
        self.tty.wait_serial(self.username)
        slp()

    def get_info(self):
        """
        Get system information.
        """
        self.tty.script_run("uname -a")
        slp()
        self.tty.script_run("cat /etc/os-release")
        slp()
        self.tty.script_run("cat /proc/cpuinfo")
