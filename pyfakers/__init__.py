import ctypes
from . import _native


ptr = _native.lib.full_name()

def try_convert():
    name = ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
    return name

