#!/usr/bin/env python

from setuptools import setup, find_packages

setup(
    name='bitcoindevkit',
    version='0.0.1',
    packages=find_packages(where="src"),
    package_dir={"": "src"},
)
