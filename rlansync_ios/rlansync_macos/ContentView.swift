//
//  ContentView.swift
//  rlansync_macos
//
//  Created by Rain Qian on 2022/7/4.
//

import SwiftUI
import rlansync_core

struct ContentView: View {

    private var obj = SwiftObject()
    @State private var items = [SimpleShareItem]()
    @State private var toolbarLinkSelected = false
    
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
                ToolbarItemGroup {
                    Button(action: pullItem) {
                        Label("Pull", systemImage: "arrow.triangle.2.circlepath")
                    }
//                    EditButton()
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
            loadFromDocument()
            
            DispatchQueue.global().async {
                obj.sendToRust()
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
    
    private func addItem() {
        toolbarLinkSelected = true
    }
    
    private func pullItem() {
        DispatchQueue.global().async {
            obj.pullFromRust()
        }
    }
    
    private let itemFormatter: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateStyle = .short
        formatter.timeStyle = .medium
        return formatter
    }()

}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
