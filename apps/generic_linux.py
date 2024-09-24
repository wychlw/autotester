#!/usr/bin/env python3
"""
Generic Linux testing procedure.
"""
import argparse
import tempfile
from boards.generic import GenericBoard
from system.generic import Generic
from utils.maintain_img import *
from utils.utils import swap_tty

from tester import *


def default_proc(url: str, work_dir: str, sd: str, username: str, passwd: str):
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

    board = GenericBoard(None, "/dev/ttyUSB0", 115200)

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

    board.flash(e, img, sd)

    info("Flash board ended...")

    console = board.get_console()
    console = Tee(console, "con.log")

    asciicast = e.exit()
    local_shell = swap_tty(asciicast, console)
    e = Exec(asciicast)

    board.power_cycle()

    info(f"Begin system test...")

    system = Generic(username, passwd, e)
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

def main():
    """
    For example:
    url = "https://mirror.tuna.tsinghua.edu.cn/ubuntu-cdimage/releases/24.10/beta/ubuntu-24.10-beta-preinstalled-server-riscv64%2Bnezha.img.xz"
    work_dir = "/home/lw/Work/plct/boards/d1/ubuntu"
    username = "ubuntu"
    passwd = "ubuntu"

    Then, run
    ./generic_linux.py -i "https://mirror.tuna.tsinghua.edu.cn/ubuntu-cdimage/releases/24.10/beta/ubuntu-24.10-beta-preinstalled-server-riscv64%2Bnezha.img.xz" -w "/home/lw/Work/plct/boards/d1/ubuntu" -u "ubuntu" -p "plct12321"
    """
    parser = argparse.ArgumentParser(description="Generic Linux testing procedure.")
    parser.add_argument("-i", "--img", type=str, help="Image url.", required=True)
    parser.add_argument("-w", "--work_dir", type=str, help="Working directory.")
    parser.add_argument("-s", "--sd", type=str, help="SD device.", default="/dev/sda")
    parser.add_argument("-u", "--username", type=str, help="Username.", required=True)
    parser.add_argument("-p", "--passwd", type=str, help="Password.", required=True)
    args = parser.parse_args()
    if args.work_dir is None:
        # Use tmp directory.
        args.work_dir = tempfile.mkdtemp()
    default_proc(args.img, args.work_dir, args.sd, args.username, args.passwd)

if __name__ == "__main__":
    main()
