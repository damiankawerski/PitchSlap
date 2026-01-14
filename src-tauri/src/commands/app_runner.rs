// App starting point for Tauri applications

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            super::switches::loopback,
            super::switches::stop_loopback,
            super::switches::throughput,
            super::switches::stop_throughput,
            super::devices_lists::get_input_devices_list,
            super::devices_lists::get_output_devices_list,
            super::devices_lists::get_virtual_devices_list,
            super::config_select::set_input_device,
            super::config_select::set_output_device,
            super::config_select::set_virtual_device,
            super::config_select::set_latency,
            super::config_getter::get_selected_input_device,
            super::config_getter::get_selected_output_device,
            super::config_getter::get_selected_virtual_input,
            super::config_getter::get_latency,
            super::config_getter::is_loopback_running,
            super::config_getter::is_throughput_running,
            super::visualizer::initialize_audio,
            super::visualizer::deinitialize_audio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

