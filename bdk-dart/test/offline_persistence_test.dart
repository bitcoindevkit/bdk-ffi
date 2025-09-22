import 'dart:io';

import 'package:bdk_dart/bdk.dart';
import 'package:test/test.dart';

import 'test_constants.dart';

const _fixtureName = 'pre_existing_wallet_persistence_test.sqlite';

String _copyFixtureToTempDir() {
  final source = File('test/data/$_fixtureName');
  final tempDir = Directory.systemTemp.createTempSync('bdk_dart_persistence_');
  addTearDown(() => tempDir.delete(recursive: true));
  final destination = File('${tempDir.path}/$_fixtureName');
  destination.writeAsBytesSync(source.readAsBytesSync(), flush: true);
  return destination.path;
}

void main() {
  group('Offline persistence', () {
    test('loads sqlite wallet with private descriptors', () {
      final dbPath = _copyFixtureToTempDir();
      final persister = Persister.newSqlite(dbPath);
      final wallet = Wallet.load(
        buildDescriptor(persistenceDescriptorString, Network.signet),
        buildDescriptor(persistenceChangeDescriptorString, Network.signet),
        persister,
        defaultLookahead,
      );

      final addressInfo =
          wallet.revealNextAddress(KeychainKind.external_);
      final expectedAddress = Address(expectedPersistedAddress, Network.signet);

      expect(addressInfo.index, equals(7));
      expect(
        addressInfo.address.scriptPubkey().toBytes(),
        orderedEquals(expectedAddress.scriptPubkey().toBytes()),
      );
    });

    test('loads sqlite wallet with public descriptors', () {
      final dbPath = _copyFixtureToTempDir();
      final persister = Persister.newSqlite(dbPath);
      final wallet = Wallet.load(
        buildDescriptor(persistencePublicDescriptorString, Network.signet),
        buildDescriptor(persistencePublicChangeDescriptorString, Network.signet),
        persister,
        defaultLookahead,
      );

      final addressInfo =
          wallet.revealNextAddress(KeychainKind.external_);

      expect(addressInfo.index, equals(7));
      final expectedAddress = Address(expectedPersistedAddress, Network.signet);
      expect(
        addressInfo.address.scriptPubkey().toBytes(),
        orderedEquals(expectedAddress.scriptPubkey().toBytes()),
      );
    });
  });
}
