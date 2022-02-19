.PHONY: sync-proto

sync-proto:
	curl \
	-LH "Accept: application/vnd.github.v3+json" \
	https://api.github.com/repos/tinkerbell/tink/zipball/HEAD \
	| tar -C proto -zxvf - --include='[^/]*/protos/*' --strip-components=2
