target := x86_64-unknown-linux-musl
image_repo := watawuwu/sample-rust-alpine-https

docker-build:
	docker build -t $(image_repo):latest .

docker-run:
	docker run --rm $(image_repo)

sample: docker-build docker-run

release:
	cargo build --release --target $(target)

.PHONY: docker-build docker-run sample release
.DEFAULT_GOAL := sample
