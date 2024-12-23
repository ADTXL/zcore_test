// Copyright 2025 The Fuchsia Authors. All rights reserved.
// This is a GENERATED file, see //zircon/tools/abigen.
// The license governing this file can be found in the LICENSE file.

#define ZX_SYS_clock_get 0
#define ZX_SYS_clock_get_new 1
#define ZX_SYS_clock_get_monotonic 2
#define ZX_SYS_nanosleep 3
#define ZX_SYS_clock_adjust 4
#define ZX_SYS_system_get_event 5
#define ZX_SYS_handle_close 6
#define ZX_SYS_handle_close_many 7
#define ZX_SYS_handle_duplicate 8
#define ZX_SYS_handle_replace 9
#define ZX_SYS_object_wait_one 10
#define ZX_SYS_object_wait_many 11
#define ZX_SYS_object_wait_async 12
#define ZX_SYS_object_signal 13
#define ZX_SYS_object_signal_peer 14
#define ZX_SYS_object_get_property 15
#define ZX_SYS_object_set_property 16
#define ZX_SYS_object_get_info 17
#define ZX_SYS_object_get_child 18
#define ZX_SYS_object_set_profile 19
#define ZX_SYS_channel_create 20
#define ZX_SYS_channel_read 21
#define ZX_SYS_channel_read_etc 22
#define ZX_SYS_channel_write 23
#define ZX_SYS_channel_write_etc 24
#define ZX_SYS_channel_call_noretry 25
#define ZX_SYS_channel_call_finish 26
#define ZX_SYS_socket_create 27
#define ZX_SYS_socket_write 28
#define ZX_SYS_socket_read 29
#define ZX_SYS_socket_share 30
#define ZX_SYS_socket_accept 31
#define ZX_SYS_socket_shutdown 32
#define ZX_SYS_thread_exit 33
#define ZX_SYS_thread_create 34
#define ZX_SYS_thread_start 35
#define ZX_SYS_thread_read_state 36
#define ZX_SYS_thread_write_state 37
#define ZX_SYS_process_exit 38
#define ZX_SYS_process_create 39
#define ZX_SYS_process_start 40
#define ZX_SYS_process_read_memory 41
#define ZX_SYS_process_write_memory 42
#define ZX_SYS_job_create 43
#define ZX_SYS_job_set_policy 44
#define ZX_SYS_task_bind_exception_port 45
#define ZX_SYS_task_suspend 46
#define ZX_SYS_task_suspend_token 47
#define ZX_SYS_task_resume_from_exception 48
#define ZX_SYS_task_create_exception_channel 49
#define ZX_SYS_task_kill 50
#define ZX_SYS_exception_get_thread 51
#define ZX_SYS_exception_get_process 52
#define ZX_SYS_event_create 53
#define ZX_SYS_eventpair_create 54
#define ZX_SYS_futex_wait 55
#define ZX_SYS_futex_wake 56
#define ZX_SYS_futex_requeue 57
#define ZX_SYS_futex_wake_single_owner 58
#define ZX_SYS_futex_requeue_single_owner 59
#define ZX_SYS_futex_get_owner 60
#define ZX_SYS_port_create 61
#define ZX_SYS_port_queue 62
#define ZX_SYS_port_wait 63
#define ZX_SYS_port_cancel 64
#define ZX_SYS_timer_create 65
#define ZX_SYS_timer_set 66
#define ZX_SYS_timer_cancel 67
#define ZX_SYS_vmo_create 68
#define ZX_SYS_vmo_read 69
#define ZX_SYS_vmo_write 70
#define ZX_SYS_vmo_get_size 71
#define ZX_SYS_vmo_set_size 72
#define ZX_SYS_vmo_op_range 73
#define ZX_SYS_vmo_create_child 74
#define ZX_SYS_vmo_set_cache_policy 75
#define ZX_SYS_vmo_replace_as_executable 76
#define ZX_SYS_vmar_allocate 77
#define ZX_SYS_vmar_destroy 78
#define ZX_SYS_vmar_map 79
#define ZX_SYS_vmar_unmap 80
#define ZX_SYS_vmar_protect 81
#define ZX_SYS_cprng_draw_once 82
#define ZX_SYS_cprng_add_entropy 83
#define ZX_SYS_fifo_create 84
#define ZX_SYS_fifo_read 85
#define ZX_SYS_fifo_write 86
#define ZX_SYS_profile_create 87
#define ZX_SYS_debuglog_create 88
#define ZX_SYS_debuglog_write 89
#define ZX_SYS_debuglog_read 90
#define ZX_SYS_ktrace_read 91
#define ZX_SYS_ktrace_control 92
#define ZX_SYS_ktrace_write 93
#define ZX_SYS_mtrace_control 94
#define ZX_SYS_debug_read 95
#define ZX_SYS_debug_write 96
#define ZX_SYS_debug_send_command 97
#define ZX_SYS_interrupt_create 98
#define ZX_SYS_interrupt_bind 99
#define ZX_SYS_interrupt_wait 100
#define ZX_SYS_interrupt_destroy 101
#define ZX_SYS_interrupt_ack 102
#define ZX_SYS_interrupt_trigger 103
#define ZX_SYS_interrupt_bind_vcpu 104
#define ZX_SYS_ioports_request 105
#define ZX_SYS_vmo_create_contiguous 106
#define ZX_SYS_vmo_create_physical 107
#define ZX_SYS_iommu_create 108
#define ZX_SYS_bti_create 109
#define ZX_SYS_bti_pin 110
#define ZX_SYS_bti_release_quarantine 111
#define ZX_SYS_pmt_unpin 112
#define ZX_SYS_framebuffer_get_info 113
#define ZX_SYS_framebuffer_set_range 114
#define ZX_SYS_pci_get_nth_device 115
#define ZX_SYS_pci_enable_bus_master 116
#define ZX_SYS_pci_reset_device 117
#define ZX_SYS_pci_config_read 118
#define ZX_SYS_pci_config_write 119
#define ZX_SYS_pci_cfg_pio_rw 120
#define ZX_SYS_pci_get_bar 121
#define ZX_SYS_pci_map_interrupt 122
#define ZX_SYS_pci_query_irq_mode 123
#define ZX_SYS_pci_set_irq_mode 124
#define ZX_SYS_pci_init 125
#define ZX_SYS_pci_add_subtract_io_range 126
#define ZX_SYS_pc_firmware_tables 127
#define ZX_SYS_smc_call 128
#define ZX_SYS_resource_create 129
#define ZX_SYS_guest_create 130
#define ZX_SYS_guest_set_trap 131
#define ZX_SYS_vcpu_create 132
#define ZX_SYS_vcpu_resume 133
#define ZX_SYS_vcpu_interrupt 134
#define ZX_SYS_vcpu_read_state 135
#define ZX_SYS_vcpu_write_state 136
#define ZX_SYS_system_mexec 137
#define ZX_SYS_system_mexec_payload_get 138
#define ZX_SYS_system_powerctl 139
#define ZX_SYS_pager_create 140
#define ZX_SYS_pager_create_vmo 141
#define ZX_SYS_pager_detach_vmo 142
#define ZX_SYS_pager_supply_pages 143
#define ZX_SYS_syscall_test_0 144
#define ZX_SYS_syscall_test_1 145
#define ZX_SYS_syscall_test_2 146
#define ZX_SYS_syscall_test_3 147
#define ZX_SYS_syscall_test_4 148
#define ZX_SYS_syscall_test_5 149
#define ZX_SYS_syscall_test_6 150
#define ZX_SYS_syscall_test_7 151
#define ZX_SYS_syscall_test_8 152
#define ZX_SYS_syscall_test_wrapper 153
#define ZX_SYS_COUNT 154

// zircon中没有需要添加的
// #define ZX_SYS_handle_close_many 182
#define ZX_SYS_job_set_critical 183
// #define ZX_SYS_task_suspend_token 184
// #define ZX_SYS_channel_write_etc 185
// #define ZX_SYS_socket_shutdown 186
#define ZX_SYS_stream_create 187
#define ZX_SYS_stream_writev 188
#define ZX_SYS_stream_writev_at 189
#define ZX_SYS_stream_readv 190
#define ZX_SYS_stream_readv_at 191
#define ZX_SYS_stream_seek 192
// #define ZX_SYS_futex_wake_single_owner 193
// #define ZX_SYS_vmo_create_child 194
// #define ZX_SYS_vmo_replace_as_executable 195
// #define ZX_SYS_cprng_draw_once 196
#define ZX_SYS_clock_create 197
#define ZX_SYS_clock_read 198
#define ZX_SYS_clock_update 199
#define ZX_SYS_futex_wake_handle_close_thread_exit 200
#define ZX_SYS_vmar_unmap_handle_close_thread_exit 201


// #define ZX_SYS_system_get_event 202
// #define ZX_SYS_task_create_exception_channel 203
// #define ZX_SYS_bti_release_quarantine 204
// #define ZX_SYS_pc_firmware_tables 205
// #define ZX_SYS_interrupt_trigger 206
// #define ZX_SYS_interrupt_destroy 207
// #define ZX_SYS_interrupt_ack 208
// #define ZX_SYS_exception_get_thread 209
// #define ZX_SYS_exception_get_process 210
// #define ZX_SYS_ioports_request 211