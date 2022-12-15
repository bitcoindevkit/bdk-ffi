import sys, os
sys.path.append(os.path.join(os.path.dirname(__name__), '.'))

project = 'bdkpython'
copyright = '2022, The Bitcoin Dev Kit developers'
author = 'The Bitcoin Dev Kit developers'
release = '0.25.0'

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.napoleon'
]

templates_path = ['_templates']
exclude_patterns = []

html_theme = 'alabaster'
html_static_path = ['_static']
