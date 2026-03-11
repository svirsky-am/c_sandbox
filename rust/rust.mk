check:
	echo check


.PHONY: i-0-examples
i-0-examples:
	cargo run -p i_0_examples

.PHONY: i-2-network-examples
i-2-network-examples:
	cargo run -p i_2_network_examples

.PHONY: i-2-1-threads-task-handle-text
i-2-1-threads-task-handle-text:
	cargo run -p i_2_1_threads_task_handle_text


.PHONY: i_2_2_task_atomic_and_mutex
i_2_2_task_atomic_and_mutex:
	cargo run -p i_2_2_task_atomic_and_mutex



.PHONY: i_2_3_task_vault
i_2_3_task_vault:
	cargo test -p i_2_3_task_vault
	cargo run -p i_2_3_task_vault


.PHONY: i_2_3_task_vault_test
i_2_3_task_vault_test:
	cargo test -p i_2_3_task_vault

.PHONY: i_2_3_task_vault_client
i_2_3_task_vault_client:
	cargo run -p i_2_3_task_vault_client

.PHONY: i_2_3_task_vault_client_socket2
i_2_3_task_vault_client_socket2:
	cargo run -p i_2_3_task_vault_client_socket2

.PHONY: i_2_3_task_vault_client_socket_reconnect
i_2_3_task_vault_client_socket_reconnect:
	cargo run -p i_2_3_task_vault_client_socket_reconnect


.PHONY: i_2_4_udp_monitor
i_2_4_udp_monitor:
	cargo build -p room_monitoring 

.PHONY: i_2_4_udp_monitor_demo
i_2_4_udp_monitor_demo:
	cargo run -p room_monitoring  --example demo

.PHONY: i_2_4_udp_monitor_demo_features
i_2_4_udp_monitor_demo_features:
	cargo run -p room_monitoring --example demo_features

.PHONY: i_2_4_udp_monitor_with_features
i_2_4_udp_monitor_with_features:
	cargo build -p room_monitoring --features 'sqlite random logging'
   

.PHONY: i_2_4_udp_monitor_run_monitor
i_2_4_udp_monitor_run_monitor:
	cargo run -p room_monitoring --bin monitor --features 'sqlite random logging'

.PHONY: i_2_4_udp_monitor_run_sensor_simulator
i_2_4_udp_monitor_run_sensor_simulator:
	cargo run -p room_monitoring --bin sensor_simulator --features 'sqlite random logging'

.PHONY: i_2_4_udp_monitor_run_sensor_simulator_without_features
i_2_4_udp_monitor_run_sensor_simulator_without_features:
	cargo run -p room_monitoring --bin sensor_simulator




.PHONY: i_3_examples
i_3_examples:
	cargo run --package i_3_examples


.PHONY: i_3_i_3_task_1_tokio_spawn
i_3_i_3_task_1_tokio_spawn:
	cargo run --package i_3_task_1_tokio_spawn


.PHONY: i_3_task2_tokio_future
i_3_task2_tokio_future:
	cargo run --package i_3_task2_tokio_future


.PHONY: i_3_task2_tokio_http_client
i_3_task2_tokio_http_client:
	cargo run --package i_3_task2_tokio_http_client


.PHONY: i_3_task3_actix_backend_practice
i_3_task3_actix_backend_practice:
	cargo run --package i_3_task3_actix_backend_practice

.PHONY: i_3_task6_actix_bank_api
i_3_task6_actix_bank_api:
	cargo run --package i_3_task6_actix_bank_api


.PHONY: i_3_task6_actix_bank_api_advaced
i_3_task6_actix_bank_api_advaced:
	cargo run --package i_3_task6_actix_bank_api_advaced



