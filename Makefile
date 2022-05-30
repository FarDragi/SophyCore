generate:
	cargo run --features migrate
	sea generate entity --output-dir ./src/database/entities --database-url postgres://postgres:123456@localhost/sophy
