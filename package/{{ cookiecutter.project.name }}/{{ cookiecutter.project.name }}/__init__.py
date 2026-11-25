"""The ``{{ cookiecutter.project.name }}`` package.

{{ cookiecutter.project.description }}
"""
{%- if cookiecutter.rust_bindings != 'bin' %}

from . import cli, constants, core, exceptions, typing, util
from .cli import *
from .constants import *
from .core import *
{%- endif %}

__version__ = '{{ cookiecutter.project.version }}'
