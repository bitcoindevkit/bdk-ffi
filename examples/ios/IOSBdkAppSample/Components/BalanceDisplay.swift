//
//  BalanceDisplay.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/4/21.
//

import SwiftUI

struct BalanceDisplay: View {
    var balance: String
    var body: some View {
        Text("₿ \(balance)")
            .font(.system(size: 32, design: .monospaced))
            .fontWeight(.semibold)
            .foregroundColor(Color.white)
            .padding(10)
            .frame(maxWidth: .infinity)
            .background(Color("Shadow"))
    }
}

struct BalanceDisplay_Previews: PreviewProvider {
    static var previews: some View {
        BalanceDisplay(balance: "0.00042069")
    }
}
