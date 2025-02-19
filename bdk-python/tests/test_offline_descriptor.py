from bdkpython import Descriptor
from bdkpython import Mnemonic
from bdkpython import DescriptorSecretKey
from bdkpython import KeychainKind
from bdkpython import Network

import unittest

class OfflineDescriptorTest(unittest.TestCase):

    def test_descriptor_bip86(self):
        mnemonic: Mnemonic = Mnemonic.from_string("space echo position wrist orient erupt relief museum myself grain wisdom tumble")
        descriptor_secret_key: DescriptorSecretKey = DescriptorSecretKey(Network.TESTNET, mnemonic, None)
        descriptor: Descriptor = Descriptor.new_bip86(descriptor_secret_key, KeychainKind.EXTERNAL, Network.TESTNET)

        self.assertEqual(
            "tr([be1eec8f/86'/1'/0']tpubDCTtszwSxPx3tATqDrsSyqScPNnUChwQAVAkanuDUCJQESGBbkt68nXXKRDifYSDbeMa2Xg2euKbXaU3YphvGWftDE7ozRKPriT6vAo3xsc/0/*)#m7puekcx",
            descriptor.__str__()
        )


if __name__ == '__main__':
    unittest.main()
