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
        i = 0
        self.tty.wait_serial("login:", 600)
        try:
            while i < 4: # May be failed to login, try again.
                slp(10)
                self.tty.writeln(self.username)
                slp()
                slp(10) # Some system would gives you "密码" instead of "Password"... Just sleep for a while.
                slp()
                self.tty.writeln(self.password)
                slp()
                i += 1
                self.tty.wait_serial("login:", 5)
        except Exception as e:
            pass
        try:
            self.tty.wait_serial("Current", 5)
            # Means the password is expired.
            slp()
            self.tty.writeln(self.password)
            slp(5)
            self.password = "plct12321"
            self.tty.writeln(self.password)
            slp(5)
            self.tty.writeln(self.password)
        except Exception as e:
            pass

    def get_info(self):
        """
        Get system information.
        """
        self.tty.script_run("uname -a")
        slp()
        self.tty.script_run("cat /etc/os-release")
        slp()
        self.tty.script_run("cat /proc/cpuinfo")
