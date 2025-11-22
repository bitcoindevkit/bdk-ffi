import 'package:bdk_dart/bdk.dart';

void main() {
  // Generate a new mnemonic with 12 words
  final mnemonic12 = Mnemonic(WordCount.words12);
  print('12-word mnemonic: ${mnemonic12}');

  // Generate a new mnemonic with 24 words
  final mnemonic24 = Mnemonic(WordCount.words24);
  print('24-word mnemonic: ${mnemonic24}');

  // Create a BIP84 descriptor from the 12-word mnemonic
  final descriptorSecretKey = DescriptorSecretKey(
    Network.testnet,
    mnemonic12,
    null,
  );

  final descriptor = Descriptor.newBip84(
    descriptorSecretKey,
    KeychainKind.external_,
    Network.testnet,
  );

  final changeDescriptor = Descriptor.newBip84(
    descriptorSecretKey,
    KeychainKind.internal,
    Network.testnet,
  );

  // Create a wallet with the descriptor and in-memory persister
  final wallet = Wallet(
    descriptor,
    changeDescriptor,
    Network.testnet,
    Persister.newInMemory(),
    0,
  );

  // Generate a receive address
  final addressInfo = wallet.nextUnusedAddress(KeychainKind.external_);
  print('\nReceive address: ${addressInfo.address}');
  print('Address index: ${addressInfo.index}');
}
