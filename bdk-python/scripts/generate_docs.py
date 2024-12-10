#!/usr/bin/env python3
import os
import sys
import inspect
import importlib
from typing import Any, Dict, List

def get_class_methods(cls: type) -> List[Dict[str, Any]]:
    """Extract method information from a class."""
    methods = []
    for name, member in inspect.getmembers(cls, predicate=inspect.isfunction):
        if not name.startswith('_') or name == '__init__':
            sig = inspect.signature(member)
            doc = inspect.getdoc(member) or ''
            methods.append({
                'name': name,
                'signature': str(sig),
                'doc': doc
            })
    return methods

def get_class_info(cls: type) -> Dict[str, Any]:
    """Extract class information including docstring and methods."""
    return {
        'name': cls.__name__,
        'doc': inspect.getdoc(cls) or '',
        'methods': get_class_methods(cls)
    }

def format_rst_class(class_info: Dict[str, Any]) -> str:
    """Format class information as RST."""
    lines = []
    lines.append(f".. py:class:: {class_info['name']}")
    lines.append('')
    
    if class_info['doc']:
        lines.append(f"   {class_info['doc']}")
        lines.append('')
    
    for method in class_info['methods']:
        lines.append(f"   .. py:method:: {method['name']}{method['signature']}")
        lines.append('')
        if method['doc']:
            for doc_line in method['doc'].split('\n'):
                lines.append(f"      {doc_line}")
            lines.append('')
    
    return '\n'.join(lines)

def generate_module_docs(module) -> str:
    """Generate RST documentation for a module."""
    lines = []
    
    # Module header
    module_name = module.__name__.split('.')[-1]
    lines.append(f"{module_name.capitalize()} Module")
    lines.append('=' * len(f"{module_name} Module"))
    lines.append('')
    
    if module.__doc__:
        lines.append(module.__doc__)
        lines.append('')
    
    # Get all classes
    for obj in inspect.getmembers(module):
        if inspect.isclass(obj) and obj.__module__ == module.__name__:
            class_info = get_class_info(obj)
            lines.append(format_rst_class(class_info))
            lines.append('')
    
    return '\n'.join(lines)


def main():
    output_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), 'docs')
    os.makedirs(output_dir, exist_ok=True)
   
    # Generate main API documentation
    api_rst_path = os.path.join(output_dir, 'api.rst')
    
    with open(api_rst_path, 'w') as f:
        f.write("BDK Python API Reference\n")
        f.write("=====================\n\n")
        f.write("This document describes the Python API for the Bitcoin Development Kit (BDK).\n\n")
        
        # Generate docs for each module
        for module_name in ['bdk', 'bitcoin']:
            try:
                module = importlib.import_module(f'bdkpython.{module_name}')
                f.write(generate_module_docs(module))
            except ImportError as e:
                print(f"Warning: Could not import {module_name}: {e}", file=sys.stderr)
        
        # Add examples section to ast
        f.write("\nExamples\n")
        f.write("--------\n\n")
        f.write("Basic Wallet Usage\n")
        f.write("~~~~~~~~~~~~~~~~\n\n")
        f.write(".. code-block:: python\n\n")

        f.write("""   from bdkpython import *   
                # Create a new wallet
                descriptor = "wpkh(...)"  # Your descriptor here
                wallet = Wallet(descriptor, network=Network.TESTNET)
                
                # Sync wallet
                blockchain = Blockchain("https://blockstream.info/testnet/api")
                wallet.sync(blockchain)
                
                # Get balance
                balance = wallet.get_balance()
                print(f"Confirmed balance: {balance.confirmed}")
                """)

if __name__ == '__main__':
    main()
