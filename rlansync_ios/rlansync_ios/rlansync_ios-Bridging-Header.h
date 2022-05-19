//
//  Use this file to import your target's public headers that you would like to expose to Swift.
//

#ifndef BridgingHeader_h
#define BridgingHeader_h

#import <Foundation/Foundation.h>

int shipping_rust_addition(int a, int b);

typedef struct CompletedCallback {
    void * _Nonnull userdata;
    void (* _Nonnull callback)(void * _Nonnull, const char*);
} CompletedCallback;

void notify(const char* from, CompletedCallback callback);

#endif
