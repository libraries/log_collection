import concurrent.futures
import json
import time

pool = concurrent.futures.ThreadPoolExecutor(4)


def echo(name: str):
    with open(name, "w") as f:
        for _ in range(1 << 32):
            f.write(json.dumps({"name": "json"}))
            f.write("\n")
            f.flush()
            time.sleep(1)


pool.submit(echo, "/tmp/log0")
pool.submit(echo, "/tmp/log1")
