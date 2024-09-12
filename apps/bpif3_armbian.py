"""
Gives a configuration,
runs a default procedure,
generates a default log.
"""

from boards.bpif3 import BPiF3
from system.armbian import Armbian
from utils.maintain_img import *
from utils.utils import swap_tty

from tester import *


def default_proc():
    """
    Default procedure for testing.
    """

    local_shell = PyShell("/bin/bash")
    local_shell = PyTee(local_shell, "run.log")
    local_shell.write(b"uname -a\n")
    local_shell.read()

    asciicast = PyAsciicast(local_shell)
    asciicast.begin()

    e = PyExec(asciicast)

    board = BPiF3("id = 0\n", "/dev/ttyUSB0", 115200)

    url = "/AArmbian-bpi-SpacemiT_24.5.0-trunk_Bananapif3_mantic_legacy_6.1.15_xfce_desktop.img.xz" # 度盘，dummy
    work_dir = "/home/lw/Work/plct/boards/bpif3/armbian"
    img = wget_image(url, work_dir)
    if img is None:
        print("Download failed.")
        return
    img = extract_image(img)
    if img is None:
        print("Extract failed.")
        return
    info(f"Image is ready at {img}")

    info("Begin flashing board...")

    board.flash(e, img)

    info("Flash board ended...")

    console = board.get_console()

    asciicast = e.exit()
    local_shell = swap_tty(asciicast, console)
    e = PyExec(asciicast)

    board.power_cycle()

    info(f"Begin system test...")

    system = Armbian(e)

    system.loggin()

    system.get_info()

    asciicast = e.exit()
    res = asciicast.end()

    with open("res.cast") as f:
        f.write(res)

if __name__ == "__main__":
    default_proc()
