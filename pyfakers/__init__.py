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


class Internet:

    def free_email_provider(self):
        return ffi.string(_native.lib.free_email_provider()).decode('utf-8')

    def domain_suffix(self):
        return ffi.string(_native.lib.domain_suffix()).decode('utf-8')

    def user_name(self):
        return ffi.string(_native.lib.user_name()).decode('utf-8')

    def free_email(self):
        return ffi.string(_native.lib.free_email()).decode('utf-8')

    def safe_email(self):
        return ffi.string(_native.lib.safe_email()).decode('utf-8')


class Company:

    def suffix(self):
        return ffi.string(_native.lib.suffix()).decode('utf-8')

    def name(self):
        return ffi.string(_native.lib.name()).decode('utf-8')

    def buzzword(self):
        return ffi.string(_native.lib.buzzword()).decode('utf-8')

    def catch_phrase(self):
        return ffi.string(_native.lib.catch_phrase()).decode('utf-8')

    def bs(self):
        return ffi.string(_native.lib.bs()).decode('utf-8')

    def profession(self):
        return ffi.string(_native.lib.profession()).decode('utf-8')

    def industry(self):
        return ffi.string(_native.lib.industry()).decode('utf-8')


class Address:

    def time_zone(self):
        return ffi.string(_native.lib.time_zone()).decode('utf-8')

    def city_prefix(self):
        return ffi.string(_native.lib.city_prefix()).decode('utf-8')

    def city_suffix(self):
        return ffi.string(_native.lib.city_suffix()).decode('utf-8')

    def street_suffix(self):
        return ffi.string(_native.lib.street_suffix()).decode('utf-8')

    def state(self):
        return ffi.string(_native.lib.state()).decode('utf-8')

    def state_abbr(self):
        return ffi.string(_native.lib.state_abbr()).decode('utf-8')

    def city(self):
        return ffi.string(_native.lib.city()).decode('utf-8')

    def street_name(self):
        return ffi.string(_native.lib.street_name()).decode('utf-8')

    def building_number(self):
        return ffi.string(_native.lib.building_number()).decode('utf-8')

    def street_address(self):
        return ffi.string(_native.lib.street_address()).decode('utf-8')

    def secondary_address(self):
        return ffi.string(_native.lib.secondary_address()).decode('utf-8')

    def postal_code(self):
        return ffi.string(_native.lib.postal_code()).decode('utf-8')

    def latitude(self):
        return ffi.string(_native.lib.latitude()).decode('utf-8')

    def longitude(self):
        return ffi.string(_native.lib.longitude()).decode('utf-8')


class PhoneNumber:

    def phone_number(self):
        return ffi.string(_native.lib.phone_number()).decode('utf-8')


Name = Name()
Internet = Internet()
Company = Company()
Address = Address()
PhoneNumber = PhoneNumber()
