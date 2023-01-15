use crate::error;
use crate::vulkan_primitives::VulkanBuffer;
use ash::vk;
use librashader_runtime::ringbuffer::{BoxRingBuffer, RingBuffer};
use librashader_runtime::uniforms::UniformStorageAccess;
use std::sync::Arc;

pub struct VkUboRing {
    ring: BoxRingBuffer<VulkanBuffer>,
    device: Arc<ash::Device>,
}

impl VkUboRing {
    pub fn new(
        device: &Arc<ash::Device>,
        mem_props: &vk::PhysicalDeviceMemoryProperties,
        ring_size: usize,
        buffer_size: usize,
    ) -> error::Result<Self> {
        let mut ring = Vec::new();
        for _ in 0..ring_size {
            ring.push(VulkanBuffer::new(
                device,
                mem_props,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                buffer_size,
            )?);
        }

        Ok(VkUboRing {
            ring: BoxRingBuffer::from_vec(ring),
            device: device.clone(),
        })
    }

    pub fn bind_to_descriptor_set(
        &mut self,
        descriptor_set: vk::DescriptorSet,
        binding: u32,
        storage: &impl UniformStorageAccess,
    ) -> error::Result<()> {
        // todo: write directly to allocated buffer.
        unsafe {
            let buffer = self.ring.current_mut();
            let mut map = buffer.map()?;
            map.copy_from(0, storage.ubo_slice())
        }

        let buffer = self.ring.current();
        unsafe {
            let buffer_info = [vk::DescriptorBufferInfo::builder()
                .buffer(buffer.handle)
                .offset(0)
                .range(storage.ubo_slice().len() as vk::DeviceSize)
                .build()];

            let write_info = [vk::WriteDescriptorSet::builder()
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .dst_set(descriptor_set)
                .dst_binding(binding)
                .dst_array_element(0)
                .buffer_info(&buffer_info)
                .build()];

            self.device.update_descriptor_sets(&write_info, &[])
        }
        self.ring.next();
        Ok(())
    }
}
