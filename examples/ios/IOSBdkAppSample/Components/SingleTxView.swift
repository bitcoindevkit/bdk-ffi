//
//  SingleTxView.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/5/21.
//

import SwiftUI

struct SingleTxView: View {
    var id: String
    var sent: String
    var received: String
    var fees: String
    var confirmed: String?
    var body: some View {
        VStack(alignment: .leading, spacing: 5) {
            HStack {
                Text("Txid:").textStyle(BasicTextStyle(white: true, bold: true))
                Text(id).textStyle(BasicTextStyle(white: true))
            }
            HStack {
                Text("Sent:").textStyle(BasicTextStyle(white: true, bold: true))
                Text(sent).textStyle(BasicTextStyle(white: true))
            }
            HStack {
                Text("Received:").textStyle(BasicTextStyle(white: true, bold: true))
                Text(received).textStyle(BasicTextStyle(white: true))
            }
            HStack {
                Text("Fees:").textStyle(BasicTextStyle(white: true, bold: true))
                Text(fees).textStyle(BasicTextStyle(white: true))
            }
            
            if (confirmed != nil) {
                HStack {
                    Text("Confirmed:").textStyle(BasicTextStyle(white: true, bold: true))
                    Text(confirmed!).textStyle(BasicTextStyle(white: true))
                }
            }
        }.padding(10)
            .background(Color("Shadow")).cornerRadius(5)
            .contextMenu {
                Button(action: {
                    UIPasteboard.general.string = id}) {
                        Text("Copy TXID")
                    }
            }
            .padding(.vertical, 10)
    }
}

struct SingleTxView_Previews: PreviewProvider {
    static var previews: some View {
        Text("Fake tx")
        //        SingleTxView(txid: "abcdefg")
    }
}
