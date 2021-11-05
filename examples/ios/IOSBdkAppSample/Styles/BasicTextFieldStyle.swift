//
//  BasicTextFieldStyle.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/5/21.
//

import SwiftUI

struct BasicTextFieldStyle: ViewModifier {
    func body(content: Content) -> some View {
        content
            .disableAutocorrection(true)
            .padding(10)
            .background(Color("Shadow"))
            .cornerRadius(5)
    }
}

struct BasicTextFieldStyle_Previews: PreviewProvider {
    static var previews: some View {
        Text("TEst")
    }
}
