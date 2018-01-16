import cffi
from . import _native


ffi = cffi.FFI()


full_name = lambda: ffi.string(_native.lib.full_name()).decode('utf-8')
