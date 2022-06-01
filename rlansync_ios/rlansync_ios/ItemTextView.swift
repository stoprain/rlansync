//
//  ItemTextView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/29.
//

import SwiftUI
import YNLib

struct ItemTextView: View {
    
    var path: String
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>
    @State private var profileText = ""
    
    var body: some View {
        TextEditor(text: $profileText)
//            .frame(height: 60)
//            .background(RoundedRectangle(cornerRadius: 4.0).stroke(Color.gray, lineWidth: 2))
//            .foregroundColor(.blue)
//            .padding()
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Save") {
                        let uuid = UUID().uuidString
                        let item = ShareItem(uuid: uuid, text: profileText, images: [])
                        item.save()
                        mode.wrappedValue.dismiss()
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
