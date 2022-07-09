//
//  ItemDirectoryView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/30.
//

import SwiftUI

public struct ItemDirectoryView: View {
    var path: String
    @State private var profileText = ""
    @State private var images = [URL]()
    private static let initialColumns = 3
    @State private var gridColumns = Array(repeating: GridItem(.flexible()), count: initialColumns)
    @State private var txtpath = ""
    
    public init(path: String) {
        self.path = path
    }
    
    public var body: some View {
        VStack {
//            TextEditor(text: .init(profileText))
            TextEditor(text: $profileText)
                .toolbar {
                    ToolbarItem {
                        Button("Save") {
                            if txtpath.count > 0 {
                                let url = URL(fileURLWithPath: txtpath)
                                print(url.lastPathComponent)
                                do {
                                    try profileText.data(using: .utf8)?.write(to: url)
                                } catch (let e) {
                                    print(e)
                                }
                                
                            } else {
                                let uuid = UUID().uuidString
                                let item = ShareItem(uuid: uuid, text: profileText, images: [])
                                item.save()
                            }
                            dismiss()
                        }
                    }
                }
            
            ScrollView {
                HStack {
                    ForEach(images, id: \.self) { image in
                        NavigationLink(destination: DetailView(item: image)) {
                            GridItemView(size: 100, item: image)
                        }
                    }
                }
            }
        }
        .onAppear {
            images.removeAll()
            print(path)
            let fm = FileManager.default
            let url = URL(string: path)!
            let fileURLs = try? fm.contentsOfDirectory(at: url, includingPropertiesForKeys: nil)
            for fu in fileURLs ?? [] {
                print(fu)
                if fu.absoluteString.contains(".txt") || fu.absoluteString.contains(".md") {
                    profileText = .init((try? String(contentsOf: fu)) ?? "")
                    txtpath = fu.path
                } else if fu.absoluteString.contains(".png") {
                    images.append(fu)
                }
            }
            print(profileText)
            print(images)
        }
    }
    
    private func dismiss() {
        #if os(iOS)
        mode.wrappedValue.dismiss()
        #else
        NotificationCenter.default.post(name: NSNotification.Name("notify"), object: nil, userInfo: nil)
        #endif
    }
}

struct ItemDirectoryView_Previews: PreviewProvider {
    static var previews: some View {
        ItemDirectoryView(path: "")
    }
}
