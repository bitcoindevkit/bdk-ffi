//
//  ReceiveView.swift
//  IOSBdkAppSample
//
//  Created by Sudarsan Balaji on 02/11/21.
//

import SwiftUI

struct ReceiveView: View {
    var address: String;
    var body: some View {
        VStack {
            Text("your wallet receive address")
            Text(address)
                .font(.largeTitle)
        }.navigationBarTitle("Receive")
    }
}

struct ReceiveView_Previews: PreviewProvider {
    static var previews: some View {
        ReceiveView(address: "some-random-address")
    }
}
