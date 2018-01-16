import os
from setuptools import setup, find_packages, Command
from pipenv.project import Project
from pipenv.utils import convert_deps_to_pip

here = os.path.abspath(os.path.dirname(__file__))

pfile = Project(chdir=False).parsed_pipfile
requirements = convert_deps_to_pip(pfile['packages'], r=False)
test_requirements = convert_deps_to_pip(pfile['dev-packages'], r=False)


class CleanCommand(Command):
    """Custom clean command to tidy up the project root."""
    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        os.system('rm -vrf ./build ./dist ./*.pyc ./*.tgz ./*.egg-info; py.cleanup -d')


try:
    with open(os.path.join(here, 'README.md')) as f:
        README = f.read()
except IOError:
    README = ''


def build_native(spec):
    # build an example rust library
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='./rust'
    )

    spec.add_cffi_module(
        module_path='pyfakers._native',
        dylib=lambda: build.find_dylib('pyfakers', in_path='target/release'),
        header_filename=lambda: build.find_header('pyfakers.h', in_path='target')
    )


setup(
    name='pyfakers',
    description="A fake data generator backed by fake-rs",
    long_description=README,
    packages=find_packages(where='.', exclude=['tests']),
    setup_requires=[
        'milksnake',
        'setuptools_scm >= 1.7.0'
    ],
    install_requires=requirements,
    zip_safe=False,
    cmdclass={'clean': CleanCommand},
    platforms='any',
    milksnake_tasks=[
        build_native
    ]
)
