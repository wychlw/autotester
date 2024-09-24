"""
For board banana pi f3
"""

from time import sleep
from tester import PyTty, Serial, SdWirec, Exec, info


class GenericBoard:
    """
    Board class for GenericBoard.
    """

    def __init__(self, sdwirec_port=None, serial_port="/dev/ttyUSB0", baud=115200) -> None:
        if sdwirec_port is not None:
            self.sdwirec = SdWirec(sdwirec_port)
        else:
            self.sdwirec = None
        self.serial_port = serial_port
        self.baud = baud

    def flash(self, shell: Exec, img: str, dsk="/dev/sda"):
        """
        Flash the board with given image.
        """
        if self.sdwirec is not None:
            self.sdwirec.to_ts()
        else:
            info("Please insert the SD card to the tester and continue.")
            sleep(10)
        sleep(0.5)
        shell.script_sudo(
            f"dd if={img} of={dsk} status=progress bs=4M ", 600)
        shell.script_sudo("sync")
        sleep(0.5)
        if self.sdwirec is not None:
            self.sdwirec.to_dut()
        else:
            info("Please insert the SD card to the board and continue.")
            sleep(10)
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
