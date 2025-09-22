import 'package:bdk_dart/bdk.dart';
import 'package:test/test.dart';

import 'test_constants.dart';

Wallet _createWallet(
  Descriptor descriptor,
  Descriptor changeDescriptor,
) {
  final persister = Persister.newInMemory();
  return Wallet(
    descriptor,
    changeDescriptor,
    Network.testnet,
    persister,
    defaultLookahead,
  );
}

void main() {
  group('Wallet construction', () {
    test('creates WPKH wallet', () {
      expect(
        () => _createWallet(
          buildBip84Descriptor(Network.testnet),
          buildBip84ChangeDescriptor(Network.testnet),
        ),
        returnsNormally,
      );
    });

    test('creates TR wallet', () {
      expect(
        () => _createWallet(
          buildBip86Descriptor(Network.testnet),
          buildBip86ChangeDescriptor(Network.testnet),
        ),
        returnsNormally,
      );
    });

    test('creates wallet with non-extended descriptors', () {
      expect(
        () => _createWallet(
          buildNonExtendedDescriptor(0),
          buildNonExtendedDescriptor(1),
        ),
        returnsNormally,
      );
    });

    test('creates single-descriptor wallet', () {
      final persister = Persister.newInMemory();
      expect(
        () => Wallet.createSingle(
          buildBip86Descriptor(Network.testnet),
          Network.testnet,
          persister,
          defaultLookahead,
        ),
        returnsNormally,
      );
    });

    test('creates wallet from public multipath descriptor', () {
      final persister = Persister.newInMemory();
      expect(
        () => Wallet.createFromTwoPathDescriptor(
          buildDescriptor(multipathDescriptorString, Network.bitcoin),
          Network.bitcoin,
          persister,
          defaultLookahead,
        ),
        returnsNormally,
      );
    });

    test('fails for private multipath descriptor', () {
      expect(
        () => buildDescriptor(
          privateMultipathDescriptorString,
          Network.testnet,
        ),
        throwsA(isA<DescriptorException>()),
      );
    });

    test('fails when descriptors do not match network', () {
      final persister = Persister.newInMemory();
      expect(
        () => Wallet(
          buildNonExtendedDescriptor(0),
          buildNonExtendedDescriptor(1),
          Network.bitcoin,
          persister,
          defaultLookahead,
        ),
        throwsA(isA<CreateWithPersistException>()),
      );
    });
  });
}
