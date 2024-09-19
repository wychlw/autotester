import tester

if __name__ == "__main__":
    s = tester.Shell("bash")
    e = tester.Exec(s)
    e.script_run("uname -a")
    e.script_run("ls")
    e.script_run("uname -a")
    e.script_run("ls")
    e.script_run("uname -a")
    e.script_run("ls")
