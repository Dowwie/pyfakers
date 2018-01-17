# py-fake-rs 


A fake data generator (a faker) for Python, extended with Rust.  The reason for
this library is performance.  When you're generating large amounts of fake data,
this library will serve you well.

This module contains bindings to [fake-rs](https://github.com/cksac/fake-rs),


## Example API

```python
from pyfakers import Name

print(Name.first_name())
print(Name.last_name())
```


## Installation

Install from pypi:
```
    pip install pyfakers
```

## Building from source

Install libffi, python3 headers, setuptools and wheel. The following command will install these on Ubuntu

```
    apt-get install libffi-dev python3-dev python3-setuptools python3-wheel
```

Check out the repository

```    
    git clone git@github.comdowwie/pyfakers.git
    cd pyfakers
```

In order to compile the Rust code, you'll need to have Rust nightly toolchain installed and enabled.

Use rustup to set this up, find installation instructions for rustup at https://www.rustup.rs/

To use Rust nightly, run the following commands (from the project root)

```
    rustup update nightly
    rustup override add nightly
```	

Install the ``pipenv`` utility from pypi

```
	pip install pipenv
```	

Install dependencies (from the project root)

```
	pipenv install
```	

Launch a virtual environment (from the project root)

```
	pipenv shell
```

Now you can build the package

```
    python3 setup.py bdist_wheel
```

Install the wheel

```
	pip install dist/<wheel file name>
```

## Full API 

```python
from pyfakers import Name, Internet, Company, Address, PhoneNumber

print(Name.first_name())
print(Name.last_name())
print(Name.full_name())
print(Name.title_descriptor())
print(Name.title_level())
print(Name.title_job())
print(Name.title())

print(Internet.free_email_provider())
print(Internet.domain_suffix())
print(Internet.user_name())
print(Internet.free_email())
print(Internet.safe_email())

print(Company.suffix())
print(Company.name())
print(Company.buzzword())
print(Company.catch_phrase())
print(Company.bs())
print(Company.profession())
print(Company.industry())

print(Address.time_zone())
print(Address.city_prefix())
print(Address.city_suffix())
print(Address.street_suffix())
print(Address.state())
print(Address.state_abbr())
print(Address.city())
print(Address.street_name())
print(Address.building_number())
print(Address.street_address())
print(Address.secondary_address())
print(Address.postal_code())
print(Address.latitude())
print(Address.longitude())

print(PhoneNumber.phone_number())
```
