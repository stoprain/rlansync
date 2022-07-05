macos:
	@cargo build --release --lib --target=x86_64-apple-darwin
	@$(RM) -rf libs/librlansync_lib_macos.a
	@lipo -create -output libs/librlansync_lib_macos.a \
		target/x86_64-apple-darwin/release/librlansync_lib.a

ios:
	@cargo build --release --lib --target=aarch64-apple-ios
	@cargo build --release --lib --target=x86_64-apple-ios
	@$(RM) -rf libs/librlansync_lib_ios.a
	@lipo -create -output libs/librlansync_lib_ios.a \
		target/x86_64-apple-ios/release/librlansync_lib.a \
		target/aarch64-apple-ios/release/librlansync_lib.a