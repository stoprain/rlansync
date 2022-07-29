//
//  SimpleShareItem.swift
//  rlansync_core
//
//  Created by Rain Qian on 2022/7/9.
//

import Foundation

public struct SimpleShareItem {
    public var url: URL
    public var date: Date
    public var text: String?
    
    public static func loadFromDocument() -> [SimpleShareItem] {
        var items = [SimpleShareItem]()
        let fileManager = FileManager.default
        let documentsURL = fileManager.urls(for: .documentDirectory, in: .userDomainMask)[0]
        do {
            let fileURLs = try fileManager.contentsOfDirectory(at: documentsURL, includingPropertiesForKeys: nil)
            for url in fileURLs {
                if url.lastPathComponent == ".DS_Store" {
                    continue
                }
                let attr = try? fileManager.attributesOfItem(atPath: url.path)
                if let date = attr?[FileAttributeKey.modificationDate] as? Date {
                    let item = SimpleShareItem(url: url, date: date)
                    items.append(item)
                }
            }
            items.sort {
                $0.date > $1.date
            }
            print("### loadFromDocument ")
//            print(items)
        } catch {
            print("Error while enumerating files \(documentsURL.path): \(error.localizedDescription)")
        }
        return items
    }
    
    public init(url: URL, date: Date) {
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
    
    public func delete() {
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
