use crate::audio::device::*;

#[test]
fn test_audio_device_options_new() {
    let options = AudioDeviceOptions::new(
        "input_device".to_string(),
        "output_device".to_string(),
        "virtual_input".to_string(),
        0.1,
    );

    assert_eq!(options.get_input_device(), "input_device");
    assert_eq!(options.get_output_device(), "output_device");
    assert_eq!(options.get_virtual_input(), "virtual_input");
    assert_eq!(options.get_latency(), 0.1);
}

#[test]
fn test_list_input_devices_does_not_panic() {
    let manager = AudioDeviceManager::default();
    let result = manager.list_input_devices();
    assert!(result.is_ok(), "Input device listing failed: {:?}", result);
}

#[test]
fn test_list_output_devices_does_not_panic() {
    let manager = AudioDeviceManager::default();
    let result = manager.list_output_devices();
    assert!(result.is_ok(), "Output device listing failed: {:?}", result);
}

#[test]
fn test_list_virtual_devices_does_not_panic() {
    let manager = AudioDeviceManager::default();
    let result = manager.list_virtual_devices();
    assert!(
        result.is_ok(),
        "Virtual device listing failed: {:?}",
        result
    );
}

#[test]
fn test_virtual_devices_are_filtered_correctly() {
    let manager = AudioDeviceManager::default();
    let result = manager.list_virtual_devices().unwrap();

    for dev_name in result {
        assert!(
            dev_name.contains("VB-Audio Virtual Cable"),
            "Device '{}' should contain 'VB-Audio Virtual Cable'",
            dev_name
        );
    }
}

#[test]
fn test_select_input_device_default() {
    let mut manager = AudioDeviceManager::default();
    let result = manager.select_input_device("default");

    match result {
        Ok(_) => {
            let device = manager.get_input_device();
            assert!(device.is_some(), "Default input device should be set");
        }
        Err(e) => {
            eprintln!("No default input device available: {}", e);
            // W niektórych systemach może nie być defaultowego — trudno.
        }
    }
}

#[test]
fn test_select_input_device_mikrofon_usb() {
    let mut manager = AudioDeviceManager::default();
    let devices = manager.list_input_devices().unwrap();

    let target = "Mikrofon (USB PnP Sound Device)";
    if devices.iter().any(|d| d == target) {
        let result = manager.select_input_device(target);
        assert!(result.is_ok(), "Should select '{}'", target);

        let dev = manager.get_input_device().unwrap();
        assert_eq!(dev.get_name(), target);
    } else {
        eprintln!("{} not found, skipping test", target);
    }
}

#[test]
fn test_select_output_device_fx_sound() {
    let mut manager = AudioDeviceManager::default();
    let devices = manager.list_output_devices().unwrap();

    let target = "FxSound Speakers (FxSound Audio Enhancer)";
    if devices.iter().any(|d| d == target) {
        let result = manager.select_output_device(target);
        assert!(result.is_ok(), "Should select '{}'", target);

        let dev = manager.get_output_device().unwrap();
        assert_eq!(dev.get_name(), target);
    } else {
        eprintln!("{} not found, skipping test", target);
    }
}
