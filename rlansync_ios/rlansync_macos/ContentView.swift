//
//  ContentView.swift
//  rlansync_macos
//
//  Created by Rain Qian on 2022/7/4.
//

import SwiftUI
import rlansync_core

struct ContentView: View {

    @State private var items = [SimpleShareItem]()
    @State private var toolbarLinkSelected = false
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
                SwiftObject.shared.sendToRust()
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
    
    private func addItem() {
        toolbarLinkSelected = true
    }
    
    private func pullItem() {
        DispatchQueue.global().async {
            SwiftObject.shared.pullFromRust()
        }
    }

}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
