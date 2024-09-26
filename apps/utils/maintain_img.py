"""
Some functions to download image from url

Functions:
    wget_image(url: str, dir_path: str) -> str
        res = the path of image file after download
        If download failed, res = None
        If file already exists, res = the path of image file
    
    extract_image(file: str) -> str
        res = the path of image file after extract
        If file is not image, res = None
        For now support .tar, .tar.gz, .tar.bz2, .tar.xz, .zip
"""
import os


def wget_image(url: str, dir_path: str) -> str:
    """
    Download image from url and save to dir_path, res = the path of image file.
    If download failed, res = None.
    If file already exists, res = the path of image file.
    """
    # get file name
    file_name = url.split('/')[-1]
    file_path = os.path.join(dir_path, file_name)
    if os.path.exists(file_path):
        res = file_path
        return res

    # The image is tend to be REALLY large... So just use wget in shell and wait for it...
    os.system(f"wget {url} -O {file_path}")

    if os.path.exists(file_path):
        res = file_path
    else:
        res = None
    return res

def extract_image(file: str) -> str:
    """
    Extract image from file and res = the path of image file.
    If file is not image, res = None.
    """
    curr = os.getcwd()
    os.chdir(os.path.dirname(file))
    res = "".join(file.split(".")[:-1])
    # check file type
    if not os.path.exists(file):
        os.chdir(curr)
        res = None
    elif not os.path.isfile(file):
        os.chdir(curr)
        res = None
    elif os.path.exists(res):
        os.chdir(curr)
        return res
    # For now support .tar, .tar.gz, .tar.bz2, .tar.xz, .zip .zst
    elif file.endswith(".zst"):
        if not os.path.exists(file[:-4]):
            os.system(f"zstd -d {file}")
        res = file[:-4]
    elif file.endswith(".tar"):
        if not os.path.exists(file[:-4]):
            os.system(f"tar -xf {file}")
        res = file[:-4]
    elif file.endswith(".tar.gz"):
        if not os.path.exists(file[:-7]):
            os.system(f"tar -xzf {file}")
        res = file[:-7]
    elif file.endswith(".tar.bz2"):
        if not os.path.exists(file[:-8]):
            os.system(f"tar -xjf {file}")
        res = file[:-8]
    elif file.endswith(".tar.xz"):
        if not os.path.exists(file[:-8]):
            os.system(f"tar -xJf {file}")
        res = file[:-8]
    elif file.endswith(".zip"):
        if not os.path.exists(file[:-4]):
            os.system(f"unzip {file}")
        res = file[:-4]
    elif file.endswith(".xz"):
        if not os.path.exists(file[:-3]):
            os.system(f"unxz -k {file}")
        res = file[:-3]
    elif file.endswith(".gz"):
        if not os.path.exists(file[:-3]):
            os.system(f"gunzip -k {file}")
        res = file[:-3]
    elif file.endswith(".bz2"):
        if not os.path.exists(file[:-4]):
            os.system(f"bunzip2 -k {file}")
        res = file[:-4]
    os.chdir(curr)
    return res
