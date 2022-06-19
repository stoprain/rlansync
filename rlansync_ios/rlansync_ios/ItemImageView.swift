//
//  ItemImageView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/6/19.
//

import SwiftUI

struct ItemImageView: View {
    var path: String
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>
    @State private var image = UIImage()
    
    var body: some View {
        Image(uiImage: image)
            .onAppear {
                if path.count > 0 {
//                    image = UIImage(contentsOfFile: path) ?? UIImage()
                    let url = URL(string: path)!
                    let d = try? Data(contentsOf: url)
                    image = UIImage(data: d!)!
//                    print(d)
//                    print(image)
//                    print(path)
                }
            }
//        TextEditor(text: $profileText)
////            .frame(height: 60)
////            .background(RoundedRectangle(cornerRadius: 4.0).stroke(Color.gray, lineWidth: 2))
////            .foregroundColor(.blue)
////            .padding()
//            .toolbar {
//                ToolbarItem(placement: .navigationBarTrailing) {
//                    Button("Save") {
//                        let uuid = UUID().uuidString
//                        let item = ShareItem(uuid: uuid, text: profileText, images: [])
//                        item.save()
//                        mode.wrappedValue.dismiss()
//                    }
//                }
//            }
//            .onAppear {
//                if path.count > 0 {
//                    let url = URL(string: path)!
//                    let s = try? String(contentsOf: url, encoding: .utf8)
//                    profileText = s ?? ""
//                }
//            }
    }
}

struct ItemImageView_Previews: PreviewProvider {
    static var previews: some View {
        ItemImageView(path: "")
    }
}
