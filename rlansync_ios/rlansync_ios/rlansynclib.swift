//
//  rlansynclib.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/19.
//

import Foundation
import YNLib

class SwiftObject {
    deinit {
        print("SwiftObject being deallocated")
    }

    func callbackWithArg(arg: String) {
        NotificationCenter.default.post(name: NSNotification.Name("notify"), object: nil, userInfo: nil)
        print("SwiftObject: received callback with arg \(arg)")
    }
    
    func sendToRust() {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        let wrapper = swift_object(
            user: ownedPointer,
            destory: destroy,
            callback_with_arg: callback_with_arg)
        notify(AppSandboxHelper.documentsPath.cString(using: .utf8)!, wrapper)
    }
    
    func pullFromRust() {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        let wrapper = swift_object(
            user: ownedPointer,
            destory: destroy,
            callback_with_arg: callback_with_arg)
        //TODO set target addr
        pull(AppSandboxHelper.documentsPath.cString(using: .utf8)!, "0.0.0.0:8888".cString(using: .utf8)!, wrapper)
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
