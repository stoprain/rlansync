//
//  ContentView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/16.
//

import SwiftUI
import rlansync_core

struct ContentView: View {
    
    let sharedIdentifier = "group.com.stoprain.rlansync"
    
    @State private var items = [SimpleShareItem]()
    @State private var toolbarLinkSelected = false
    @ObservedObject var observer = Observer()
    let pub = NotificationCenter.default.publisher(for: Notification.Name("notify"))
    
    var body: some View {
        NavigationView {
            List {
                ForEach(items, id: \.url) { item in
                    NavigationLink {
                        if item.url.absoluteString.contains(".txt") || item.url.absoluteString.contains(".md") {
                            ItemTextView(path: item.url.absoluteString)
                        } else if item.url.absoluteString.contains(".png") {
                            ItemImageView(path: item.url.absoluteString)
                        } else {
                            ItemDirectoryView(path: item.url.absoluteString)
                        }
                    } label: {
                        Text(item.text ?? "")
                    }
                }
                .onDelete(perform: deleteItems)
            }
            .listStyle(.plain)
            .toolbar {
                ToolbarItemGroup(placement: .bottomBar) {
                    Button(action: pullItem) {
                        Label("Pull", systemImage: "arrow.triangle.2.circlepath")
                    }
                    EditButton()
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
            let _ = loadFromSuite()
            loadFromDocument()
            
            DispatchQueue.global().async {
                obj.sendToRust()
            }
        }
        .onReceive(observer.$enteredForeground) { _ in
            if loadFromSuite() {
                loadFromDocument()
            }
        }
        .onReceive(pub) { output in
            loadFromDocument()
        }
    }
    
    private func deleteItems(offsets: IndexSet) {
        withAnimation {
            offsets.map { items[$0] }.forEach { item in
                item.delete()
                if let index = items.firstIndex(where: { $0.url.path == item.url.path }) {
                    items.remove(at: index)
                }
            }
        }
    }
    
    private func loadFromDocument() {
        items = SimpleShareItem.loadFromDocument()
    }
    
    private func loadFromSuite() -> Bool {
        var hasChange = false
        if let prefs = UserDefaults(suiteName: sharedIdentifier) {
            let uuids = prefs.array(forKey: "share.uuids") as? [String]
            let allkeys = prefs.dictionaryRepresentation().keys
            for uuid in uuids ?? [] {
                print("###### \(uuid)")
                hasChange = true
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
        return hasChange
    }
    
    private func addItem() {
        toolbarLinkSelected = true
    }
    
    private func pullItem() {
        DispatchQueue.global().async {
            SwiftObject.shared.pullFromRust()
        }
    }
}

class Observer: ObservableObject {

    @Published var enteredForeground = true

    init() {
        if #available(iOS 13.0, *) {
            NotificationCenter.default.addObserver(self, selector: #selector(willEnterForeground), name: UIScene.willEnterForegroundNotification, object: nil)
        } else {
            NotificationCenter.default.addObserver(self, selector: #selector(willEnterForeground), name: UIApplication.willEnterForegroundNotification, object: nil)
        }
    }

    @objc func willEnterForeground() {
        enteredForeground.toggle()
    }

    deinit {
        NotificationCenter.default.removeObserver(self)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
