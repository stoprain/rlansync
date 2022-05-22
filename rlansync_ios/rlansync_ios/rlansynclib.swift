//
//  rlansynclib.swift
//  rlansync_ios
//
//  Created by Rain Qian on 2022/5/19.
//

import Foundation
import YNLib

private class WrapClosure<T> {
    fileprivate let closure: T
    init(closure: T) {
        self.closure = closure
    }
}

public func RAsyncOperation(closure: @escaping (UnsafePointer<Int8>) -> Void) {
    let wrappedClosure = WrapClosure(closure: closure)
    let userdata = Unmanaged.passRetained(wrappedClosure).toOpaque()

    let callback: @convention(c) (UnsafeMutableRawPointer, UnsafePointer<Int8>?) -> Void = { (_ userdata: UnsafeMutableRawPointer, _ success: UnsafePointer<Int8>?) in
        let wrappedClosure: WrapClosure<(UnsafePointer<Int8>?) -> Void> = Unmanaged.fromOpaque(userdata).takeRetainedValue()
        wrappedClosure.closure(success)
    }

    let completion = CompletedCallback(userdata: userdata, callback: callback)
    notify(AppSandboxHelper.documentsPath.cString(using: .utf8)!, completion)
}
