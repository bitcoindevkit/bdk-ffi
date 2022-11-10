import unittest
from bdk import *

class TestBdk(unittest.TestCase):

    def test_some_enum(self):
        network = Network.TESTNET

    def test_some_dict(self):
        a = AddressInfo(index=42, address="testaddress")
        self.assertEqual(42, a.index)
        self.assertEqual("testaddress", a.address)

if __name__=='__main__':
    unittest.main()
