import cffi
from . import _native


ffi = cffi.FFI()


class Name:

    def first_name(self):
        return ffi.string(_native.lib.first_name()).decode('utf-8')

    def last_name(self):
        return ffi.string(_native.lib.last_name()).decode('utf-8')

    def full_name(self):
        return ffi.string(_native.lib.full_name()).decode('utf-8')

    def title_descriptor(self):
        return ffi.string(_native.lib.title_descriptor()).decode('utf-8')

    def title_level(self):
        return ffi.string(_native.lib.title_level()).decode('utf-8')

    def title_job(self):
        return ffi.string(_native.lib.title_job()).decode('utf-8')

    def title(self):
        return ffi.string(_native.lib.title()).decode('utf-8')


Name = Name()
