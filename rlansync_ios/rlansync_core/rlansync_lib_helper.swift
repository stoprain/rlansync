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
            print("rlansync_bridge callback \(s)")
        }
    }
    
    public func setup() {
        rust.setup(AppSandboxHelper.documentsPath)
    }
    
    public func pull() {
        rust.pull(AppSandboxHelper.documentsPath)
    }
    
    public func update(path: String, tag: String) {
        
    }
}


//public func callbackWithArg(arg: String) {
//    guard let data = arg.data(using: .utf8) else {
//        return
//    }
//    let decoder = JSONDecoder()
//    do {
//        let decoded = try decoder.decode([EntryInfo].self, from: data)
//        //TODO
//    } catch (let e) {
//        print(e)
//        print("Failed to decode JSON")
//    }
//    DispatchQueue.main.async {
//        NotificationCenter.default.post(name: NSNotification.Name("notify"), object: nil, userInfo: nil)
//    }
//}
