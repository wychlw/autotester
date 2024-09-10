from abc import ABC, abstractmethod
from .tester import PyTty

class TtyAbst(ABC):
    @abstractmethod
    def read(self) -> bytearray:
        pass

    @abstractmethod
    def read_line(self) -> bytearray:
        pass

    @abstractmethod
    def write(self, data: bytearray):
        pass
