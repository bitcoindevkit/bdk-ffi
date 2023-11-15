import bdkpython as bdk
import unittest

class TestLiveWallet(unittest.TestCase):

    def test_synced_balance(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            bdk.Network.TESTNET
        )
        wallet: bdk.Wallet = bdk.Wallet.new_no_persist(
            descriptor,
            None,
            bdk.Network.TESTNET
        )
        esploraClient: bdk.EsploraClient = bdk.EsploraClient(url = "https://mempool.space/testnet/api")
        update = esploraClient.scan(
            wallet = wallet,
            stop_gap = 10,
            parallel_requests = 1
        )
        wallet.apply_update(update)

        self.assertGreater(wallet.get_balance().total, 0)

    def test_broadcast_transaction(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            bdk.Network.TESTNET
        )
        wallet: bdk.Wallet = bdk.Wallet.new_no_persist(
            descriptor,
            None,
            bdk.Network.TESTNET
        )
        esploraClient: bdk.EsploraClient = bdk.EsploraClient(url = "https://mempool.space/testnet/api")
        update = esploraClient.scan(
            wallet = wallet,
            stop_gap = 10,
            parallel_requests = 1
        )
        wallet.apply_update(update)

        self.assertGreater(wallet.get_balance().total(), 0)

        recipient = bdk.Address(
            address = "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989",
            network = bdk.Network.TESTNET
        )

        psbt = bdk.TxBuilder().add_recipient(script=recipient.script_pubkey(), amount=4200).fee_rate(2.0).finish(wallet)
        # print(psbt.serialize())
        self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")

        walletDidSign = wallet.sign(psbt)
        self.assertTrue(walletDidSign)
        tx = psbt.extract_tx()
        
        esploraClient.broadcast(tx)
    
    
if __name__ == '__main__':
    unittest.main()
