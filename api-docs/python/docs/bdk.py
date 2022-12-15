class Address(object):
    """
    A bitcoin address.

    Args:
        address: A bitcoin address
    """
    def __init__(self, address):
        address = address[str]

    """
    Return the ScriptPubKey.
    """
    def script_pubkey(self):
        return script[Script]


class Script(object):
    def __init__(self, raw_output_script):
        raw_output_script = list(int(x) for x in raw_output_script)