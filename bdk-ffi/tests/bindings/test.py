from bdkpython import BlockId
from bdkpython.bitcoin import Network

import unittest

class TestBdk(unittest.TestCase):

    # A type from the bitcoin-ffi library
    def test_some_enum(self):
        network = Network.TESTNET

    # A type from the bdk-ffi library
    def test_some_dict(self):
        block_id = BlockId(height=0, hash="abcd")

if __name__=='__main__':
    unittest.main()
