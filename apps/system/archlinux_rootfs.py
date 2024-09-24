"""
For Armbian OS
"""

from time import sleep
from tester import Exec

from system.system import slp

class ArchlinuxRootfs:
    """
    System class for Archlinux.
    """

    def __init__(self, username, password, local: Exec, remote: Exec):
        self.username = username
        self.password = password
        self.local = local
        self.remote = remote

    def prepare(self, ori_img: str, rootfs_link: str):
        """
        Prepare the rootfs.
        """
        # download the rootfs
        self.local.script_run("wget " + rootfs_link)
        slp()
        # mount the image
        self.local.script_run("mkdir mnt")
        dev = self.local.script_sudo("losetup -f")
        self.local.script_sudo("losetup -P " + dev + " " + ori_img)
        parts = self.local.script_run("ls " + dev + "*")
        part = parts.split("\n")[-1]
        self.local.script_sudo("mount " + part + " mnt")
        slp()
        # move the old rootfs to old
        self.local.script_sudo("mkdir mnt/old")
        self.local.script_sudo("mv mnt/* mnt/old")
        slp()
        # extract the new rootfs
        self.local.script_sudo("tar -xf " + rootfs_link.split("/")[-1] + " -C mnt")
        slp()
        # move some files
        self.local.script_sudo("mv mnt/old/boot mnt/")
        self.local.script_sudo("mv mnt/old/home mnt/")
        self.local.script_sudo("cp -r mnt/old/lib/modules mnt/lib/")
        self.local.script_sudo("cp -r mnt/old/lib/firmware mnt/lib/")
        slp()
        # change fstab etc
        self.local.script_sudo("cp mnt/old/etc/fstab mnt/etc/")
        # finish
        self.local.script_sudo("umount mnt")
        self.local.script_sudo("losetup -d " + dev)

    def setup(self):
        "pass"

    def loggin(self):
        "pass"
        self.remote.wait_serial("login:", 600)
        slp()
        self.remote.writeln(self.username)
        slp(10)
        self.remote.writeln(self.password)
        slp()
        self.remote.wait_serial(self.username)
        slp()

    def get_info(self):
        "pass"
        self.remote.script_run("uname -a")
        self.remote.script_run("cat /etc/os-release")
        self.remote.script_run("cat /proc/cpuinfo")
