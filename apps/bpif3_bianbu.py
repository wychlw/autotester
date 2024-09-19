"""
Gives a configuration,
runs a default procedure,
generates a default log.
"""

from boards.bpif3 import BPiF3
from system.generic import Generic
from utils.maintain_img import *
from utils.utils import swap_tty

from tester import *


def default_proc():
    """
    Default procedure for testing.
    """

    local_shell = Shell("bash")
    local_shell = Tee(local_shell, "run.log")
    local_shell.write(b"uname -a\n")
    local_shell.read()

    asciicast = Asciicast(local_shell)
    asciicast.begin()

    e = Exec(asciicast)

    board = BPiF3("id = 0\n", "/dev/ttyUSB0", 115200)

    url = "https://archive.spacemit.com/image/k1/version/bianbu/v2.0rc1/bianbu-24.04-desktop-k1-v2.0rc1-release-20240909135447.img.zip"
    work_dir = "/home/lw/Work/plct/boards/bpif3/bianbu"
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

    board.flash(e, img, "/dev/mmcblk0")

    info("Flash board ended...")

    console = board.get_console()
    console = Tee(console, "con.log")

    asciicast = e.exit()
    local_shell = swap_tty(asciicast, console)
    e = Exec(asciicast)

    board.power_cycle()

    info(f"Begin system test...")

    system = Generic("root", "bianbu", e)

    system.setup()
    system.loggin()

    asciicast = e.exit()
    logger = PyTty("wrap=true\nsimple_recorder=true\n", asciicast)
    logger.begin()
    e = Exec(logger)
    system.tty = e

    system.get_info()

    logger = e.exit()
    info_log = logger.end()
    asciicast = logger.exit()
    res = asciicast.end()

    with open("res.cast", "w") as f:
        f.write(res)

    with open("info.log", "w") as f:
        f.write(info_log)

if __name__ == "__main__":
    default_proc()
