"""
For Armbian OS
"""

from time import sleep
from tester import PyExec

def slp(t = 0.5):
    """
    Sleep for t second.
    """
    sleep(t)

class Armbian:
    """
    System class for Armbian.
    """
    def __init__(self, tty: PyExec):
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

    def loggin(self):
        """
        Loggin to the board.
        """

        # init settings
        self.wit("IP address:")
        self.wln("")
        self.wit("Create root password:")
        self.wln("autotest_123")
        self.wit("Repeat root password:")
        self.wln("autotest_123")
        # Choose default system command shell:
        slp(2)
        self.wln("1")

        # register new user
        self.wit("(eg. your first name):")
        self.wln("plct")
        self.wit("password:")
        self.wln("plct_123")
        self.wit("password:")
        self.wln("plct_123")
        self.wit("real name:")
        self.wln("Plct")

        # Set user language based on your location? [Y/n]
        self.wit("[Y/n]")
        self.wln("y")
        # Set time zone
        self.wit("choice:")
        self.wln("328")
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

        # Loggin
        self.wit("root@")

    def get_info(self):
        """
        Get system information.
        """
        self.tty.assert_script_run("uname -a")
        self.tty.assert_script_run("cat /etc/os-release")
        self.tty.assert_script_run("cat /proc/cpuinfo")
