//
//  BackButtonMod.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/5/21.
//

import SwiftUI

struct BackButtonMod: ViewModifier {
    @Environment(\.presentationMode) var presentation
    func body(content: Content) -> some View {
        content
            .navigationBarBackButtonHidden(true)
               .toolbar(content: {
                  ToolbarItem (placement: .navigation)  {
                     Image(systemName: "arrow.left")
                     .foregroundColor(.white)
                     .shadow(color: Color("Shadow"), radius: 2, x: 2, y: 2)
                     .onTapGesture {
                         // code to dismiss the view
                         self.presentation.wrappedValue.dismiss()
                     }
                  }
               })
    }
}

struct BackButtonMod_Previews: PreviewProvider {
    static var previews: some View {
        Text("Heyy").modifier(BackButtonMod())
    }
}
