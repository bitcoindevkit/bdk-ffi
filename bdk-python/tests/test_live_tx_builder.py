from bdkpython import Descriptor
from bdkpython import KeychainKind
from bdkpython import Wallet
from bdkpython import EsploraClient
from bdkpython import ScriptAmount
from bdkpython import FullScanRequest
from bdkpython import Address
from bdkpython import Psbt
from bdkpython import TxBuilder
from bdkpython import Connection

import unittest
import os

SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

descriptor: Descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
    Network.TESTNET
)
change_descriptor: Descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
    Network.TESTNET
)

class LiveTxBuilderTest(unittest.TestCase):

    def tearDown(self) -> None:
        if os.path.exists("./bdk_persistence.sqlite"):
            os.remove("./bdk_persistence.sqlite")

    def test_tx_builder(self):
        descriptor: Descriptor = Descriptor(
            "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            Network.SIGNET
        )
        connection: Connection = Connection.new_in_memory()
        wallet: Wallet = Wallet(
            descriptor,
            change_descriptor,
            Network.SIGNET,
            connection
        )
        esplora_client: EsploraClient = EsploraClient(url = SIGNET_ESPLORA_URL)
        full_scan_request: FullScanRequest = wallet.start_full_scan().build()
        update = esplora_client.full_scan(
            request=full_scan_request,
            stop_gap=10,
            parallel_requests=1
        )
        wallet.apply_update(update)
        
        self.assertGreater(
            wallet.balance().total.to_sat(),
            0,
            f"Wallet balance must be greater than 0! Please send funds to {wallet.reveal_next_address(KeychainKind.EXTERNAL).address} and try again."
        )
        
        recipient = Address(
            address="tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989",
            network=Network.SIGNET
        )
        
        psbt = TxBuilder().add_recipient(script=recipient.script_pubkey(), amount=Amount.from_sat(4200)).fee_rate(fee_rate=FeeRate.from_sat_per_vb(2)).finish(wallet)
        
        self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")

    def complex_tx_builder(self):
        connection: Connection = Connection.new_in_memory()
        wallet: Wallet = Wallet(
            descriptor,
            change_descriptor,
            Network.SIGNET,
            connection
        )
        esplora_client: EsploraClient = EsploraClient(url = SIGNET_ESPLORA_URL)
        full_scan_request: FullScanRequest = wallet.start_full_scan().build()
        update = esplora_client.full_scan(
            request=full_scan_request,
            stop_gap=10,
            parallel_requests=1
        )
        wallet.apply_update(update)
        
        self.assertGreater(
            wallet.balance().total.to_sat(),
            0,
            f"Wallet balance must be greater than 0! Please send funds to {wallet.reveal_next_address(KeychainKind.EXTERNAL).address} and try again."
        )
        
        recipient1 = Address(
            address="tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989",
            network=Network.SIGNET
        )
        recipient2 = Address(
            address="tb1qw2c3lxufxqe2x9s4rdzh65tpf4d7fssjgh8nv6",
            network=Network.SIGNET
        )
        all_recipients = list(
            ScriptAmount(recipient1.script_pubkey, 4200),
            ScriptAmount(recipient2.script_pubkey, 4200)
        )
        
        psbt: Psbt = TxBuilder().set_recipients(all_recipients).fee_rate(fee_rate=FeeRate.from_sat_per_vb(2)).finish(wallet)
        wallet.sign(psbt)
        
        self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")


if __name__ == '__main__':
    unittest.main()