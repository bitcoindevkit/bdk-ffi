import 'package:bdk_dart/bdk.dart';
import 'package:test/test.dart';

import 'test_constants.dart';

void main() {
  group('Mnemonic', () {
    test('produces expected BIP86 descriptor', () {
      final mnemonic = Mnemonic.fromString(
        "space echo position wrist orient erupt relief museum myself grain wisdom tumble",
      );
      final descriptorSecretKey = DescriptorSecretKey(
        Network.testnet,
        mnemonic,
        null,
      );
      final descriptor = Descriptor.newBip86(
        descriptorSecretKey,
        KeychainKind.external_,
        Network.testnet,
      );

      expect(
        descriptor.toString(),
        equals(
          "tr([be1eec8f/86'/1'/0']tpubDCTtszwSxPx3tATqDrsSyqScPNnUChwQAVAkanuDUCJQESGBbkt68nXXKRDifYSDbeMa2Xg2euKbXaU3YphvGWftDE7ozRKPriT6vAo3xsc/0/*)#m7puekcx",
        ),
      );
    });
  });
}
