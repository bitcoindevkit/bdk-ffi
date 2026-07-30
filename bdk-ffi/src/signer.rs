use crate::descriptor::Descriptor;

use bdk_wallet::bitcoin::key::Secp256k1;
use bdk_wallet::signer::SignersContainer as BdkSignersContainer;

use std::sync::Arc;

/// Container for multiple signers.
#[derive(Debug, uniffi::Object)]
pub struct SignersContainer {
    pub(crate) inner: BdkSignersContainer,
}

#[uniffi::export]
impl SignersContainer {
    /// Build a new signer container from a descriptor's key map.
    ///
    /// Also looks at the same descriptor to determine the `SignerContext` to attach to the signers.
    #[uniffi::constructor]
    pub fn from_descriptor(descriptor: Arc<Descriptor>) -> Self {
        Self::from_descriptor_with_context(Arc::clone(&descriptor), descriptor)
    }

    /// Build a new signer container from a signer descriptor's key map.
    ///
    /// Also looks at the corresponding descriptor to determine the `SignerContext` to attach to
    /// the signers.
    #[uniffi::constructor]
    pub fn from_descriptor_with_context(
        signer_descriptor: Arc<Descriptor>,
        context_descriptor: Arc<Descriptor>,
    ) -> Self {
        let secp = Secp256k1::new();
        let inner = BdkSignersContainer::build(
            signer_descriptor.key_map.clone(),
            &context_descriptor.extended_descriptor,
            &secp,
        );

        Self { inner }
    }

    /// Returns the number of signer entries registered in the container.
    pub fn len(&self) -> u64 {
        self.inner.signers().len() as u64
    }

    /// Returns true when the container has no signers.
    pub fn is_empty(&self) -> bool {
        self.inner.signers().is_empty()
    }
}
