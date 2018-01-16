# Rustface-py

A fake data generator (a faker) for Python, extended with Rust.

 * **Fast** - Implemented in Rust, which is [pretty fast](https://benchmarksgame.alioth.debian.org/u64q/rust.html)
 * **Easy to install** - [Portable](https://www.python.org/dev/peps/pep-0513/), prebuilt wheel files provided for Linux and macOS

This module contains bindings to [fake-rs](https://github.com/cksac/fake-rs),
a fake-data generator written in [Rust](https://www.rust-lang.org/).


## Installation

Install with pip, wheel files are provided for Linux and macOS:

    pip install pyfakers 

## Usage example

```python
from pyfakers import ..

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

## License

These bindings, [Rustface](https://github.com/atomashpolskiy/rustface/blob/master/LICENSE) and [SeetaFace](https://github.com/seetaface/SeetaFaceEngine/blob/master/LICENSE) are all released under the BSD 2-Clause license.
