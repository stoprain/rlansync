//
//  ContentView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/16.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        Text("Hello, world! \(shipping_rust_addition(30, 1))")
            .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
