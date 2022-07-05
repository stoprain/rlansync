//
//  DetailView.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/30.
//

import SwiftUI

struct DetailView: View {
    let item: URL

    var body: some View {
        AsyncImage(url: item) { image in
            image
                .resizable()
                .scaledToFit()
        } placeholder: {
            ProgressView()
        }
    }
}

struct DetailView_Previews: PreviewProvider {
    static var previews: some View {
        DetailView(item: URL(fileURLWithPath: ""))
    }
}
