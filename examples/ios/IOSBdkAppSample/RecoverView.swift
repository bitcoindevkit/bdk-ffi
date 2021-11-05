//
//  RecoverView.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/5/21.
//

import SwiftUI

struct RecoverView: View {
    @State private var words = Array(repeating: "", count: 12)
    
    var body: some View {
        BackgroundWrapper {
            ScrollView {
                ForEach(0..<12) { i in
                    TextField(
                    "Word \(i + 1)",
                    text: $words[i]
                    )
                    .modifier(BasicTextFieldStyle())
                }
                .disableAutocorrection(true).padding(.bottom, 10)
                BasicButton(action: {}, text: "Recover")
            }
        }
        .navigationTitle("Recover Wallet")
        .modifier(BackButtonMod())
    }
}

struct RecoverView_Previews: PreviewProvider {
    static var previews: some View {
        RecoverView()
    }
}
