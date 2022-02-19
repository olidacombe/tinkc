.PHONY: sync-proto

sync-proto:
	@rm -rf proto
	@mkdir proto
	@curl \
	-sLH "Accept: application/vnd.github.v3+json" \
	https://api.github.com/repos/tinkerbell/tink/zipball/HEAD \
	| tar -C proto -zxvf - --include='[^/]*/protos/*.proto' --strip-components=3
	@curl \
	-sLH "Accept: application/vnd.github.v3+json" \
	https://api.github.com/repos/googleapis/googleapis/zipball/HEAD \
	| tar -C proto -zxvf - --include='[^/]*/google/api/*.proto' --strip-components=1
