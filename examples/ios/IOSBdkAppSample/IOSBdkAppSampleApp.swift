//
//  IOSBdkAppSampleApp.swift
//  IOSBdkAppSample
//
//  Created by Sudarsan Balaji on 29/10/21.
//

import SwiftUI

@main
struct IOSBdkAppSampleApp: App {
    @StateObject var wallet = WalletViewModel();
    var body: some Scene {
        WindowGroup {
            WalletView().environmentObject(wallet).preferredColorScheme(.dark)
        }
    }
}
