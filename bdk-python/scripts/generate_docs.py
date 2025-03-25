import os
import re

# Define the directory where the Python source files are located
src_dir = 'src/bdkpython'

# Define the output file for the API documentation
output_file = 'docs/api.rst'

# Define categories and corresponding classes
categories = {
    "Core Types": ["Amount", "FeeRate", "Address"],
    "Wallet Operations": ["TxBuilder", "BumpFeeTxBuilder", "Psbt", "Blockchain", "ElectrumClient", "EsploraClient", "Wallet"],
    "Utilities": ["Mnemonic", "Descriptor", "DescriptorSecretKey", "DescriptorPublicKey", "ChangeSet"],
    "Exceptions": ["InternalError", "FeeRateError"]
}

# Regex pattern to match class definitions
class_pattern = re.compile(r'^class ([A-Za-z][A-Za-z0-9_]*)')

# Scan the source directory for Python files and extract public classes
public_classes = {}
for root, _, files in os.walk(src_dir):
    for file in files:
        if file.endswith('.py'):
            with open(os.path.join(root, file), 'r') as f:
                for line in f:
                    match = class_pattern.match(line)
                    if match:
                        class_name = match.group(1)
                        # Only consider classes not starting with underscore
                        if not class_name.startswith('_'):
                            public_classes[class_name] = root

# Generate the RST content
rst_content = "API Reference\n============\n\n.. currentmodule:: bdkpython\n\n"

for category, class_list in categories.items():
    rst_content += f"{category}\n{'-' * len(category)}\n\n"
    for class_name in class_list:
        if class_name in public_classes:
            rst_content += f".. autoclass:: {class_name}\n   :members:\n   :undoc-members:\n   :show-inheritance:\n\n"

# Write the RST content to the output file
with open(output_file, 'w') as f:
    f.write(rst_content)

print(f"API documentation has been generated in {output_file}")