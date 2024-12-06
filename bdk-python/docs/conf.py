import os
import sys
sys.path.insert(0, os.path.abspath('../src'))

project = 'Bitcoin Dev Kit Python'
copyright = '2024, Bitcoin Dev Kit'
author = 'Bitcoin Dev Kit'

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.napoleon',
    'sphinx.ext.viewcode',
    'sphinx.ext.intersphinx',
]

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

autodoc_member_order = 'bysource'
autodoc_typehints = 'description'
