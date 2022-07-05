//
//  ItemDirectoryView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/30.
//

import SwiftUI

public struct ItemDirectoryView: View {
    var path: String
    @State private var profileText: LocalizedStringKey = ""
    @State private var images = [URL]()
    private static let initialColumns = 3
    @State private var gridColumns = Array(repeating: GridItem(.flexible()), count: initialColumns)
    
    public init(path: String) {
        self.path = path
    }
    
    public var body: some View {
        VStack {
//            TextEditor(text: .init(profileText))
            Text(profileText)
            
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
                if fu.absoluteString.contains(".md") {
                    profileText = .init((try? String(contentsOf: fu)) ?? "")
                } else if fu.absoluteString.contains(".png") {
                    images.append(fu)
                }
            }
            print(profileText)
            print(images)
        }
    }
}

struct ItemDirectoryView_Previews: PreviewProvider {
    static var previews: some View {
        ItemDirectoryView(path: "")
    }
}
