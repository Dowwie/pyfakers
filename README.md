# py-fake-rs 


A fake data generator (a faker) for Python, extended with Rust.

This module contains bindings to [fake-rs](https://github.com/cksac/fake-rs),
a fake-data generator written in [Rust](https://www.rust-lang.org/).


## Installation

Install with pip, wheel files are provided for Linux and macOS:

    pip install pyfakers 

## Usage example

```python
import pyfakers as faker

print(faker.full_name())

```


## Building from source

Install libffi, python3 headers, setuptools and wheel. The following command will install these on Ubuntu:

    apt-get install libffi-dev python3-dev python3-setuptools python3-wheel

Check out the repository:

    git clone git@github.com:dowwie/pyfakers.git
    cd pyfakers

In order to compile the Rust code, you'll need to have Rust nightly toolchain installed and enabled.

Use rustup to set this up, find installation instructions for rustup at https://www.rustup.rs/

To use Rust nightly, run the following commands (from the project root):

    rustup update nightly
    rustup override add nightly

Install the ``pipenv`` utility from pypi:
	
	pip install pipenv

Install dependencies (from the project root):

	pipenv install

Launch a virtual environment (from the project root):

	pipenv shell

Now you can build the package:

    python3 setup.py bdist_wheel

Install the wheel

	pip install dist/<wheel file name>

