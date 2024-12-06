API Reference
============

.. currentmodule:: bdkpython

Core Types
---------

Amount and Fees
~~~~~~~~~~~~~~
.. autoclass:: Amount
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: FeeRate
   :members:
   :undoc-members:
   :show-inheritance:

Addresses and Scripts
~~~~~~~~~~~~~~~~~~~
.. autoclass:: Address
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: Script
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: OutPoint
   :members:
   :undoc-members:
   :show-inheritance:

Descriptors
~~~~~~~~~~
.. autoclass:: Descriptor
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: DescriptorPublicKey
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: DescriptorSecretKey
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: DerivationPath
   :members:
   :undoc-members:
   :show-inheritance:

Wallet Operations
---------------

Transaction Building
~~~~~~~~~~~~~~~~~~
.. autoclass:: TxBuilder
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: BumpFeeTxBuilder
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: TxBuilderResult
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: Psbt
   :members:
   :undoc-members:
   :show-inheritance:

Blockchain Clients
~~~~~~~~~~~~~~~~
.. autoclass:: Blockchain
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: ElectrumClient
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: EsploraClient
   :members:
   :undoc-members:
   :show-inheritance:

Wallet
~~~~~~
.. autoclass:: Wallet
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: WalletSync
   :members:
   :undoc-members:
   :show-inheritance:

Utilities
--------
.. autoclass:: Mnemonic
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: Network
   :members:
   :undoc-members:
   :show-inheritance:

Exceptions
---------
.. autoexception:: InternalError
   :members:
   :show-inheritance:

.. autoexception:: FeeRateError
   :members:
   :show-inheritance:

.. autoexception:: ParseAmountError
   :members:
   :show-inheritance:
