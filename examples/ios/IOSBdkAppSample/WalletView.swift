//
//  WalletView.swift
//  IOSBdkAppSample
//
//  Created by Sudarsan Balaji on 29/10/21.
//

import SwiftUI
import Combine

class Progress : BdkProgress {
    func update(progress: Float, message: String?) {
        print("progress", progress, message as Any)
    }
}

struct WalletView: View {
    @EnvironmentObject var viewModel: WalletViewModel
    @State var balance: UInt64 = 0
    @State var transactions: [Transaction] = []
    func sync() {
        switch viewModel.state {
        case .loaded(let wallet):
            do {
                try wallet.sync(progressUpdate: Progress(), maxAddressParam: nil)
                balance = try wallet.getBalance()
                let wallet_transactions = try wallet.getTransactions()
                transactions = wallet_transactions.sorted(by: {
                switch $0 {
                case .confirmed(_, let confirmation_a):
                    switch $1 {
                    case .confirmed(_, let confirmation_b): return confirmation_a.timestamp > confirmation_b.timestamp
                    default: return false
                    }
                default:
                    switch $1 {
                    case .unconfirmed(_): return true
                    default: return false
                    }
                } })
          } catch let error {
              print(error)
          }
        default: do { }
        }
    }
    var body: some View {
        NavigationView {
            switch viewModel.state {
            case .empty:
                Color.clear
                    .onAppear(perform: viewModel.load)
            case .loading:
                ProgressView()
            case .failed(_):
                Text("Failed to load wallet")
            case .loaded(let wallet):
                    VStack {
                        Button(action: self.sync) {
                            Text("Sync")
                        }.padding()
                        Text("Balance")
                        HStack {
                            Text(String(format: "%.8f", Double(balance) / Double(100000000)))
                            Text("BTC")
                        }
                        HStack {
                            NavigationLink(destination: ReceiveView(address: wallet.getNewAddress())) {
                                Text("Receive")
                                    .padding()
                                    .foregroundColor(Color.white)
                                    .background(Color.blue)
                                    .cornerRadius(5)
                                    .textCase(.uppercase)
                            }
                            NavigationLink(destination: SendView(onSend: { recipient, amount in
                                do {
                                    let psbt = try PartiallySignedBitcoinTransaction(wallet: wallet, recipient: recipient, amount: amount)
                                    try wallet.sign(psbt: psbt)
                                    let id = try wallet.broadcast(psbt: psbt)
                                    print(id)
                                } catch let error {
                                    print(error)
                                }
                            })) {
                                Text("Send")
                                    .padding()
                                    .foregroundColor(Color.white)
                                    .background(Color.blue)
                                    .cornerRadius(5)
                                    .textCase(.uppercase)
                            }
                        }
                        NavigationLink(destination: TransactionsView(transactions: transactions)) {
                            Text("Transactions")
                                .foregroundColor(Color.accentColor)
                                .padding()
                        }
                    }.navigationBarTitle("BitcoinDevKit")
            }
        }
    }
}

struct WalletView_Previews: PreviewProvider {
    static var previews: some View {
        WalletView()
    }
}
