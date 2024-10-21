#!/usr/bin/env python3
"""
Auto update support matrix result lib
"""

import os
import tempfile
import time
import sys
import importlib
import subprocess
import re

import pygit2

from .testlib.support_matrix_parser import System
from .testlib.template_parser import SysTemplate
from .utils.urlmarker import URL_REGEX


def maintain_template_lib(git_url: str, branch="test_template"):
    """
    maintain support_matrix template lib
    req: cwd under workspace
    """
    workspace = os.getcwd()
    template_lib = os.path.join(workspace, "template_lib")
    if not os.path.exists(template_lib):
        os.makedirs(template_lib)
    _repo = pygit2.clone_repository(
        git_url, template_lib, checkout_branch=branch)
    print(f"Template lib is maintained at: {template_lib}")
    return template_lib


def maintain_result_lib(git_url: str, branch="main"):
    """
    maintain support_matrix result lib
    req: cwd under workspace
    """
    workspace = os.getcwd()
    result_lib = os.path.join(workspace, "result_lib")
    if not os.path.exists(result_lib):
        os.makedirs(result_lib)
    _repo = pygit2.clone_repository(
        git_url, result_lib, checkout_branch=branch)
    print(f"Result lib is maintained at: {result_lib}")
    return result_lib


def walk_systems(template_lib: str, result_lib: str):
    """
    walk through template_lib and update result_lib
    """
    boards = filter(lambda x: x in [
        '.github',
        'assets',
        '.git',
        '.vscode',
        '__pycache__',
    ], os.listdir(template_lib))
    for board in boards:
        template_board_path = os.path.join(template_lib, board)
        if not os.path.isdir(template_board_path):
            continue
        systems = os.listdir(template_board_path)
        result_board_path = os.path.join(result_lib, board)
        for system in systems:
            template_system_path = os.path.join(template_board_path, system)
            if not os.path.isdir(template_system_path):
                continue
            result_system_path = os.path.join(result_board_path, system)
            handle_system(template_system_path, result_system_path)


def run_script(script: str, template: SysTemplate) -> SysTemplate:
    """
    script will always have a function:
    default_proc(url: str, work_dir: str, sd: str, username: str, passwd: str)
    the output is a res.cast and info.log, under cur dir
    """
    sys.path.append(os.path.dirname(script))
    mod = importlib.import_module(os.path.basename(script)[:-3])
    try:
        mod.default_proc(
            template.url,
            os.getcwd(),
            "todo!",
            template.username,
            template.password
        )
        with open("info.log", "r", "utf-8") as f:
            info = f.read()
            template.add_info(info)
        # upload the cast to asciinema
        cast = subprocess.run(
            ["asciinema", "upload", "res.cast"],
            capture_output=True,
            check=True
        )
        cast = cast.stdout.decode("utf-8")
        reg = re.compile(URL_REGEX)
        link = reg.search(cast).group()
        template.add_asciinema(link)
    except Exception as e:
        print(f"Runtime Error: {e} when running {
              script} with system {template.sys}")
        template.status = "cfh"
    finally:
        template.auto_add_img_content(template.image_link)
        template.add_username(template.username)
        template.add_password(template.password)
        template.add_version(template.sys_ver)
        current_time = time.strftime("%Y-%m-%d", time.localtime())
        template.last_update = current_time
    sys.path.pop()
    return template


def handle_system(template_system_path: str, result_system_path: str):
    res = System(result_system_path)
    template = SysTemplate(template_system_path)

    board_name = result_system_path.split('/')[-2]
    sys_name = template.sys

    # Currently, use update time to determine whether to update
    res_time = int(time.mktime(time.strptime(res.last_update, "%Y-%m-%d")))
    template_time = int(time.mktime(
        time.strptime(template.last_update, "%Y-%m-%d")))

    if template_time <= res_time:
        print(f"No need to update {board_name}/{sys_name}")
        return

    # for specific board, the script will be written in {board_name}/{sys_name}.py,
    # otherwise, use generic script
    script_path = os.path.abspath(__file__)
    script_path = os.path.dirname(script_path)

    specific_script = os.path.join(script_path, f"{board_name}/{sys_name}.py")
    generic_script = os.path.join(script_path, "generic.py")

    if os.path.exists(specific_script):
        script = specific_script
    else:
        script = generic_script

    template = run_script(script, template)
    new_content = str(template)
    with open(os.path.join(result_system_path, "README.md"), "w", encoding="utf-8") as f:
        f.write(new_content)
    print(f"Renew {board_name}/{sys_name} result")

def main():
    # create a temproary workspace from mktemp -d
    workspace = tempfile.mkdtemp()
    os.chdir(workspace)
    print(f"Workspace: {workspace}")

    git_url = "https://github.com/wychlw/support-matrix.git"

    template_lib = maintain_template_lib(git_url)
    result_lib = maintain_result_lib(git_url)

    walk_systems(template_lib, result_lib)

if __name__ == "__main__":
    main()
