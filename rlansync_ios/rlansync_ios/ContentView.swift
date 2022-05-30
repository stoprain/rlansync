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
                            ItemDirectoryView(path: item.absoluteString)
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
                let uuids = prefs.array(forKey: "share.uuids") as? [String]
                let allkeys = prefs.dictionaryRepresentation().keys
                for uuid in uuids ?? [] {
                    print("###### \(uuid)")
                    var text = ""
                    var images = [Data]()
                    for key in allkeys {
                        if key.contains(uuid) {
                            if key.contains("public.plain-text") {
                                text = prefs.string(forKey: key) ?? ""
                            } else if key.contains("public.image") {
                                let imageDatas = prefs.array(forKey: key) as? [Data]
                                for data in imageDatas ?? [] {
                                    images.append(data)
                                }
                            }
                        }
                    }
                    let si = ShareItem(uuid: uuid, text: text, images: images)
                    si.save()
                }
                for key in allkeys {
                    prefs.removeObject(forKey: key)
                }
                prefs.removeObject(forKey: "share.uuids")
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
