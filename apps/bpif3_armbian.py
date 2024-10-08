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

    local_shell = Shell("/bin/bash")
    local_shell = Tee(local_shell, "run.log")

    asciicast = Asciicast(local_shell)
    asciicast.begin()

    e = Exec(asciicast)
    e.script_run("uname -a")

    e.script_run("uname -a")


    e.script_run("uname -a")


    board = BPiF3("id = 0\n", "/dev/ttyUSB0", 115200)

    url = "https://mirrors.tuna.tsinghua.edu.cn/armbian-releases/bananapif3/archive/Armbian_24.8.1_Bananapif3_noble_legacy_6.1.15_minimal.img.xz"
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

    board.flash(e, img, "/dev/sda")

    info("Flash board ended...")

    console = board.get_console()
    console = Tee(console, "con.log")

    asciicast = e.exit()
    local_shell = swap_tty(asciicast, console)
    e = Exec(asciicast)

    board.power_cycle()

    info(f"Begin system test...")

    system = Armbian(e)

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
