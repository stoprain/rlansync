//
//  ShareItem.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/30.
//

import Foundation

public struct ShareItem {
    public init(uuid: String, text: String, images: [Data]) {
        self.uuid = uuid
        self.text = text
        self.images = images
    }
    
    var uuid: String
    var text: String
    var images: [Data]
    
    public func save() {
        let fm = FileManager.default
        let path = AppSandboxHelper.documentsPath + "/" + uuid
        if !fm.fileExists(atPath: path) {
            try? fm.createDirectory(atPath: path, withIntermediateDirectories: true)
            let textPath = path + "/" + UUID().uuidString + ".md"
            try? text.write(toFile: textPath, atomically: true, encoding: .utf8)
            for image in images {
                let imagePath = path + "/" + UUID().uuidString + ".png"
                let url = URL(fileURLWithPath: imagePath)
                do {
                    try image.write(to: url)
                } catch (let e) {
                    print(e)
                }
            }
        } else {
            print("warning: file already exist \(uuid)")
        }
    }
}
