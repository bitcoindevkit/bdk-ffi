from bdkpython import *
from bdkpython.bitcoin import *
import unittest
import os

network: Network = Network.SIGNET

ip: IpAddress = IpAddress.from_ipv4(68, 47, 229, 218)
peer: Peer = Peer(address=ip, port=None, v2_transport=False)

descriptor: Descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
    network=network
)
change_descriptor: Descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
    network=network
)

class LiveKyotoTest(unittest.IsolatedAsyncioTestCase):

    def tearDown(self) -> None:
        if os.path.exists("./bdk_persistence.sqlite"):
            os.remove("./bdk_persistence.sqlite")
        if os.path.exists("./data/signet/headers.db"):
            os.remove("./data/signet/headers.db")
        if os.path.exists("./data/signet/peers.db"):
            os.remove("./data/signet/peers.db")

    async def testKyoto(self) -> None:
        connection: Connection = Connection.new_in_memory()
        wallet: Wallet = Wallet(
            descriptor,
            change_descriptor,
            network,
            connection
        )
        peers = [peer]
        client: LightClient = LightClientBuilder().peers(peers).connections(1).build(wallet)
        publisher: EventPublisher = client.event_publisher
        subsciber: EventSubscriber = client.event_subscriber
        node: LightNode = client.node
        node.run()
        update: Update = await subsciber.update(None)
        self.assertIsNotNone(update, "Update is None. This should not be possible.")
        wallet.apply_update(update)
        self.assertGreater(
            wallet.balance().total.to_sat(),
            0,
            f"Wallet balance must be greater than 0! Please send funds to {wallet.reveal_next_address(KeychainKind.EXTERNAL).address} and try again."
        )
        await publisher.shutdown()

if __name__ == "__main__":
    unittest.main()