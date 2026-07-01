"""The ``{{ cookiecutter.project.name }}`` package.

{{ cookiecutter.project.description }}
"""

from . import cli, core, util
from .cli import *
from .core import *
from .util import *

__version__ = '{{ cookiecutter.project.version }}'
