import cffi
from . import _native


ffi = cffi.FFI()


def try_convert():
    ptr_name = _native.lib.full_name()
    name = ffi.string(ptr_name)
    return name

