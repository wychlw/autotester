"""
Some functions to download image from url

Functions:
    wget_image(url: str, dir_path: str) -> str
        return the path of image file after download
        If download failed, return None
        If file already exists, return the path of image file
    
    extract_image(file: str) -> str
        return the path of image file after extract
        If file is not image, return None
        For now support .tar, .tar.gz, .tar.bz2, .tar.xz, .zip
"""
import os


def wget_image(url: str, dir_path: str) -> str:
    """
    Download image from url and save to dir_path, return the path of image file.
    If download failed, return None.
    If file already exists, return the path of image file.
    """
    # get file name
    file_name = url.split('/')[-1]
    file_path = os.path.join(dir_path, file_name)
    if os.path.exists(file_path):
        return file_path

    # The image is tend to be REALLY large... So just use wget in shell and wait for it...
    os.system(f"wget {url} -O {file_path}")

    if os.path.exists(file_path):
        return file_path
    else:
        return None

def extract_image(file: str) -> str:
    """
    Extract image from file and return the path of image file.
    If file is not image, return None.
    """
    # check file type
    if not os.path.exists(file):
        return None
    if not os.path.isfile(file):
        return None
    # For now support .tar, .tar.gz, .tar.bz2, .tar.xz, .zip
    if file.endswith(".tar"):
        if os.path.exists(file[:-4]):
            return file[:-4]
        os.system(f"tar -xf {file}")
        return file[:-4]
    if file.endswith(".tar.gz"):
        if os.path.exists(file[:-7]):
            return file[:-7]
        os.system(f"tar -xzf {file}")
        return file[:-7]
    if file.endswith(".tar.bz2"):
        if os.path.exists(file[:-8]):
            return file[:-8]
        os.system(f"tar -xjf {file}")
        return file[:-8]
    if file.endswith(".tar.xz"):
        if os.path.exists(file[:-8]):
            return file[:-8]
        os.system(f"tar -xJf {file}")
        return file[:-8]
    if file.endswith(".zip"):
        if os.path.exists(file[:-4]):
            return file[:-4]
        os.system(f"unzip -k {file}")
        return file[:-4]
    if file.endswith(".xz"):
        if os.path.exists(file[:-3]):
            return file[:-3]
        os.system(f"unxz -k {file}")
        return file[:-3]
    if file.endswith(".gz"):
        if os.path.exists(file[:-3]):
            return file[:-3]
        os.system(f"gunzip -k {file}")
        return file[:-3]
    if file.endswith(".bz2"):
        if os.path.exists(file[:-4]):
            return file[:-4]
        os.system(f"bunzip2 -k {file}")
        return file[:-4]
    return None
