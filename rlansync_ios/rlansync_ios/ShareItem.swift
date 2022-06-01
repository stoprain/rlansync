//
//  ShareItem.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/30.
//

import YNLib

struct SimpleShareItem {
    var url: URL
    var date: Date
    var text: String?
    
    internal init(url: URL, date: Date) {
        self.url = url
        self.date = date
   
        let fm = FileManager.default
        let fileURLs = try? fm.contentsOfDirectory(at: url, includingPropertiesForKeys: nil)
        for fu in fileURLs ?? [] {
            if fu.lastPathComponent.contains(".md") {
                text = (try? String(contentsOf: fu)) ?? ""
                return
            }
        }
        
        text = url.lastPathComponent
        //\(item.date, formatter: itemFormatter)
    }
    
    func delete() {
        let fm = FileManager.default
        print("#### delete \(fm.fileExists(atPath: url.path))")
        do {
            try FileManager.default.removeItem(atPath: url.path)
        } catch (let e) {
            print(e)
        }
        print("#### delete \(fm.fileExists(atPath: url.path))")
    }
}

struct ShareItem {
    var uuid: String
    var text: String
    var images: [Data]
    
    func save() {
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
