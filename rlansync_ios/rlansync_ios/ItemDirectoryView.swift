//
//  ItemDirectoryView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/30.
//

import SwiftUI

struct ItemDirectoryView: View {
    var path: String
    @State private var profileText = ""
    @State private var images = [URL]()
    private static let initialColumns = 3
    @State private var gridColumns = Array(repeating: GridItem(.flexible()), count: initialColumns)
    
    var body: some View {
        VStack {
            TextEditor(text: $profileText)
            
            LazyVGrid(columns: gridColumns) {
                ForEach(images, id: \.self) { image in
                    GeometryReader { geo in
                        NavigationLink(destination: DetailView(item: image)) {
                            GridItemView(size: geo.size.width, item: image)
                        }
                    }
                }
            }
        }
        .onAppear {
            print(path)
            let fm = FileManager.default
            let url = URL(string: path)!
            let fileURLs = try? fm.contentsOfDirectory(at: url, includingPropertiesForKeys: nil)
            for fu in fileURLs ?? [] {
                print(fu)
                if fu.absoluteString.contains(".md") {
                    profileText = (try? String(contentsOf: fu)) ?? ""
                } else if fu.absoluteString.contains(".png") {
                    images.append(fu)
                }
            }
            print(images)
        }
    }
}

struct ItemDirectoryView_Previews: PreviewProvider {
    static var previews: some View {
        ItemDirectoryView(path: "")
    }
}
