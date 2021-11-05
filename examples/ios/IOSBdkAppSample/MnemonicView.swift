//
//  IntroView.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/4/21.
//

import SwiftUI

struct MnemonicView: View {
    var words: String
    @State private var goHome = false
    
    var body: some View {
        BackgroundWrapper {
            Spacer()
            Text(words).textStyle(BasicTextStyle(big: true, white: true)).contextMenu {
                Button(action: {
                    UIPasteboard.general.string = self.words}) {
                        Text("Copy to clipboard")
                    }
            }
            Spacer()
            NavigationLink(destination: WalletView(), isActive: $goHome) { EmptyView() }
            BasicButton(action: { () in goHome = true}, text: "Back to Wallet")
        }
        .navigationTitle("Mnemonic")
        .modifier(BackButtonMod())
    }
}

struct RecoveryView_Previews: PreviewProvider {
    static var previews: some View {
        MnemonicView(words: "clutch solar sand travel vital fitness hand piece dial flag garment grant")
    }
}
