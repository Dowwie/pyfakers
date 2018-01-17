from pyfakers import Name, Internet, Company, Address, PhoneNumber
import pytest


def test_api():

    assert bool(Name.first_name())
    assert bool(Name.last_name())
    assert bool(Name.full_name())
    assert bool(Name.title_descriptor())
    assert bool(Name.title_level())
    assert bool(Name.title_job())
    assert bool(Name.title())

    assert bool(Internet.free_email_provider())
    assert bool(Internet.domain_suffix())
    assert bool(Internet.user_name())
    assert bool(Internet.free_email())
    assert bool(Internet.safe_email())

    assert bool(Company.suffix())
    assert bool(Company.name())
    assert bool(Company.buzzword())
    assert bool(Company.catch_phrase())
    assert bool(Company.bs())
    assert bool(Company.profession())
    assert bool(Company.industry())

    assert bool(Address.time_zone())
    assert bool(Address.city_prefix())
    assert bool(Address.city_suffix())
    assert bool(Address.street_suffix())
    assert bool(Address.state())
    assert bool(Address.state_abbr())
    assert bool(Address.city())
    assert bool(Address.street_name())
    assert bool(Address.building_number())
    assert bool(Address.street_address())
    assert bool(Address.secondary_address())
    assert bool(Address.postal_code())
    assert bool(Address.latitude())
    assert bool(Address.longitude())

    assert bool(PhoneNumber.phone_number())

