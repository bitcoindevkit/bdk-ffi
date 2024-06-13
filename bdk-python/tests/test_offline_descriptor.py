import bdkpython as bdk
import unittest

class OfflineDescriptorTest(unittest.TestCase):

    def test_descriptor_bip86(self):
        mnemonic: bdk.Mnemonic = bdk.Mnemonic.from_string("space echo position wrist orient erupt relief museum myself grain wisdom tumble")
        descriptor_secret_key: bdk.DescriptorSecretKey = bdk.DescriptorSecretKey(bdk.Network.TESTNET, mnemonic, None)
        descriptor: bdk.Descriptor = bdk.Descriptor.new_bip86(descriptor_secret_key, bdk.KeychainKind.EXTERNAL, bdk.Network.TESTNET)

        self.assertEqual(
            "tr([be1eec8f/86'/1'/0']tpubDCTtszwSxPx3tATqDrsSyqScPNnUChwQAVAkanuDUCJQESGBbkt68nXXKRDifYSDbeMa2Xg2euKbXaU3YphvGWftDE7ozRKPriT6vAo3xsc/0/*)#m7puekcx",
            descriptor.__str__()
        )


if __name__ == '__main__':
    unittest.main()
