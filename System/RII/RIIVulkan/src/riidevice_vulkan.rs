struct riidevice_vulkan {

};

impl riidevice for riidevice_vulkan {
    physical_device: vk::PhysicalDevice,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
}

#[no_mangle]
pub extern "system" fn fetch_riidevice() -> riidevice {
	let result = riidevice_vulkan {

	};
}