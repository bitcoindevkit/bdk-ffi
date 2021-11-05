//
//  SendView.swift
//  IOSBdkAppSample
//
//  Created by Sudarsan Balaji on 02/11/21.
//

import SwiftUI
import Combine
import CodeScanner

struct SendView: View {
    @State var to: String = ""
    @State var amount: String = "0.000"
    @State private var isShowingScanner = false
    @Environment(\.presentationMode) var presentationMode
    func handleScan(result: Result<String, CodeScannerView.ScanError>) {
       self.isShowingScanner = false
        switch result {
        case .success(let code):
            self.to = code
        case .failure(let error):
            print(error)
        }
    }
    var onSend : (String, UInt64) -> ()
    var body: some View {
        Form {
            Section(header: Text("Recipient")) {
                TextField("Address", text: $to)
            }
            Section(header: Text("Amount (BTC)")) {
                TextField("Amount", text: $amount)
                    .keyboardType(.numberPad)
                    .onReceive(Just(amount)) { newValue in
                        let filtered = newValue.filter { "0123456789.".contains($0) }
                        if filtered != newValue {
                            self.amount = filtered
                        }
                    }
            }
            Section {
                Button("Send") {
                    onSend(to, UInt64((Double(amount) ?? 0) * Double(100000000)))
                    presentationMode.wrappedValue.dismiss()
                }.disabled(to == "" || (Double(amount) ?? 0) == 0)
                Spacer()
            }
        }.navigationBarTitle("Send")
            .sheet(isPresented: $isShowingScanner) {
                CodeScannerView(codeTypes: [.qr], simulatedData: "Testing1234", completion: self.handleScan)}
        Button(action: {self.isShowingScanner = true}) {
            Text("Scan Wallet Address").padding(30)
        }
    }
}

struct SendView_Previews: PreviewProvider {
    static func onSend(to: String, amount: UInt64) {
        
    }
    static var previews: some View {
        SendView(onSend: self.onSend)
    }
}
