//
//  TransactionsView.swift
//  IOSBdkAppSample
//
//  Created by Sudarsan Balaji on 02/11/21.
//

import SwiftUI

class TransactionsViewModel: ObservableObject {
    enum State {
        case empty
        case loading
        case failed(Error)
        case loaded([Transaction])
    }
    @Published private(set) var state = State.empty
    private var wallet: OnlineWalletProtocol
    
    init(wallet: OnlineWalletProtocol) {
        self.wallet = wallet
    }
    
    func load() {
        do {
            state = State.loading
            let transactions = try wallet.getTransactions()
            state = State.loaded(transactions)
        } catch let error {
            print(error)
            state = State.failed(error)
        }
    }
}

struct TransactionsView: View {
    @ObservedObject var viewModel: TransactionsViewModel
    
    var body: some View {
        ScrollView {
            switch viewModel.state {
            case .empty:
                Color.clear
                    .onAppear(perform: viewModel.load)
            case .loading:
                ProgressView()
            case .failed(_):
                Text("Failed to load wallet")
            case .loaded(let transactions):
                if transactions.isEmpty {
                    Text("No transactions yet.")
                } else {
                    ForEach(1..<transactions.count) { index in
                        Text(String(describing: transactions[index]))
                    }
                }
            }
        }.navigationBarTitle("Transactions")
    }
}

struct DummyWallet : OnlineWalletProtocol {
    func getNewAddress() -> String {
        ""
    }
    
    func getBalance() throws -> UInt64 {
        0
    }
    
    func sign(psbt: PartiallySignedBitcoinTransaction) throws {
        
    }
    
    func getTransactions() throws -> [Transaction] {
        [Transaction.unconfirmed(details: TransactionDetails(fees: nil, received: 1000, sent: 10000, id: "some-tx-id")), Transaction.confirmed(details: TransactionDetails(fees: nil, received: 1000, sent: 10000, id: "some-tx-id"), confirmation: Confirmation(height: 20087, timestamp: 1635863544))]
    }
    
    func getNetwork() -> Network {
        Network.testnet
    }
    
    func sync(progressUpdate: BdkProgress, maxAddressParam: UInt32?) throws {
        
    }
    
    func broadcast(psbt: PartiallySignedBitcoinTransaction) throws -> String {
        "random-txid"
    }
    
}

struct TransactionsView_Previews: PreviewProvider {
    static var previews: some View {
        TransactionsView(viewModel: TransactionsViewModel(wallet: DummyWallet()))
    }
}
