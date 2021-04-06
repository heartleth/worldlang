import time
import sys

def print(*a):
    __builtins__["print"](*a, sep='', end='')
def println(*a):
    __builtins__["print"](*a, sep='')
def input_line(q=''):
    return __builtins__["input"](q)
def input(q=''):
    __builtins__["print"](q)
    ret = ''
    buf = ''
    while True:
        buf = sys.stdin.read(1)
        if buf != '\n' and buf != '\r' and buf != ' ' and buf != '\t':
            ret += buf
            break
    while True:
        buf = sys.stdin.read(1)
        if buf == '\n' or buf == '\r' or buf == ' ' or buf == '\t':
            break
        else :
            ret += buf
    return ret
def tup(*a):
    return (*a,)
def vec(*a):
    return [*a,]
def until(st, ed):
    return range(st, ed+1)
# sum
# max
def cat(c1, c2):
    return c1+c2
def map(c, fn):
    return __builtins__["map"](fn, c)
def each(c, fn):
    __builtins__["map"](fn, c)
def fold(c, fn):
    ret = c[0]
    for e in c[1:]:
        ret = fn(ret, e)
    return ret
def bfold(i, c, fn):
    ret = i
    for e in c:
        ret = fn(ret, e)
    return ret
def filter(c, fn):
    return filter(fn, c)
def integrate(c, fn):
    f = c[0]
    ret = [f]
    for e in c[1:]:
        f = fn(f, e)
        ret.insert(f)
    return ret
def wait(times):
    time.sleep(times/1000)
    return times
def get_time(fn):
    start = time.time();
    fn()
    println("{:.6} second(s) spent.".format(time.time()-start))
# Todo