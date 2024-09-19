import tester

if __name__ == "__main__":
    s = tester.Shell("bash")
    s = tester.Tee(s, "run.log")
    e = tester.Exec(s)
    e.script_run("echo 'Hello, world'")
    e.script_run("uname -a")
    e.script_run("ls")
    e.script_run("uname -a")
    e.script_run("ls")
    e.script_run("uname -a")
    e.script_run("ls")
