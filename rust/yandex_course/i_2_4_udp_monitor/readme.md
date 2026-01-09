
Run monitor:
```sh 
	cargo run -p room_monitoring --bin monitor --features 'sqlite random logging'
```

Run sender:
```sh 
	cargo run -p room_monitoring --bin sensor_simulator --features 'sqlite random logging'
```