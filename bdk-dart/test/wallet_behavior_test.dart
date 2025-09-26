import 'package:bdk_dart/bdk.dart';
import 'package:test/test.dart';

import 'test_constants.dart';

Wallet _buildTestWallet() {
  final persister = Persister.newInMemory();
  return Wallet(
    buildBip84Descriptor(Network.testnet),
    buildBip84ChangeDescriptor(Network.testnet),
    Network.testnet,
    persister,
    defaultLookahead,
  );
}

void main() {
  group('Wallet behaviour', () {
    test('produces addresses valid for expected networks', () {
      final wallet = _buildTestWallet();
      final addressInfo = wallet.revealNextAddress(KeychainKind.external_);

      expect(addressInfo.address.isValidForNetwork(Network.testnet), isTrue);
      expect(addressInfo.address.isValidForNetwork(Network.testnet4), isTrue);
      expect(addressInfo.address.isValidForNetwork(Network.signet), isTrue);
      expect(addressInfo.address.isValidForNetwork(Network.regtest), isFalse);
      expect(addressInfo.address.isValidForNetwork(Network.bitcoin), isFalse);
    });

    test('starts with zero balance before sync', () {
      final wallet = _buildTestWallet();
      expect(wallet.balance().total.toSat(), equals(0));
    });

    test('single-descriptor wallet returns identical external/internal addresses', () {
      final persister = Persister.newInMemory();
      final wallet = Wallet.createSingle(
        buildBip84Descriptor(Network.testnet),
        Network.testnet,
        persister,
        defaultLookahead,
      );

      final externalAddress = wallet.peekAddress(KeychainKind.external_, 0);
      final internalAddress = wallet.peekAddress(KeychainKind.internal, 0);

      expect(
        externalAddress.address.scriptPubkey().toBytes(),
        orderedEquals(internalAddress.address.scriptPubkey().toBytes()),
      );
    });
  });
}
