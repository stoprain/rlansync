//
//  rlansync_core.h
//  rlansync_core
//
//  Created by Rain Qian on 2022/7/4.
//

#import <Foundation/Foundation.h>

//! Project version number for rlansync_core.
FOUNDATION_EXPORT double rlansync_coreVersionNumber;

//! Project version string for rlansync_core.
FOUNDATION_EXPORT const unsigned char rlansync_coreVersionString[];

// In this header, you should import all the public headers of your framework using statements like #import <rlansync_core/PublicHeader.h>


struct RustByteSlice {
    const uint8_t *bytes;
    size_t len;
};

struct swift_object {
    void *user;
    void (*destory)(void *user);
    void (*callback_with_arg)(void *user, struct RustByteSlice arg);
};

//void rust_setup(const char* from, struct swift_object object);
//void rust_sync(const char* from);
//void rust_update(const char* from, const char* tag);
