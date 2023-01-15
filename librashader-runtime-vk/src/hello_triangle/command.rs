use crate::hello_triangle::physicaldevice::find_queue_family;
use crate::hello_triangle::vulkan_base::VulkanBase;
use ash::prelude::VkResult;
use ash::vk;
use std::sync::Arc;

pub struct VulkanCommandPool {
    pool: vk::CommandPool,
    device: Arc<ash::Device>,
    pub buffers: Vec<vk::CommandBuffer>,
}

impl VulkanCommandPool {
    pub fn new(base: &VulkanBase, frames_in_flight: u32) -> VkResult<VulkanCommandPool> {
        let indices = find_queue_family(&base.instance, base.physical_device.clone());

        let create_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(indices.graphics_family())
            .build();

        unsafe {
            let pool = base.device.create_command_pool(&create_info, None)?;
            let buffer_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(frames_in_flight)
                .build();

            let buffers = base.device.allocate_command_buffers(&buffer_info)?;
            Ok(VulkanCommandPool {
                pool,
                device: base.device.clone(),
                buffers,
            })
        }
    }
}
