import 'package:bdk_dart/bdk.dart';
import 'package:test/test.dart';

import 'test_constants.dart';

void main() {
  group('Descriptor construction', () {
    test('creates extended WPKH descriptors across networks', () {
      expect(
        () => buildBip84Descriptor(Network.regtest),
        returnsNormally,
      );
      expect(
        () => buildBip84Descriptor(Network.testnet),
        returnsNormally,
      );
      expect(
        () => buildBip84Descriptor(Network.testnet4),
        returnsNormally,
      );
      expect(
        () => buildBip84Descriptor(Network.signet),
        returnsNormally,
      );
      expect(
        () => buildMainnetBip84Descriptor(),
        returnsNormally,
      );
    });

    test('creates extended TR descriptors across networks', () {
      expect(
        () => buildBip86Descriptor(Network.regtest),
        returnsNormally,
      );
      expect(
        () => buildBip86Descriptor(Network.testnet),
        returnsNormally,
      );
      expect(
        () => buildBip86Descriptor(Network.testnet4),
        returnsNormally,
      );
      expect(
        () => buildBip86Descriptor(Network.signet),
        returnsNormally,
      );
      expect(
        () => buildMainnetBip86Descriptor(),
        returnsNormally,
      );
    });

    test('creates non-extended descriptors for all networks', () {
      expect(
        () => Descriptor(
          "tr($testExtendedPrivKey/$bip86TestReceivePath/0)",
          Network.regtest,
        ),
        returnsNormally,
      );
      expect(
        () => Descriptor(
          "tr($testExtendedPrivKey/$bip86TestReceivePath/0)",
          Network.testnet,
        ),
        returnsNormally,
      );
      expect(
        () => Descriptor(
          "tr($testExtendedPrivKey/$bip86TestReceivePath/0)",
          Network.testnet4,
        ),
        returnsNormally,
      );
      expect(
        () => Descriptor(
          "tr($testExtendedPrivKey/$bip86TestReceivePath/0)",
          Network.signet,
        ),
        returnsNormally,
      );
      expect(
        () => Descriptor(
          "tr($mainnetExtendedPrivKey/$bip86MainnetReceivePath/0)",
          Network.bitcoin,
        ),
        returnsNormally,
      );
    });

    test('fails to create addr() descriptor', () {
      expect(
        () => Descriptor(
          "addr(tb1qhjys9wxlfykmte7ftryptx975uqgd6kcm6a7z4)",
          Network.testnet,
        ),
        throwsA(isA<DescriptorException>()),
      );
    });
  });
}
