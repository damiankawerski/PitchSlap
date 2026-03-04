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
            super::modulation_conf::enable_modulation,
            super::modulation_conf::disable_modulation,
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
            super::modulation_conf::append_effect,
            super::modulation_conf::remove_effect,
            super::modulation_conf::set_effect_parameter,
            super::modulation_conf::get_active_effects,
            super::modulation_conf::get_parameters,
            super::modulation_conf::set_auto_tune_scale,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

