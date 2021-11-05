//
//  TransactionsView.swift
//  IOSBdkAppSample
//
//  Created by Sudarsan Balaji on 02/11/21.
//

import SwiftUI

extension Transaction {
    public func getDetails() -> TransactionDetails {
        switch self {
        case .unconfirmed(let details): return details
        case .confirmed(let details, _): return details
        }
    }
}

extension Date {
    func getFormattedDate(format: String) -> String {
        let dateformat = DateFormatter()
        dateformat.dateFormat = format
        return dateformat.string(from: self)
    }
}

struct TransactionView: View {
    var transaction: Transaction
    
    var body: some View {
        VStack(alignment: .leading) {
            switch transaction {
            case .unconfirmed(let details):
                SingleTxView(id: details.id, sent: String(details.sent), received: String(details.received), fees: String(details.fees ?? 0))
            case .confirmed(let details, let confirmation):
                SingleTxView(id: details.id, sent: String(details.sent), received: String(details.received), fees: String(details.fees ?? 0), confirmed: "block \(confirmation.height) at \(Date(timeIntervalSince1970: TimeInterval(confirmation.timestamp)).getFormattedDate(format: "yyyy-MM-dd HH:mm:ss"))")
            }
        }
    }
}

struct TransactionsView: View {
    var transactions: [Transaction]
    
    var body: some View {
        BackgroundWrapper {
            
            List {
                if transactions.isEmpty {
                    Text("No transactions yet.").padding()
                } else {
                    ForEach(transactions, id: \.self) { transaction in
                        TransactionView(transaction: transaction)
                    }
                }
            }
            .onAppear {
                UITableView.appearance().backgroundColor = .clear }
        }.navigationTitle("Transactions")
            .modifier(BackButtonMod())
    }
}

struct TransactionsView_Previews: PreviewProvider {
    static var previews: some View {
        TransactionsView(transactions: [Transaction.unconfirmed(details: TransactionDetails(fees: nil, received: 1000, sent: 10000, id: "some-tx-id")), Transaction.confirmed(details: TransactionDetails(fees: nil, received: 1000, sent: 10000, id: "some-other-tx-id"), confirmation: Confirmation(height: 20087, timestamp: 1635863544))])
    }
}
