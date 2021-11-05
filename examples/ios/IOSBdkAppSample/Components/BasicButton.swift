//
//  BasicButton.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/4/21.
//

import SwiftUI

struct BasicButton: View {
    var action: () -> Void
    var text: String
    var color = "Primary"
    
    var body: some View {
        Button(action: action) {
                    Text(text)
                        .font(.system(size: 14, design: .monospaced))
                        .fontWeight(.bold)
                        .frame(maxWidth: .infinity, maxHeight: 40)
                        .foregroundColor(Color("Shadow"))
                        .padding(10)
                        .background(Color(color))
                        .cornerRadius(10.0)
                        .shadow(color: Color("Shadow"), radius: 1, x: 5, y: 5)
                }
    }
}

struct BasicButton_Previews: PreviewProvider {
    static var previews: some View {
        BasicButton(action: {}, text: "Test")
    }
}
