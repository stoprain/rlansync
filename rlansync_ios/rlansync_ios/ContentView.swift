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
    
    private var obj = SwiftObject()
    @State private var items = [SimpleShareItem]()
    @State private var toolbarLinkSelected = false
    @ObservedObject var observer = Observer()
    
    var body: some View {
        NavigationView {
            List {
                ForEach(items, id: \.url) { item in
                    NavigationLink {
                        if item.url.absoluteString.contains(".txt") {
                            ItemTextView(path: item.url.absoluteString)
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
                ToolbarItem(placement: .navigationBarTrailing) {
                    EditButton()
                }
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
        let fileManager = FileManager.default
        let documentsURL = fileManager.urls(for: .documentDirectory, in: .userDomainMask)[0]
        do {
            items.removeAll()
            let fileURLs = try fileManager.contentsOfDirectory(at: documentsURL, includingPropertiesForKeys: nil)
            for url in fileURLs {
                let attr = try? fileManager.attributesOfItem(atPath: url.path)
                if let date = attr?[FileAttributeKey.modificationDate] as? Date {
                    let item = SimpleShareItem(url: url, date: date)
                    items.append(item)
                }
            }
            items.sort {
                $0.date > $1.date
            }
            print("### loadFromDocument ")
            print(items)
        } catch {
            print("Error while enumerating files \(documentsURL.path): \(error.localizedDescription)")
        }
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
    
    private let itemFormatter: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateStyle = .short
        formatter.timeStyle = .medium
        return formatter
    }()
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
