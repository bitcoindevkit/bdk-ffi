//
//  BasicButtonStyle.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/4/21.
//
import SwiftUI

struct BasicButtonStyle: ButtonStyle {
    var bgColor: Color

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .frame(maxWidth: .infinity, maxHeight: 40)
            .foregroundColor(Color("Shadow"))
            .background(Color("Primary"))
            .cornerRadius(10.0)
            .shadow(color: Color("Shadow"), radius: 1, x: 5, y: 5)
            .padding(20)
    }
}

struct BasicButtonStyle_Previews: PreviewProvider {
    static var previews: some View {
        Button(action: {}) {
            Text("Hello").textStyle(BasicTextStyle())
        }.buttonStyle(BasicButtonStyle(bgColor: Color("Primary")))
    }
}
