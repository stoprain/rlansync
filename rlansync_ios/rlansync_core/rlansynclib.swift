//
//  rlansynclib.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/19.
//

import Foundation

struct EntryInfo: Codable {
    var path: String
    var modified: Int
    var digest: String
}

public class SwiftObject {
    deinit {
        print("SwiftObject being deallocated")
    }
    
    public static let shared = SwiftObject()
    
    private init() {
        
    }

    public func callbackWithArg(arg: String) {
        guard let data = arg.data(using: .utf8) else {
            return
        }
        let decoder = JSONDecoder()
        do {
            let decoded = try decoder.decode([EntryInfo].self, from: data)
            //TODO
        } catch (let e) {
            print(e)
            print("Failed to decode JSON")
        }
        DispatchQueue.main.async {
            NotificationCenter.default.post(name: NSNotification.Name("notify"), object: nil, userInfo: nil)
        }
    }
    
    public func sendToRust() {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        let wrapper = swift_object(
            user: ownedPointer,
            destory: destroy,
            callback_with_arg: callback_with_arg)
        rust_setup(AppSandboxHelper.documentsPath.cString(using: .utf8)!, wrapper)
    }
    
    public func pullFromRust() {
        rust_sync(AppSandboxHelper.documentsPath.cString(using: .utf8)!)
    }
}

private func callback_with_arg(user: UnsafeMutableRawPointer?, arg: RustByteSlice) {
    let obj: SwiftObject = Unmanaged.fromOpaque(user!).takeUnretainedValue()
    obj.callbackWithArg(arg: arg.asString()!)
}

private func destroy(user: UnsafeMutableRawPointer?) {
    let _ = Unmanaged<SwiftObject>.fromOpaque(user!).takeRetainedValue()
}

extension RustByteSlice {
    func asUnsafeBufferPointer() -> UnsafeBufferPointer<UInt8> {
        return UnsafeBufferPointer(start: bytes, count: len)
    }

    func asString(encoding: String.Encoding = .utf8) -> String? {
        return String(bytes: asUnsafeBufferPointer(), encoding: encoding)
    }
}
