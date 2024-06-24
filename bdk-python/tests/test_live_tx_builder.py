import bdkpython as bdk
import unittest
import os

SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

descriptor: bdk.Descriptor = bdk.Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
    bdk.Network.TESTNET
)
change_descriptor: bdk.Descriptor = bdk.Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
    bdk.Network.TESTNET
)

class LiveTxBuilderTest(unittest.TestCase):

    def tearDown(self) -> None:
        if os.path.exists("./bdk_persistence.sqlite"):
            os.remove("./bdk_persistence.sqlite")

    def test_tx_builder(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            bdk.Network.SIGNET
        )
        wallet: bdk.Wallet = bdk.Wallet(
            descriptor,
            change_descriptor,
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
        
        self.assertGreater(
            wallet.balance().total.to_sat(),
            0,
            f"Wallet balance must be greater than 0! Please send funds to {wallet.reveal_next_address(bdk.KeychainKind.EXTERNAL).address} and try again."
        )
        
        recipient = bdk.Address(
            address="tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989",
            network=bdk.Network.SIGNET
        )
        
        psbt = bdk.TxBuilder().add_recipient(script=recipient.script_pubkey(), amount=bdk.Amount.from_sat(4200)).fee_rate(fee_rate=bdk.FeeRate.from_sat_per_vb(2)).finish(wallet)
        
        self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")

    def complex_tx_builder(self):
        wallet: bdk.Wallet = bdk.Wallet(
            descriptor,
            change_descriptor,
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
        
        self.assertGreater(
            wallet.balance().total.to_sat(),
            0,
            f"Wallet balance must be greater than 0! Please send funds to {wallet.reveal_next_address(bdk.KeychainKind.EXTERNAL).address} and try again."
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
