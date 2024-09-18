"""
For board banana pi f3
"""

from time import sleep
from tester import PyTty, Serial, SdWirec, Exec, info


class BPiF3:
    """
    Board class for Banana Pi F3.
    """

    def __init__(self, sdwirec_port="id = 0\n", serial_port="/dev/ttyUSB0", baud=115200) -> None:
        self.sdwirec = SdWirec(sdwirec_port)
        self.serial_port = serial_port
        self.baud = baud

    def flash(self, shell: Exec, img: str, dsk="/dev/sda"):
        """
        Flash the board with given image.
        """
        self.sdwirec.to_ts()
        sleep(0.5)
        shell.assert_script_sudo(
            f"dd if={img} of={dsk} status=progress", 600)
        shell.assert_script_sudo("sync")
        sleep(0.5)
        self.sdwirec.to_dut()
        sleep(0.5)

    def power_cycle(self):
        """
        Power cycle the board.
        Now we don't have a very good way though...
        Maybe some relay or something?
        But for now, manually power cycle the board.
        """
        info("Please power cycle the board and continue. You have arount 10s.")
        sleep(10)

    def get_console(self) -> PyTty:
        """
        Get the console of the board.
        """
        tty = Serial(self.serial_port, self.baud)
        return tty
