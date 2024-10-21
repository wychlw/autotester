"""
Parse metadata for systems
"""
import os
import re
import urllib.parse
import frontmatter

class SysTemplate:
    """
    eg:
    ---
    sys: bianbu
    sys_ver: 2.0rc1
    sys_var: null

    status: basic
    last_update: 2024-09-20

    image_link: https://archive.spacemit.com/image/k1/version/bianbu/v2.0rc1/bianbu-24.04-desktop-k1-v2.0rc1-release-20240909135447.img.zip
    username: root
    password: bianbu
    ---
    """
    sys: str
    sys_ver: str | None
    sys_var: str | None
    status: str
    last_update: str
    image_link: str
    username: str
    password: str

    markdown_content: str

    def __init_by_file(self, path):
        meta_path = os.path.join(path, 'README.md')
        if not os.path.exists(meta_path):
            raise FileNotFoundError(f"{meta_path} not found")
        with open(meta_path, 'r', encoding="utf-8") as file:
            post = frontmatter.load(file)
            if 'sys' not in post.keys():
                raise FileNotFoundError(f"{meta_path} has no frontmatter")
            self.sys = post['sys']
            self.sys_ver = post['sys_ver']
            self.sys_var = post['sys_var']
            self.status = post['status']
            self.last_update = post['last_update']
            self.image_link = post['image_link']
            self.username = post['username']
            self.password = post['password']
            self.markdown_content = post.content

    def __init__(self, *args, **kwargs):
        if len(kwargs) > 0:
            self.sys = kwargs['sys']
            self.sys_ver = kwargs['sys_ver']
            self.sys_var = kwargs['sys_var']
            self.status = kwargs['status']
            self.last_update = kwargs['last_update']
            self.image_link = kwargs['image_link']
            self.username = kwargs['username']
            self.password = kwargs['password']
            self.markdown_content = ""
        else:
            self.__init_by_file(args[0])
    def add_asciinema(self, link: str):
        """
        eg: [![asciicast](https://asciinema.org/a/sAccZbGletHEuqNUrHYeCZkLa.svg)](https://asciinema.org/a/sAccZbGletHEuqNUrHYeCZkLa)
        """
        self.markdown_content.replace("[[asciinema]]", f"[![asciinema]({link}.svg)]({link})")
    def add_info(self, info: str):
        """
        The output of the script
        """
        self.markdown_content.replace("[[info]]", info)
    def add_version(self, version: str):
        """
        eg: v2.0rc1
        """
        self.markdown_content.replace("[[version]]", version)
    def add_image_link(self, link: str):
        """
        eg: https://archive
        """
        self.markdown_content.replace("[[image_link]]", link)
    def add_image_zip(self, link: str):
        """
        eg: archive.img.tar.gz
        """
        self.markdown_content.replace("[[image_file_zip]]", link)
    def add_image_img(self, link: str):
        """
        eg: archive.img
        """
        self.markdown_content.replace("[[image_file_img]]", link)
    def auto_add_img_content(self, link: str):
        """
        link is the link to the image file
        """
        img_zip_name = urllib.parse.urlparse(link).path.split("/")[-1]
        img_name = img_zip_name
        if img_name.contains(".img"):
            img_name = img_name.split(".img")[0] + ".img"
        self.add_image_link(link)
        self.add_image_zip(img_zip_name)
        self.add_image_img(img_name)
    def add_username(self, username: str):
        """
        eg: root
        """
        self.markdown_content.replace("[[username]]", username)
    def add_password(self, password: str):
        """
        eg: root
        """
        self.markdown_content.replace("[[password]]", password)
    def strip_replacement(self) -> str:
        """
        replace `[[.*]]` to null
        """
        strinfo = re.compile(r"\[\[.*\]\]")
        return strinfo.sub("", self.markdown_content)
    def __str__(self):
        article = frontmatter.Post(self.strip_replacement())
        article.metadata.update({
            "sys": self.sys,
            "sys_ver": self.sys_ver,
            "sys_var": self.sys_var,
            "status": self.status,
            "last_update": self.last_update,
        })
        return frontmatter.dumps(article)
