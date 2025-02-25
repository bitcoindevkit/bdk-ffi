from bdkpython import Descriptor
from bdkpython import KeychainKind
from bdkpython import Wallet
from bdkpython import EsploraClient
from bdkpython import FullScanRequest
from bdkpython import Address
from bdkpython import Psbt
from bdkpython import TxBuilder
from bdkpython import Connection
from bdkpython import Network
from bdkpython import Amount
from bdkpython import FeeRate

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

class LiveWalletTest(unittest.TestCase):

    def tearDown(self) -> None:
        if os.path.exists("./bdk_persistence.sqlite"):
            os.remove("./bdk_persistence.sqlite")

    def test_synced_balance(self):
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
        
        print(f"Transactions count: {len(wallet.transactions())}")
        transactions = wallet.transactions()[:3]
        for tx in transactions:
            sent_and_received = wallet.sent_and_received(tx.transaction)
            print(f"Transaction: {tx.transaction.compute_txid()}")
            print(f"Sent {sent_and_received.sent.to_sat()}")
            print(f"Received {sent_and_received.received.to_sat()}")


    def test_broadcast_transaction(self):
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
        
        psbt: Psbt = TxBuilder().add_recipient(script=recipient.script_pubkey(), amount=Amount.from_sat(4200)).fee_rate(fee_rate=FeeRate.from_sat_per_vb(2)).finish(wallet)
        self.assertTrue(psbt.serialize().startswith("cHNi"), "The PSBT should start with cHNi")
        
        walletDidSign = wallet.sign(psbt)
        self.assertTrue(walletDidSign)
        tx = psbt.extract_tx()
        print(f"Transaction Id: {tx.compute_txid()}")
        fee = wallet.calculate_fee(tx)
        print(f"Transaction Fee: {fee.to_sat()}")
        fee_rate = wallet.calculate_fee_rate(tx)
        print(f"Transaction Fee Rate: {fee_rate.to_sat_per_vb_ceil()} sat/vB")
        
        esplora_client.broadcast(tx)
    
    
if __name__ == '__main__':
    unittest.main()