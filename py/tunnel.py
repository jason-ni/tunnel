import os
import ctypes
import time
import threading
import asyncio


def call_hello(golib, w):
    golib.Hello(w)


async def read_hello(r):
    loop = asyncio.get_event_loop()
    reader = asyncio.StreamReader(loop=loop)
    protocol = asyncio.StreamReaderProtocol(reader)
    transport, _ = await loop.connect_read_pipe(lambda: protocol, r)
    while True:
        out = await reader.read(16)
        print("=== from go:", out)


def main():
    r, w = os.pipe2(os.O_NONBLOCK | os.O_CLOEXEC)
    golib = ctypes.cdll.LoadLibrary('../gomodule/libgomodule.so')
    t = threading.Thread(target=call_hello, args=[golib, w])
    t.setDaemon(True)
    t.start()
    '''
    while True:
        print("=== before read")
        try:
            out = os.read(r, 10)
            print("==== from go:", out)
        except BlockingIOError as e:
            print("==== blocking io error: ", e)
            time.sleep(1)
    '''
    asyncio.get_event_loop().run_until_complete(read_hello(os.fdopen(r)))


if __name__ == '__main__':
    main()
