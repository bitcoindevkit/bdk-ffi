import os
import re

# Define the directory where the Python source files are located
src_dir = 'src/bdkpython'
package_root = 'bdkpython'

# Define the output file for the API documentation
output_file = 'docs/api.rst'

# Regex pattern to match class definitions
class_pattern = re.compile(r'^class ([A-Za-z][A-Za-z0-9_]*)')

# Store classes with their full module path
public_classes = {}

for root, _, files in os.walk(src_dir):
    for file in files:
        if file.endswith('.py'):
            module_path = os.path.relpath(os.path.join(root, file), src_dir)
            module_path = module_path.replace(os.sep, '.').removesuffix('.py')
            full_module = f'{package_root}.{module_path}'
            with open(os.path.join(root, file), 'r') as f:
                for line in f:
                    match = class_pattern.match(line)
                    if match:
                        class_name = match.group(1)
                        if not class_name.startswith('_'):
                            fqcn = f'{full_module}.{class_name}'
                            public_classes[fqcn] = True

# Generate the RST content
rst_content = "API Reference\n============\n\n"

for fqcn in sorted(public_classes.keys()):
    rst_content += f".. autoclass:: {fqcn}\n"
    rst_content += "   :members:\n"
    rst_content += "   :undoc-members:\n"
    rst_content += "   :show-inheritance:\n\n"

# Write the RST content to the output file
with open(output_file, 'w') as f:
    f.write(rst_content)

print(f"API documentation has been generated in {output_file}")
