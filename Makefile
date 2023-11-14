test-e2e:
	docker compose down --remove-orphans
	docker compose up -d
	cargo run --quiet -- -v --dry-run scheduler > test/scheduler.e2e.log
	diff test/scheduler.log test/scheduler.e2e.log
	cargo run --quiet -- -v --dry-run worker > test/worker.e2e.log
	diff test/worker.log test/worker.e2e.log
	cargo run --quiet -- -v --dry-run uploader > test/uploader.e2e.log
	diff test/uploader.log test/uploader.e2e.log
	docker compose down

test-e2e-regenerate:
	docker compose down --remove-orphans
	docker compose up -d
	cargo run --quiet -- -v --dry-run scheduler > test/scheduler.log
	cargo run --quiet -- -v --dry-run worker > test/worker.log
	cargo run --quiet -- -v --dry-run uploader > test/uploader.log
	docker compose down

docker-test-e2e:
	docker compose down --remove-orphans
	docker compose run scheduler --help  # This force the build if needed without polluting test logs
	docker compose run scheduler > test/scheduler.e2e.log
	diff test/scheduler.log test/scheduler.e2e.log
	docker compose run worker > test/worker.e2e.log
	diff test/worker.log test/worker.e2e.log
	docker compose run uploader > test/uploader.e2e.log
	diff test/uploader.log test/uploader.e2e.log
	docker compose down