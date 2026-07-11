"""The ``{{ cookiecutter.project.name }}`` package.

{{ cookiecutter.project.description }}
"""

from . import cli, core, exceptions, typing, util
from .cli import *
from .core import *

__version__ = '{{ cookiecutter.project.version }}'
