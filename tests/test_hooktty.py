import tester

class test_hook(tester.TtyAbst):
    def read(self) -> bytearray:
        res = b"Hello, world!\n"
        return res
    
    def read_line(self) -> bytearray:
        res = b"Hello, world!\n"
        return res
    
    def write(self, data: bytearray):
        print(data)

if __name__ == "__main__":
    hook = test_hook()
    hooked = tester.build_ttyhook(hook)
    hooked.write(b"Hello, world!\n")
    print(hooked.read())