//
//  ContentView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/16.
//

import SwiftUI
import YNLib

struct ContentView: View {
    
    let sharedIdentifier = "group.com.stoprain.rlansync"
    
    @State private var items = [URL]()
    @State private var toolbarLinkSelected = false
    
    var body: some View {
        NavigationView {
            List {
                ForEach(items, id: \.self) { item in
                    NavigationLink {
                        if item.absoluteString.contains(".txt") {
                            ItemTextView(path: item.absoluteString)
                        } else {
                            Text(item.lastPathComponent)
                        }
                    } label: {
                        Text(item.lastPathComponent)
                    }
                }
            }
            .toolbar {
//                ToolbarItem(placement: .navigationBarTrailing) {
//                    EditButton()
//                }
                ToolbarItem {
                    Button(action: addItem) {
                        Label("Add Item", systemImage: "plus")
                    }
                }
            }
            .background {
                NavigationLink(destination: ItemTextView(path: ""),
                               isActive: $toolbarLinkSelected) {
                    
                }.hidden()
            }
            Text("Detail pane")
        }
        .onAppear {
            let fileManager = FileManager.default
            let documentsURL = fileManager.urls(for: .documentDirectory, in: .userDomainMask)[0]
            do {
                items.removeAll()
                let fileURLs = try fileManager.contentsOfDirectory(at: documentsURL, includingPropertiesForKeys: nil)
                for url in fileURLs {
//                    items.append(url.absoluteString)
                    items.append(url)
                }
            } catch {
                print("Error while enumerating files \(documentsURL.path): \(error.localizedDescription)")
            }
            
            if let prefs = UserDefaults(suiteName: sharedIdentifier) {
                let t = prefs.string(forKey: "share.public.plain-text")
                print("##### \(t)")
                
                let tt = prefs.array(forKey: "share.public.image")
                print("##### \(tt)")
            }
        }
//        Text("Hello, world! \(shipping_rust_addition(30, 1))")
//            .padding()
//            .onAppear {
////                DispatchQueue.global().async {
////                    RAsyncOperation { result in
////                        print("RAsyncOperation \(String(cString: result))")
////                    }
////                }
//            }
    }
    
    private func addItem() {
        toolbarLinkSelected = true
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
