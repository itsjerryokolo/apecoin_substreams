ENDPOINT ?= mainnet.eth.streamingfast.io:443

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml store_account_holdings -s 14204533 -t +500 --debug-modules-output

.PHONY: tt
tt: 
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -s 14276500 -t +2000 -o json

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml
