"""
This module contains the abstract class for all systems.
"""

from abc import ABC, abstractmethod
from time import sleep
from tester import Exec


def slp(t=0.5):
    """
    Sleep for t second.
    """
    sleep(t)

class Systems(ABC):
    """
    Abstract class for all systems.
    This is mainly an interface definition, instead of a base class.
    """
    @abstractmethod
    def loggin(self):
        """
        Loggin to the board.
        """

    @abstractmethod
    def setup(self, *args, **kwargs):
        """
        some system may need initial setup.
        """

    @abstractmethod
    def get_info(self):
        """
        Get system information.
        """
