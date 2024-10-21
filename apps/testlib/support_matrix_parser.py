#!/usr/bin/env python3
"""
Parse metadata of systems and boards
"""
import os
import yaml
import frontmatter

class System:
    """
    eg:
    ---
    sys: deepin
    sys_ver: 23
    sys_var: null

    status: basic
    last_update: 2024-06-21
    ---
    """
    sys: str
    sys_ver: str | None
    sys_var: str | None
    status: str
    last_update: str
    link: str | None

    def strip(self):
        """
        dummy for strip the system
        """
        return self

    def __str__(self):
        return status_map(self.status)

    def __len__(self):
        return len(status_map(self.status))

    def __init_by_file(self, path, base_link=""):
        base_name = os.path.basename(path)
        self.link = os.path.join(base_link, base_name)
        meta_path = os.path.join(path, 'README.md')
        if not os.path.exists(meta_path):
            raise FileNotFoundError(f"{meta_path} not found")
        with open(meta_path, 'r', encoding="utf-8") as file:
            post = frontmatter.load(file)
            if 'sys' not in post.keys():
                raise FileNotFoundError(f"{meta_path} has no frontmatter")
            if post['sys'] == 'revyos':
                self.sys = 'debian'
            else:
                self.sys = post['sys']
            self.sys_ver = post['sys_ver']
            self.sys_var = post['sys_var']
            self.status = post['status']
            self.last_update = post['last_update']

    def __init__(self, *args, **kwargs):
        if len(kwargs) > 0:
            self.sys = kwargs['sys']
            self.sys_ver = kwargs['sys_ver']
            self.sys_var = kwargs['sys_var']
            self.status = kwargs['status']
            self.last_update = kwargs['last_update']
            self.link = kwargs['link']
        else:
            self.__init_by_file(*args, **kwargs)


def status_map(status: str):
    """
    map status to pretty string
    """
    if status == 'wip':
        return 'WIP'
    if status == 'cft':
        return 'CFT'
    if status == 'cfh':
        return 'CFH'
    if status == 'basic':
        return 'Basic'
    if status == 'good':
        return 'Good'
    return status


class Board:
    """
    a collection of systems and eg:
    ---
    product: VisionFive 2
    cpu: JH7110
    cpu_core: SiFive U74 + SiFive S7 + SiFive E24
    ---
    """
    product: str
    cpu: str
    link: str
    cpu_core: str
    systems: list[System]

    def append_system(self, system: System):
        """
        append a system to the board
        """
        self.systems.append(system)

    def gen_row(self, system_arr: dict[str]):
        """
        generate a row of the table
        """
        row = [
            self.cpu,
            self.cpu_core,
            self.link,
            self
        ]

        na_count = 0

        for k, _ in system_arr.items():
            for system in self.systems:
                if system.sys == k:
                    row.append(system)
                    break
            else:
                row.append('N/A')
                na_count += 1

        if na_count == len(system_arr):
            return None

        return row

    def strip(self):
        """
        dummy for strip the board
        """
        self.product = self.product.strip()
        return self

    def __str__(self):
        return self.product

    def __len__(self):
        return len(self.product)

    def __init__(self, path: str):
        base_name = os.path.basename(path)
        self.link = base_name
        readme_path = os.path.join(path, 'README.md')
        if not os.path.exists(readme_path):
            raise FileNotFoundError(f"{readme_path} not found")
        with open(readme_path, 'r', encoding="utf-8") as file:
            post = frontmatter.load(file)
            self.product = post['product']
            self.cpu = post['cpu']
            self.cpu_core = post['cpu_core']
        self.systems = []

        for folder in os.listdir(path):
            if os.path.isdir(os.path.join(path, folder)):
                try:
                    system = System(os.path.join(path, folder), self.link)
                except FileNotFoundError as e:
                    global check_success
                    check_success = False
                    print(f"Error: {e}")
                    continue
                self.append_system(system)

        if not os.path.exists(os.path.join(path, 'others.yml')):
            return
        with open(os.path.join(path, 'others.yml'), 'r', encoding="utf-8") as file:
            data = yaml.load(file, Loader=yaml.FullLoader)
            for i in data:
                system = System(
                    sys=i['sys'],
                    sys_ver=i['sys_ver'],
                    sys_var=i['sys_var'],
                    status=i['status'],
                    last_update='2000-00-00',
                    link=None
                )
                self.append_system(system)


class Systems:
    """
    support matrix of systems
    """
    linux: dict[str]
    bsd: dict[str]
    rtos: dict[str]
    others: dict[str]

    exclude_dir = [
        '.github',
        'assets',
        '.git',
        '.vscode',
        '__pycache__',
    ]
    boards: list[Board]

    def __init__(self, path):
        meta_path = os.path.join(path, 'assets', 'metadata.yml')
        with open(meta_path, 'r', encoding="utf-8") as file:
            def mp(x):
                res = {}
                for l in x:
                    for i in l.items():
                        res[i[0]] = i[1]
                return res
            data = yaml.load(file, Loader=yaml.FullLoader)
            self.linux = mp(data['linux'])
            self.bsd = mp(data['bsd'])
            self.rtos = mp(data['rtos'])
            self.others = mp(data['others'])
        self.boards = []
        for folder in os.listdir(path):
            if folder in self.exclude_dir:
                continue
            p = os.path.join(path, folder)
            if not os.path.isdir(p):
                continue
            try:
                board = Board(p)
                self.boards.append(board)
            except FileNotFoundError as e:
                global check_success
                check_success = False
                print(f"Error: {e}")
                continue
