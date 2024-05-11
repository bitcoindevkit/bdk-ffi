import bdkpython as bdk
import unittest
import os

SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

class LiveTxBuilderTest(unittest.TestCase):

    def tearDown(self) -> None:
        if os.path.exists("./bdk_persistence.db"):
            os.remove("./bdk_persistence.db")

    def test_tx_builder(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            bdk.Network.SIGNET
        )
        wallet: bdk.Wallet = bdk.Wallet(
            descriptor,
            None,
            "./bdk_persistence.db",
            bdk.Network.SIGNET
        )
        esplora_client: bdk.EsploraClient = bdk.EsploraClient(url = SIGNET_ESPLORA_URL)
        full_scan_request: bdk.FullScanRequest = wallet.start_full_scan()
        update = esplora_client.full_scan(
            full_scan_request=full_scan_request,
            stop_gap=10,
            parallel_requests=1
        )
        wallet.apply_update(update)
        wallet.commit()
        
        self.assertGreater(
            wallet.get_balance().total.to_sat(),
            0,
            f"Wallet balance must be greater than 0! Please send funds to {wallet.reveal_next_address(bdk.KeychainKind.EXTERNAL).address.as_string()} and try again."
        )
        
        recipient = bdk.Address(
            address="tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989",
            network=bdk.Network.SIGNET
        )
        
        psbt = bdk.TxBuilder().add_recipient(script=recipient.script_pubkey(), amount=4200).fee_rate(fee_rate=bdk.FeeRate.from_sat_per_vb(2)).finish(wallet)
        
        self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")

    def complex_tx_builder(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            bdk.Network.SIGNET
        )
        change_descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)",
            bdk.Network.SIGNET
        )
        wallet: bdk.Wallet = bdk.Wallet(
            descriptor,
            change_descriptor,
            "./bdk_persistence.db",
            bdk.Network.SIGNET
        )
        esplora_client: bdk.EsploraClient = bdk.EsploraClient(url = SIGNET_ESPLORA_URL)
        full_scan_request: bdk.FullScanRequest = wallet.start_full_scan()
        update = esplora_client.full_scan(
            full_scan_request=full_scan_request,
            stop_gap=10,
            parallel_requests=1
        )
        wallet.apply_update(update)
        wallet.commit()
        
        self.assertGreater(
            wallet.get_balance().total.to_sat(),
            0,
            f"Wallet balance must be greater than 0! Please send funds to {wallet.reveal_next_address(bdk.KeychainKind.EXTERNAL).address.as_string()} and try again."
        )
        
        recipient1 = bdk.Address(
            address="tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989",
            network=bdk.Network.SIGNET
        )
        recipient2 = bdk.Address(
            address="tb1qw2c3lxufxqe2x9s4rdzh65tpf4d7fssjgh8nv6",
            network=bdk.Network.SIGNET
        )
        all_recipients = list(
            bdk.ScriptAmount(recipient1.script_pubkey, 4200),
            bdk.ScriptAmount(recipient2.script_pubkey, 4200)
        )
        
        psbt: bdk.Psbt = bdk.TxBuilder().set_recipients(all_recipients).fee_rate(fee_rate=bdk.FeeRate.from_sat_per_vb(2)).enable_rbf().finish(wallet)
        wallet.sign(psbt)
        
        self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")


if __name__ == '__main__':
    unittest.main()
