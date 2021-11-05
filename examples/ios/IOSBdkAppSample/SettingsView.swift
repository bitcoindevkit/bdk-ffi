//
//  SettingsView.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/4/21.
//

import SwiftUI

struct SettingsView: View {
    @State private var goToWords = false
    @State private var goToRecovery = false
    let words = "clutch solar sand travel vital fitness hand piece dial flag garment grant"
    
    var body: some View {
            NavigationLink(destination: MnemonicView(words: words), isActive: $goToWords) { EmptyView() }
            NavigationLink(destination: RecoverView(), isActive: $goToRecovery) { EmptyView() }
        BackgroundWrapper {
            Spacer()
            Text("Running on Bitcoin Testnet").textStyle(BasicTextStyle(white: true, bold: true))
            Spacer()
            BasicButton(action: { goToWords = true }, text: "Create a New Wallet")
            BasicButton(action: { goToRecovery = true}, text: "Recover an Existing Wallet")
        }
        .navigationTitle("Wallet Setup")
        .modifier(BackButtonMod())
 
    }
}

struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView()
    }
}
