"""
This module contains the abstract class for all boards.
"""

from abc import ABC, abstractmethod
from tester import Tty


class Boards(ABC):
    """
    Abstract class for all boards.
    This is mainly an interface definition, instead of a base class.
    """
    @abstractmethod
    def flash(self, img: str, *args, **kwargs):
        """
        Flash the board with given image.
        """

    @abstractmethod
    def power_cycle(self, *args, **kwargs):
        """
        Power cycle the board. 
        Basically, trigger a reboot or reset to the board.
        """

    @abstractmethod
    def get_console(self, *args, **kwargs) -> Tty:
        """
        Get the console of the board.
        """
