from bdkpython import Persister, Network, Descriptor, KeychainKind, CbfBuilder, CbfComponents, CbfClient, CbfNode, CbfError, IpAddress, ScanType, Peer, Update, Wallet
import unittest
import os
import asyncio

network: Network = Network.SIGNET

ip: IpAddress = IpAddress.from_ipv4(68, 47, 229, 218)
peer: Peer = Peer(address=ip, port=None, v2_transport=False)

descriptor: Descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)",
    network=network
)
change_descriptor: Descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)",
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
        persister: Persister = Persister.new_in_memory()
        wallet: Wallet = Wallet(
            descriptor,
            change_descriptor,
            network,
            persister
        )
        peers = [peer]
        light_client: CbfComponents = CbfBuilder().scan_type(ScanType.NEW()).peers(peers).connections(1).build(wallet)
        client: CbfClient = light_client.client
        node: CbfNode = light_client.node
        async def log_loop(client: CbfClient):
            while True:
                log = await client.next_log()
                print(log)
        log_task = asyncio.create_task(log_loop(client))
        node.run()
        try:
            update: Update = await client.update()
            wallet.apply_update(update)
            self.assertGreater(
                wallet.balance().total.to_sat(),
                0,
                f"Wallet balance must be greater than 0! Please send funds to {wallet.reveal_next_address(KeychainKind.EXTERNAL).address} and try again."
            )
            log_task.cancel()
            client.shutdown()
        except CbfError as e:
            raise e

if __name__ == "__main__":
    unittest.main()
