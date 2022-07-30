//
//  rlansynclib.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/19.
//

import Foundation
import rlansync_lib

struct EntryInfo: Codable {
    var path: String
    var modified: Int
    var digest: String
}

public class rlansync_lib_helper {
    
    public static let shared = rlansync_lib_helper()
    
    let rust: RustApp!
    
    private init() {
        rust = RustApp()
        rlansync_bridge.shared.callback = { (s: String) in
            print("rlansync_bridge callback")
            guard let data = s.data(using: .utf8) else {
                return
            }
            let decoder = JSONDecoder()
            do {
                let decoded = try decoder.decode([EntryInfo].self, from: data)
                print("decoded EntryInfo \(decoded.count)")
            } catch (let e) {
                print(e)
                print("Failed to decode JSON")
            }
            DispatchQueue.main.async {
                NotificationCenter.default.post(name: NSNotification.Name("notify"), object: nil, userInfo: nil)
            }
        }
    }
    
    public func setup() {
        rust.setup(AppSandboxHelper.documentsPath)
    }
    
    public func pull() {
        rust.pull(AppSandboxHelper.documentsPath)
    }
    
    //uuid, tag
    public func update(path: String, tag: String) {
        rust.update(path, tag)
    }
}
