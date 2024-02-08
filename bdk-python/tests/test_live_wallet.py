import bdkpython as bdk
import unittest
import os

class LiveWalletTest(unittest.TestCase):

    def tearDown(self) -> None:
        if os.path.exists("./bdk_persistence.db"):
            os.remove("./bdk_persistence.db")

    def test_synced_balance(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            bdk.Network.TESTNET
        )
        wallet: bdk.Wallet = bdk.Wallet(
            descriptor,
            None,
            "./bdk_persistence.db",
            bdk.Network.TESTNET
        )
        esploraClient: bdk.EsploraClient = bdk.EsploraClient(url = "https://esplora.testnet.kuutamo.cloud/")
        update = esploraClient.full_scan(
            wallet = wallet,
            stop_gap = 10,
            parallel_requests = 1
        )
        wallet.apply_update(update)
        
        self.assertGreater(wallet.get_balance().total, 0)
        
        print(f"Transactions count: {len(wallet.transactions())}")
        transactions = wallet.transactions()[:3]
        for tx in transactions:
            sent_and_received = wallet.sent_and_received(tx)
            print(f"Transaction: {tx.txid()}")
            print(f"Sent {sent_and_received.sent}")
            print(f"Received {sent_and_received.received}")


    def test_broadcast_transaction(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            bdk.Network.TESTNET
        )
        wallet: bdk.Wallet = bdk.Wallet(
            descriptor,
            None,
            "./bdk_persistence.db",
            bdk.Network.TESTNET
        )
        esploraClient: bdk.EsploraClient = bdk.EsploraClient(url = "https://esplora.testnet.kuutamo.cloud/")
        update = esploraClient.full_scan(
            wallet = wallet,
            stop_gap = 10,
            parallel_requests = 1
        )
        wallet.apply_update(update)
        
        self.assertGreater(wallet.get_balance().total, 0)
        
        recipient = bdk.Address(
            address = "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989",
            network = bdk.Network.TESTNET
        )
        
        psbt = bdk.TxBuilder().add_recipient(script=recipient.script_pubkey(), amount=4200).fee_rate(fee_rate=bdk.FeeRate.from_sat_per_vb(2.0)).finish(wallet)
        # print(psbt.serialize())
        self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")
        
        walletDidSign = wallet.sign(psbt)
        self.assertTrue(walletDidSign)
        tx = psbt.extract_tx()
        print(f"Transaction Id: {tx.txid()}")
        fee = wallet.calculate_fee(tx)
        print(f"Transaction Fee: {fee}")
        fee_rate = wallet.calculate_fee_rate(tx)
        print(f"Transaction Fee Rate: {fee_rate.as_sat_per_vb()} sat/vB")
        
        esploraClient.broadcast(tx)
    
    
if __name__ == '__main__':
    unittest.main()
