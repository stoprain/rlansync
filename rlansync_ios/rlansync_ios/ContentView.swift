//
//  ContentView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/16.
//

import SwiftUI
import YNLib

struct ContentView: View {
    var body: some View {
        Text("Hello, world! \(shipping_rust_addition(30, 1))")
            .padding()
            .onAppear {
                DispatchQueue.global().async {
                    RAsyncOperation { result in
                        print("RAsyncOperation \(String(cString: result))")
                    }
                }
            }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
