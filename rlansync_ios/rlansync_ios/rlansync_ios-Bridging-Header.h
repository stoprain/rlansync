//
//  Use this file to import your target's public headers that you would like to expose to Swift.
//

#ifndef BridgingHeader_h
#define BridgingHeader_h

#import <Foundation/Foundation.h>

struct RustByteSlice {
    const uint8_t *bytes;
    size_t len;
};

struct swift_object {
    void *user;
    void (*destory)(void *user);
    void (*callback_with_arg)(void *user, struct RustByteSlice arg);
};

void notify(const char* from, struct swift_object object);

//void pull(const char* from, const char* addr, struct swift_object object);
void pull(const char* from, struct swift_object object);

#endif
