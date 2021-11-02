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
    func getBalance() -> UInt64 {
        var balance: UInt64 = 0
        switch viewModel.state {
        case .loaded(let wallet):
            do { balance = try wallet.getBalance() }
            catch { print("failed to fetch balance") }
        default: do {}
        }
        return balance
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
                        Button("Sync") {
                            do {
                                try wallet.sync(progressUpdate: Progress(), maxAddressParam: nil)
                                balance = getBalance()
                            } catch {
                                print("Failed to sync wallet")
                            }
                            
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
                            Button("Send") {
                                
                            }.padding()
                                .foregroundColor(Color.white)
                                .background(Color.blue)
                                .cornerRadius(5)
                                .textCase(.uppercase)
                        }
                        Text("Transactions")
                            .foregroundColor(Color.accentColor)
                            .padding()
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
