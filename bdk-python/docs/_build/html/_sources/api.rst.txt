BDK Python API Reference
=====================

This document describes the Python API for the Bitcoin Development Kit (BDK).

Bdk Module
==========
Bitcoin Module
==============

Examples
--------

Basic Wallet Usage
~~~~~~~~~~~~~~~~

.. code-block:: python

   from bdkpython import *   
                # Create a new wallet
                descriptor = "wpkh(...)"  # Your descriptor here
                wallet = Wallet(descriptor, network=Network.TESTNET)
                
                # Sync wallet
                blockchain = Blockchain("https://blockstream.info/testnet/api")
                wallet.sync(blockchain)
                
                # Get balance
                balance = wallet.get_balance()
                print(f"Confirmed balance: {balance.confirmed}")
                