from bdkpython.bitcoin import Network

import unittest

class TestBdk(unittest.TestCase):

    # A type from the bitcoin-ffi library
    def test_some_enum(self):
        network = Network.TESTNET

if __name__=='__main__':
    unittest.main()
