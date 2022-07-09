//
//  ItemTextView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/29.
//

import SwiftUI

public struct ItemTextView: View {
    
    var path: String
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>
    @State private var profileText = ""
    
    public init(path: String) {
        self.path = path
    }
    
    public var body: some View {
        TextEditor(text: $profileText)
            .toolbar {
                ToolbarItem {
                    Button("Save") {
                        let uuid = UUID().uuidString
                        let item = ShareItem(uuid: uuid, text: profileText, images: [])
                        item.save()
                        #if os(iOS)
                        mode.wrappedValue.dismiss()
                        #else
                        #endif
                    }
                }
            }
            .onAppear {
                if path.count > 0 {
                    let url = URL(string: path)!
                    let s = try? String(contentsOf: url, encoding: .utf8)
                    profileText = s ?? ""
                }
            }
    }
}

struct ItemTextView_Previews: PreviewProvider {
    static var previews: some View {
        ItemTextView(path: "")
    }
}
