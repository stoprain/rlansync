macos:
	@cargo +nightly build -Z build-std --release --lib --target aarch64-apple-ios-macabi
	@cargo +nightly build -Z build-std --release --lib --target x86_64-apple-ios-macabi
	@$(RM) -rf libs/librlansync_lib-maccatalyst.a
	@lipo -create -output libs/librlansync_lib-maccatalyst.a \
		target/aarch64-apple-ios-macabi/release/librlansync_lib.a \
		target/x86_64-apple-ios-macabi/release/librlansync_lib.a

ios:
	@cargo +nightly build -Z build-std --release --lib --target aarch64-apple-ios
	@$(RM) -rf libs/librlansync_lib-ios.a
	@cp target/aarch64-apple-ios/release/librlansync_lib.a libs/librlansync_lib.a