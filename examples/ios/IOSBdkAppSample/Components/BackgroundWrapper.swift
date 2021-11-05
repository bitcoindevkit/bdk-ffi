//
//  BackgroundWrapper.swift
//  MyFirstApp
//
//  Created by Paul Miller on 11/5/21.
//

import SwiftUI

struct BackgroundWrapper<Content: View>: View {
    // @Environment(\.presentationMode) var presentation
    let content: Content
    
    init(@ViewBuilder content: () -> Content) {
        self.content = content()
    }
    
    var body: some View {
        ZStack {
            Color("Background").ignoresSafeArea()
            VStack {
                content
                
            }.padding(10)
        }
//        .navigationBarBackButtonHidden(true)
//           .toolbar(content: {
//              ToolbarItem (placement: .navigation)  {
//                 Image(systemName: "arrow.left")
//                 .foregroundColor(.white)
//                 .onTapGesture {
//                     // code to dismiss the view
//                     self.presentation.wrappedValue.dismiss()
//                 }
//              }
//           })
    }
    
}

struct BackgroundWrapper_Previews: PreviewProvider {
    static var previews: some View {
        BackgroundWrapper() {
            Text("Testing")
        }
    }
}
