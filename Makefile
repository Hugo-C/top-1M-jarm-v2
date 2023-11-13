test-e2e:  # TODO switch to Docker
	docker compose up -d --force-recreate
	cargo run --quiet -- -v scheduler > test/scheduler.e2e.log
	diff test/scheduler.log test/scheduler.e2e.log
	cargo run --quiet -- -v --dry-run worker > test/worker.e2e.log
	diff test/worker.log test/worker.e2e.log
	cargo run --quiet -- -v --dry-run uploader > test/uploader.e2e.log
	diff test/uploader.log test/uploader.e2e.log
	docker compose down

