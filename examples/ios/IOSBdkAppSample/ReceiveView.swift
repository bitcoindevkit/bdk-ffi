//
//  ReceiveView.swift
//  IOSBdkAppSample
//
//  Created by Sudarsan Balaji on 02/11/21.
//



import SwiftUI
import CoreImage.CIFilterBuiltins



let context = CIContext()
let filter = CIFilter.qrCodeGenerator()

struct ReceiveView: View {
    func generateQRCode(from string: String) -> UIImage {
        let data = Data(string.utf8)
        filter.setValue(data, forKey: "inputMessage")

        if let outputImage = filter.outputImage {
            if let cgimg = context.createCGImage(outputImage, from: outputImage.extent) {
                return UIImage(cgImage: cgimg)
            }
        }

        return UIImage(systemName: "xmark.circle") ?? UIImage()
    }
    var address: String;
    var body: some View {
        VStack {
            Spacer()
            Image(uiImage: generateQRCode(from: "\(address)"))
                .interpolation(.none)
                .resizable()
                .scaledToFit()
                .frame(width: 200, height: 200)
            Spacer()
            Text("your wallet receive address").padding(5)
            Text(address).padding(10)
            .font(.largeTitle)
            Spacer()
        }
        .navigationBarTitle("Receive")
    }
}


struct ReceiveView_Previews: PreviewProvider {
    static var previews: some View {
        ReceiveView(address: "some-random-address")
    }
}
