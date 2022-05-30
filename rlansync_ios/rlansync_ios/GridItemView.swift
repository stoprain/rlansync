//
//  GridItemView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/30.
//

import SwiftUI

struct GridItemView: View {
    let size: CGFloat
    let item: URL

    var body: some View {
        ZStack(alignment: .topTrailing) {
            if let url = item {
                AsyncImage(url: url) { image in
                    image
                        .resizable()
                        .scaledToFill()
                } placeholder: {
                    ProgressView()
                }
                .frame(width: size, height: size)
            }
        }
    }
}

struct GridItemView_Previews: PreviewProvider {
    static var previews: some View {
        GridItemView(size: 50, item: URL(fileURLWithPath: ""))
    }
}
