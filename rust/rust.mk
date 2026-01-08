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
