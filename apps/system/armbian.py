"""
For Armbian OS
"""

from time import sleep
from tester import Exec

from system.system import slp

class Armbian:
    """
    System class for Armbian.
    """

    def __init__(self, tty: Exec):
        self.tty = tty

    def wln(self, cmd: str):
        """
        Write a command to the board.
        """
        self.tty.writeln(cmd)
        slp()

    def wit(self, wait: str, timeout: int = 600):
        """
        Wait for a string.
        """
        self.tty.wait_serial(wait, timeout)
        slp()

    def setup(self):
        """
        Setup the armbian.
        """

        # init settings
        self.wit("Create root password")
        self.wln("autotest_123")
        self.wit("Repeat root password")
        self.wln("autotest_123")
        # Choose default system command shell:
        # slp(2)
        # self.wln("1")

        # register new user
        self.wit("(eg. your first name)")
        self.wln("plct")
        self.wit("password")
        self.wln("plct_123")
        self.wit("password")
        self.wln("plct_123")
        self.wit("real name")
        self.wln("")

        try:
            self.wit("wireless", 10)
            self.wln("n")
        except Exception:
            pass

        # At your location, more locales are possible:
        self.wit("location")
        slp(2)
        self.wln("332")
        # Please select a continent, ocean, "coord", or "TZ".
        self.wit("#?")
        self.wln("4")
        # Please select a country whose clocks agree with yours.
        self.wit("#?")
        self.wln("10")
        # Set time zone
        self.wit("#?")
        self.wln("1")
        # Confirm time zone
        self.wit("#?")
        self.wln("1")

    def loggin(self):
        """
        Login to the board.
        """
        self.wit("root@")

    def get_info(self):
        """
        Get system information.
        """
        self.tty.script_run("uname -a")
        self.tty.script_run("cat /etc/os-release")
        self.tty.script_run("cat /proc/cpuinfo")
