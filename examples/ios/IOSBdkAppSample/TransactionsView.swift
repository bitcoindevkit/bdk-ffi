//
//  TransactionsView.swift
//  IOSBdkAppSample
//
//  Created by Sudarsan Balaji on 02/11/21.
//

import SwiftUI

struct TransactionsView: View {
    var transactions: [Transaction]
    
    var body: some View {
        ScrollView {
            if transactions.isEmpty {
                Text("No transactions yet.").padding()
            } else {
                ForEach(1..<transactions.count) { index in
                    Text(String(describing: transactions[index])).padding()
                }
            }
        }.navigationBarTitle("Transactions")
    }
}

struct TransactionsView_Previews: PreviewProvider {
    static var previews: some View {
        TransactionsView(transactions: [Transaction.unconfirmed(details: TransactionDetails(fees: nil, received: 1000, sent: 10000, id: "some-tx-id")), Transaction.confirmed(details: TransactionDetails(fees: nil, received: 1000, sent: 10000, id: "some-tx-id"), confirmation: Confirmation(height: 20087, timestamp: 1635863544))])
    }
}
