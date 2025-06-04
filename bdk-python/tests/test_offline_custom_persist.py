
import binascii
import json
import unittest
from typing import Any, Dict, List, Optional

import bdkpython as bdk

initial_txs = [
    "0200000000010101d7eb881ab8cac7d6adc6a7f9aa13e694813d95330c7299cee3623e5d14bd590000000000fdffffff02c5e6c1010000000016001407103a1cccf6a1ea654bee964a4020d20c41fb055c819725010000001600146337ec04bf42015e5d077b90cae05c06925c491a0247304402206aae2bf32da4c3b71cb95e6633c22f9f5a4a4f459975965c0c39b0ab439737b702200c4b16d2029383190965b07adeb87e6d634c68c70d2742f25e456874e8dc273a012103930326d6d72f8663340ce4341d0d3bdb1a1c0734d46e5df8a3003ab6bb50073b00000000",
    "02000000000101b0db431cffebeeeeec19ee8a09a2ae4755722ea73232dbb99b8e754eaad6ac300100000000fdffffff024ad24201000000001600146a7b71a68b261b0b7c79e5bb00f0f3d65d5ae4a285ae542401000000160014e43ff61232ca20061ef1d241e73f322a149a23d902473044022059f4b2fa8b9da34dbb57e491f3d5b8a47a623d7e6ebc1b6adfe6d2be744c9640022073cfc8311c49a8d48d69076466d32be591d3c0092b965828cfbcaca69fd409c90121027aa62d03db46272fa31bc1a6cb095bb66bc5409dd74b25e88e3099d84a17a3e469000000",
]
descriptor: bdk.Descriptor = bdk.Descriptor(
    "wpkh([44250c36/84'/1'/0']tpubDCrUjjHLB1fxk1oRveETjw62z8jsUuqx7JkBUW44VBszGmcY3Eun3apwVcE5X2bfF5MsM3uvuQDed6Do33ZN8GiWcnj2QPqVDspFT1AyZJ9/0/*)",
    bdk.Network.REGTEST,
)
change_descriptor: bdk.Descriptor = bdk.Descriptor(
    "wpkh([44250c36/84'/1'/0']tpubDCrUjjHLB1fxk1oRveETjw62z8jsUuqx7JkBUW44VBszGmcY3Eun3apwVcE5X2bfF5MsM3uvuQDed6Do33ZN8GiWcnj2QPqVDspFT1AyZJ9/1/*)",
    bdk.Network.REGTEST,
)


serialized_persistence = """{"descriptor": "wpkh([44250c36/84'/1'/0']tpubDCrUjjHLB1fxk1oRveETjw62z8jsUuqx7JkBUW44VBszGmcY3Eun3apwVcE5X2bfF5MsM3uvuQDed6Do33ZN8GiWcnj2QPqVDspFT1AyZJ9/0/*)#9q4e992d", "change_descriptor": "wpkh([44250c36/84'/1'/0']tpubDCrUjjHLB1fxk1oRveETjw62z8jsUuqx7JkBUW44VBszGmcY3Eun3apwVcE5X2bfF5MsM3uvuQDed6Do33ZN8GiWcnj2QPqVDspFT1AyZJ9/1/*)#55sccs64", "network": "REGTEST", "local_chain": {"changes": [{"height": 0, "hash": "06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f"}]}, "tx_graph": {"txs": ["0200000000010101d7eb881ab8cac7d6adc6a7f9aa13e694813d95330c7299cee3623e5d14bd590000000000fdffffff02c5e6c1010000000016001407103a1cccf6a1ea654bee964a4020d20c41fb055c819725010000001600146337ec04bf42015e5d077b90cae05c06925c491a0247304402206aae2bf32da4c3b71cb95e6633c22f9f5a4a4f459975965c0c39b0ab439737b702200c4b16d2029383190965b07adeb87e6d634c68c70d2742f25e456874e8dc273a012103930326d6d72f8663340ce4341d0d3bdb1a1c0734d46e5df8a3003ab6bb50073b00000000", "02000000000101b0db431cffebeeeeec19ee8a09a2ae4755722ea73232dbb99b8e754eaad6ac300100000000fdffffff024ad24201000000001600146a7b71a68b261b0b7c79e5bb00f0f3d65d5ae4a285ae542401000000160014e43ff61232ca20061ef1d241e73f322a149a23d902473044022059f4b2fa8b9da34dbb57e491f3d5b8a47a623d7e6ebc1b6adfe6d2be744c9640022073cfc8311c49a8d48d69076466d32be591d3c0092b965828cfbcaca69fd409c90121027aa62d03db46272fa31bc1a6cb095bb66bc5409dd74b25e88e3099d84a17a3e469000000"], "txouts": {}, "anchors": [], "last_seen": {"2d2f7cedc21b4272bf57e3eaaeec241959d15bfa7b710ae984ec1ef2b804c1c0": 0, "b0db431cffebeeeeec19ee8a09a2ae4755722ea73232dbb99b8e754eaad6ac30": 0}}, "indexer": {"last_revealed": {"d29ab90c8fe23b5f43f94462e9128ae15368e83d628a466108d64a08c4abd41f": 8}}}"""


class ChangeSetConverter:
    @staticmethod
    def to_dict(changeset: bdk.ChangeSet) -> Dict:
        """
        Serialize a bdk.ChangeSet into a JSON string.
        """

        def _serialize_descriptor(descriptor: Optional[bdk.Descriptor]) -> Optional[str]:
            if descriptor is None:
                return None
            return str(descriptor)

        def _serialize_blockhash(bh: Optional[bdk.BlockHash]) -> Optional[str]:
            if bh is None:
                return None
            return bh.serialize().hex()

        def _serialize_chainchange(cc: bdk.ChainChange) -> Dict[str, Any]:
            return {"height": cc.height, "hash": _serialize_blockhash(cc.hash)}

        def _serialize_local_chain(local_chain: bdk.LocalChainChangeSet) -> Dict[str, Any]:
            return {"changes": [_serialize_chainchange(cc) for cc in local_chain.changes]}

        def _serialize_tx(tx: bdk.Transaction) -> str:
            return tx.serialize().hex()

        def _serialize_outpoint(hop: bdk.HashableOutPoint) -> Dict[str, Any]:
            op = hop.outpoint()
            txid_hex = op.txid.serialize().hex()
            return {"txid": txid_hex, "vout": op.vout}

        def _serialize_txout(txout: bdk.TxOut) -> Dict[str, Any]:
            # TxOut.script_pubkey is a bdk.Script instance
            script_obj: bdk.Script = txout.script_pubkey
            script_bytes = script_obj.to_bytes()
            return {"value": txout.value, "script_pubkey": script_bytes.hex()}

        def _serialize_tx_graph(tx_graph: bdk.TxGraphChangeSet) -> Dict[str, Any]:
            txs_list = [_serialize_tx(tx) for tx in tx_graph.txs]

            txouts_dict: Dict[str, Dict[str, Any]] = {}
            for hop, txout in sorted(tx_graph.txouts.items()):
                key = json.dumps(_serialize_outpoint(hop))
                txouts_dict[key] = _serialize_txout(txout)

            anchors_list: List[Dict[str, Any]] = []
            for anchor in tx_graph.anchors:
                cbt = anchor.confirmation_block_time
                # Serialize BlockId inside ConfirmationBlockTime
                block_id = cbt.block_id
                bh_hex = _serialize_blockhash(block_id.hash)
                block_id_obj = {"height": block_id.height, "hash": bh_hex}
                cbt_obj = {"block_id": block_id_obj, "confirmation_time": cbt.confirmation_time}
                try:
                    txid_hex = anchor.txid.serialize().hex()
                except AttributeError:
                    txid_hex = str(anchor.txid)
                anchors_list.append({"confirmation_block_time": cbt_obj, "txid": txid_hex})

            def sort_key(t):
                txid_obj , height =t
                return  txid_obj.serialize().hex()

            last_seen_dict: Dict[str, int] = {}
            for txid_obj, height in sorted(tx_graph.last_seen.items(), key=sort_key):
                try:
                    txid_hex = txid_obj.serialize().hex()
                except AttributeError:
                    txid_hex = str(txid_obj)
                last_seen_dict[txid_hex] = height

            return {
                "txs": txs_list,
                "txouts": txouts_dict,
                "anchors": anchors_list,
                "last_seen": last_seen_dict,
            }

        def _serialize_indexer(indexer: bdk.IndexerChangeSet) -> Dict[str, Any]:
            lr: Dict[str, int] = {}
            for did_obj, idx in sorted(indexer.last_revealed.items()):
                did_hex = did_obj.serialize().hex()
                lr[did_hex] = idx
            return {"last_revealed": lr}

        out: Dict[str, Any] = {}
        out["descriptor"] = _serialize_descriptor(changeset.descriptor())
        out["change_descriptor"] = _serialize_descriptor(changeset.change_descriptor())

        network = changeset.network()
        if network is None:
            out["network"] = None
        else:
            out["network"] = network.name

        out["local_chain"] = _serialize_local_chain(changeset.localchain_changeset())
        out["tx_graph"] = _serialize_tx_graph(changeset.tx_graph_changeset())
        out["indexer"] = _serialize_indexer(changeset.indexer_changeset())

        return out

    @staticmethod
    def from_dict(parsed_json: Dict) -> bdk.ChangeSet:
        """
        Deserialize a JSON string back into a bdk.ChangeSet.
        """

        def _deserialize_descriptor(
            descriptor_str: Optional[str], network: Optional[bdk.Network]
        ) -> Optional[bdk.Descriptor]:
            if descriptor_str is None:
                return None
            return bdk.Descriptor(descriptor_str, network)

        def _deserialize_blockhash(hexstr: Optional[str]) -> Optional[bdk.BlockHash]:
            if hexstr is None:
                return None
            raw = binascii.unhexlify(hexstr)
            return bdk.BlockHash.from_bytes(raw)

        def _deserialize_chainchange(data: Dict[str, Any]) -> bdk.ChainChange:
            height = data["height"]
            hash_hex = data["hash"]
            bh = _deserialize_blockhash(hash_hex)
            return bdk.ChainChange(height=height, hash=bh)

        def _deserialize_local_chain(data: Dict[str, Any]) -> bdk.LocalChainChangeSet:
            changes_list = data.get("changes", [])
            cc_objs: List[bdk.ChainChange] = [_deserialize_chainchange(cc) for cc in changes_list]
            return bdk.LocalChainChangeSet(changes=cc_objs)

        def _deserialize_tx(hexstr: str) -> bdk.Transaction:
            raw = binascii.unhexlify(hexstr)
            return bdk.Transaction(raw)

        def _deserialize_outpoint(key_str: str) -> bdk.HashableOutPoint:
            obj = json.loads(key_str)
            txid_hex = obj["txid"]
            vout = obj["vout"]
            try:
                txid_bytes = binascii.unhexlify(txid_hex)
                txid_obj = bdk.Txid.from_bytes(txid_bytes)
            except Exception:
                txid_obj = bdk.Txid(txid_hex)
            outpoint = bdk.OutPoint(txid=txid_obj, vout=vout)
            return bdk.HashableOutPoint(outpoint=outpoint)

        def _deserialize_txout(data: Dict[str, Any]) -> bdk.TxOut:
            value = data["value"]
            script_hex = data["script_pubkey"]
            script_bytes = binascii.unhexlify(script_hex)
            script_obj = bdk.Script(raw_output_script=script_bytes)
            return bdk.TxOut(value=value, script_pubkey=script_obj)

        def _deserialize_tx_graph(data: Dict[str, Any]) -> bdk.TxGraphChangeSet:
            tx_hex_list = data.get("txs", [])
            tx_objs: List[bdk.Transaction] = [_deserialize_tx(h) for h in tx_hex_list]

            txouts_data = data.get("txouts", {})
            txouts_dict: Dict[bdk.HashableOutPoint, bdk.TxOut] = {}
            for key_str, txout_data in sorted(txouts_data.items()):
                hop = _deserialize_outpoint(key_str)
                txout_obj = _deserialize_txout(txout_data)
                txouts_dict[hop] = txout_obj

            anchors_list: List[bdk.Anchor] = []
            for anc in data.get("anchors", []):
                cbt_data = anc["confirmation_block_time"]
                block_id_data = cbt_data["block_id"]
                height = block_id_data["height"]
                hash_hex = block_id_data["hash"]
                bh = _deserialize_blockhash(hash_hex)
                block_id_obj = bdk.BlockId(height=height, hash=bh)

                confirmation_time = cbt_data["confirmation_time"]
                cbt_obj = bdk.ConfirmationBlockTime(
                    block_id=block_id_obj, confirmation_time=confirmation_time
                )

                txid_hex = anc["txid"]
                try:
                    txid_bytes = binascii.unhexlify(txid_hex)
                    txid_obj = bdk.Txid.from_bytes(txid_bytes)
                except Exception:
                    txid_obj = bdk.Txid(txid_hex)

                anchors_list.append(bdk.Anchor(confirmation_block_time=cbt_obj, txid=txid_obj))

            last_seen_data = data.get("last_seen", {})
            last_seen_dict: Dict[bdk.Txid, int] = {}
            for txid_hex, height in sorted(last_seen_data.items()):
                try:
                    txid_obj = bdk.Txid.from_bytes(binascii.unhexlify(txid_hex))
                except Exception:
                    txid_obj = bdk.Txid(txid_hex)
                last_seen_dict[txid_obj] = height

            return bdk.TxGraphChangeSet(
                txs=tx_objs, txouts=txouts_dict, anchors=anchors_list, last_seen=last_seen_dict
            )

        def _deserialize_indexer(data: Dict[str, Any]) -> bdk.IndexerChangeSet:
            lr_data = data.get("last_revealed", {})
            lr_dict: Dict[bdk.DescriptorId, int] = {}
            for did_hex, idx in sorted(lr_data.items()):
                did_bytes = binascii.unhexlify(did_hex)
                did_obj = bdk.DescriptorId.from_bytes(did_bytes)
                lr_dict[did_obj] = idx
            return bdk.IndexerChangeSet(last_revealed=lr_dict)

        net = parsed_json.get("network")
        if net is None:
            network_obj = None
        else:
            network_obj = getattr(bdk.Network, net)

        descr = _deserialize_descriptor(parsed_json.get("descriptor"), network_obj)
        change_descr = _deserialize_descriptor(parsed_json.get("change_descriptor"), network_obj)
        local_chain_obj = _deserialize_local_chain(parsed_json["local_chain"])
        tx_graph_obj = _deserialize_tx_graph(parsed_json["tx_graph"])
        indexer_obj = _deserialize_indexer(parsed_json["indexer"])

        changeset = bdk.ChangeSet.from_descriptor_and_network(
            descriptor=descr,
            change_descriptor=change_descr,
            network=network_obj,
        )
        changeset = bdk.ChangeSet.from_merge(
            changeset, bdk.ChangeSet.from_local_chain_changes(local_chain_changes=local_chain_obj)
        )
        changeset = bdk.ChangeSet.from_merge(
            changeset, bdk.ChangeSet.from_tx_graph_changeset(tx_graph_changeset=tx_graph_obj)
        )
        changeset = bdk.ChangeSet.from_merge(
            changeset, bdk.ChangeSet.from_indexer_changeset(indexer_changes=indexer_obj)
        )
        return changeset


class MyPersistence(bdk.Persistence):
    def __init__(self):
        self.memory = []

    def merge_all(self) -> bdk.ChangeSet:
        total_changeset = bdk.ChangeSet()
        for changeset_dict in self.memory:
            total_changeset = bdk.ChangeSet.from_merge(total_changeset, changeset_dict)
        return total_changeset

    def initialize(self) -> bdk.ChangeSet:
        return self.merge_all()

    def persist(self, changeset: bdk.ChangeSet):
        self.memory.append(changeset)


class PersistenceTest(unittest.TestCase):

    def test_synced_transactions(self):

        myp = MyPersistence()
        persister = bdk.Persister.custom(myp)

        wallet: bdk.Wallet = bdk.Wallet(descriptor, change_descriptor, bdk.Network.REGTEST, persister)

        wallet.apply_unconfirmed_txs(
            [bdk.UnconfirmedTx(tx=bdk.Transaction(bytes.fromhex(tx)), last_seen=0) for tx in initial_txs]
        )

        wallet.persist(persister=persister)

        # initialize new wallet with memory of myp
        myp2 = MyPersistence()
        myp2.memory = [ChangeSetConverter.from_dict(json.loads(serialized_persistence))]
        persister2 = bdk.Persister.custom(myp2)

        wallet2 = bdk.Wallet.load(
            descriptor=descriptor,
            change_descriptor=change_descriptor,
            persister=persister2,
        )

        # check for equality
        outputs = wallet.list_output()
        outputs2 = wallet2.list_output()
        assert len(outputs) == len(outputs2)
        for o, o2 in zip(outputs, outputs2):
            assert o.outpoint.txid == o2.outpoint.txid
            assert o.outpoint.vout == o2.outpoint.vout

        txs = wallet.transactions()
        txs2 = wallet2.transactions()
        assert txs, "Sync error"
        assert len(txs) == len(txs2)
        for tx, tx2 in zip(txs, txs2):
            assert tx.transaction.compute_txid().serialize() == tx2.transaction.compute_txid().serialize()

        assert wallet.balance().total.to_sat() == 50641167
        d_myp = ChangeSetConverter.to_dict(myp.initialize())
        d_myp2 = ChangeSetConverter.to_dict(myp2.initialize())
        assert json.dumps(d_myp) == json.dumps(d_myp2)

if __name__ == "__main__":
    unittest.main()
