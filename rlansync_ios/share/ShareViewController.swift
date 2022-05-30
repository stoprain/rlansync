//
//  ShareViewController.swift
//  share
//
//  Created by Rain Qian on 2022/5/29.
//

import UIKit
import Social
import UniformTypeIdentifiers

/*
 https://developer.apple.com/design/human-interface-guidelines/ios/extensions/sharing-and-actions/
 https://dmtopolog.com/ios-app-extensions-data-sharing/
 
 https://developer.apple.com/documentation/uniformtypeidentifiers/uttype
 https://developer.apple.com/documentation/uniformtypeidentifiers/system_declared_uniform_type_identifiers
 https://daringfireball.net/linked/2011/08/05/markdown-uti
 https://www.jianshu.com/p/c6b34eb5f753
 */

class ShareViewController: SLComposeServiceViewController {
    
    let sharedIdentifier = "group.com.stoprain.rlansync"

    override func isContentValid() -> Bool {
        // Do validation of contentText and/or NSExtensionContext attachments here
        return true
    }

    override func didSelectPost() {
        // This is called after the user selects Post. Do the upload of contentText and/or NSExtensionContext attachments.
    
        // Inform the host that we're done, so it un-blocks its UI. Note: Alternatively you could call super's -didSelectPost, which will similarly complete the extension context.
//        self.extensionContext!.completeRequest(returningItems: [], completionHandler: nil)
        
//        print("#################")
        
//        print(extensionContext?.inputItems)
        
        /*
         >> Chrome
         Optional([<NSExtensionItem: 0x280b94e40> - userInfo: {
             NSExtensionItemAttachmentsKey =     (
                 "<NSItemProvider: 0x28229a760> {types = (\n    \"public.url\"\n)}"
             );
             "com.apple.UIKit.NSExtensionItemUserInfoIsContentManagedKey" = 0;
         }])
         
         >> Photos
         ### image Optional(file:///var/mobile/Media/PhotoData/OutgoingTemp/[UUID]/IMG_4823.jpg)
         ### image Optional(file:///var/mobile/Media/PhotoData/OutgoingTemp/[UUID]/IMG_4824.jpg)
         */
        
        /*
         
         Transfer format
         
         uuid.public.plain-text
             # [contentText]
             [public.plain-text]
             [public.url]
         
         uuid.public.image
            []
         
         
         share.uuids = [String]()
         share.[uuid].public.plain-text
         share.[uuid].public.image
         
         */
        print("#################")
        let uuid = UUID().uuidString
        var text = "#\(contentText ?? "")\n"
        var images = [Data]()
        if let prefs = UserDefaults(suiteName: sharedIdentifier) {
            var uuids = prefs.array(forKey: "share.uuids") as? [String]
            if uuids == nil {
                uuids = [String]()
            }
            uuids?.append(uuid)
            prefs.set(uuids, forKey: "share.uuids")
            
            prefs.set(text, forKey: "share.\(uuid).public.plain-text")
            if let item = extensionContext?.inputItems.first as? NSExtensionItem {
                for itemProvider in item.attachments ?? [] {
                    if itemProvider.hasItemConformingToTypeIdentifier("public.plain-text") {
                        itemProvider.loadItem(forTypeIdentifier: "public.plain-text", options: nil) { data, error in
                            if let t = data as? String {
                                text += t + "\n"
                                prefs.set(text, forKey: "share.\(uuid).public.plain-text")
                            }
//                            print("## plain-text \(data)")
                        }
                    }
                    if itemProvider.hasItemConformingToTypeIdentifier("public.url") {
                        let _ = itemProvider.loadObject(ofClass: String.self) { data, error in
                            if let t = data {
                                text += t + "\n"
                                prefs.set(text, forKey: "share.\(uuid).public.plain-text")
                            }
//                            print("## url \(data)")
                        }
                    }
                    if itemProvider.hasItemConformingToTypeIdentifier("public.image") {
//                        itemProvider.loadItem(forTypeIdentifier: "public.image", options: nil) { data, error in
//                            if let t = data as? UIImage {
////                                values["public.image"]
////                                text += t + "\n"
//                            }
//                            print("### image \(data)")
//                        }
                        
                        let _ = itemProvider.loadObject(ofClass: UIImage.self) { data, error in
                            let i = data as? UIImage
                            images.append(i!.pngData()!)
                            prefs.set(images, forKey: "share.\(uuid).public.image")
                            print("### images \(images.count)")
                            print("### image \(i?.size)")
                        }
                    }
                }
            }
            print("### \(text)")
        }
        
        
//            print("attributedContentText \(item.attributedContentText)")
//            print("attributedTitle \(item.attributedTitle)")
//            print("userinfo \(item.userInfo?["NSExtensionItemAttributedContentTextKey"])")
//            for i in item.attachments ?? [] {
//                if i.hasItemConformingToTypeIdentifier("public.image") {
//                    i.loadItem(forTypeIdentifier: "public.image", options: nil) { data, error in
//                        print("## image \(data)")
//                    }
//                }
//                if i.hasItemConformingToTypeIdentifier("public.plain-text") {
//                    i.loadItem(forTypeIdentifier: "public.plain-text", options: nil) { data, error in
//                        print("## plain-text \(data)")
//                    }
//                }
//                if i.hasItemConformingToTypeIdentifier("public.url") {
//                    i.loadItem(forTypeIdentifier: "public.url", options: nil) { data, error in
//                        print("## url \(data)")
//                    }
//                }
////                if i.canLoadObject(ofClass: String.self) {
////                    i.loadObject(ofClass: String.self) { data, error in
////                        print("##### string \(data)")
////                    }
////                } else if i.canLoadObject(ofClass: UIImage.self) {
////                    i.loadObject(ofClass: UIImage.self) { data, error in
////                        print("##### image \(data)")
////                    }
////                }
//            }
//           let itemProvider = item.attachments?.first as? NSItemProvider,
//           itemProvider.hasItemConformingToTypeIdentifier("public.url") {
//            itemProvider.loadItem(forTypeIdentifier: "public.url") { url, error in
//                if let shareURL = url as? URL {
//                    print(shareURL)
//                }
//            }
        
        
//        print(textView.text)
//        print(contentText)
        
        self.extensionContext!.completeRequest(returningItems: [], completionHandler: nil)
    }

    override func configurationItems() -> [Any]! {
        // To add configuration options via table cells at the bottom of the sheet, return an array of SLComposeSheetConfigurationItem here.
        
//        if let prefs = UserDefaults(suiteName: sharedIdentifier) {
//            let t = prefs.data(forKey: "share.public.plain-text")
//            print("##### \(t)")
//
//            let tt = prefs.data(forKey: "share.public.image")
//            print("##### \(tt)")
//        }
        
        return []
    }

}
