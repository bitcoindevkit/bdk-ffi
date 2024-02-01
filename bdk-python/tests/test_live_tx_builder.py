import bdkpython as bdk
import unittest

class TestLiveTxBuilder(unittest.TestCase):

    def test_tx_builder(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            bdk.Network.TESTNET
        )
        # wallet: bdk.Wallet = bdk.Wallet.new_no_persist(
        #     descriptor,
        #     None,
        #     bdk.Network.TESTNET
        # )
        # esploraClient: bdk.EsploraClient = bdk.EsploraClient(url = "https://mempool.space/testnet/api")
        # update = esploraClient.full_scan(
        #     wallet = wallet,
        #     stop_gap = 10,
        #     parallel_requests = 1
        # )
        # wallet.apply_update(update)
        #
        # self.assertGreater(wallet.get_balance().total, 0)
        #
        # recipient = bdk.Address(
        #     address = "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989",
        #     network = bdk.Network.TESTNET
        # )
        #
        # psbt = bdk.TxBuilder().add_recipient(script=recipient.script_pubkey(), amount=4200).fee_rate(fee_rate=bdk.FeeRate.from_sat_per_vb(2.0)).finish(wallet)
        # # print(psbt.serialize())
        #
        # self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")

    def complex_tx_builder(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            bdk.Network.TESTNET
        )
        change_descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)",
            bdk.Network.TESTNET
        )
        # wallet: bdk.Wallet = bdk.Wallet.new_no_persist(
        #     descriptor,
        #     change_descriptor,
        #     bdk.Network.TESTNET
        # )
        # esploraClient: bdk.EsploraClient = bdk.EsploraClient(url = "https://mempool.space/testnet/api")
        # update = esploraClient.full_scan(
        #     wallet = wallet,
        #     stop_gap = 10,
        #     parallel_requests = 1
        # )
        # wallet.apply_update(update)
        #
        # self.assertGreater(wallet.get_balance().total, 0)
        #
        # recipient1 = bdk.Address(
        #     address = "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989",
        #     network = bdk.Network.TESTNET
        # )
        # recipient2 = bdk.Address(
        #     address = "tb1qw2c3lxufxqe2x9s4rdzh65tpf4d7fssjgh8nv6",
        #     network = bdk.Network.TESTNET
        # )
        # all_recipients = list(
        #     bdk.ScriptAmount(recipient1.script_pubkey, 4200),
        #     bdk.ScriptAmount(recipient2.script_pubkey, 4200)
        # )
        #
        # psbt: bdk.PartiallySignedTransaction = bdk.TxBuilder().add_recipient(script=recipient.script_pubkey(), amount=4200).fee_rate(fee_rate=bdk.FeeRate.from_sat_per_vb(2.0)).finish(wallet)
        # wallet.sign(psbt)
        #
        # self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")


if __name__ == '__main__':
    unittest.main()
