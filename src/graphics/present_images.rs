use super::Device;
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::vk;
pub struct PresentImage {
    present_images: Vec<vk::Image>,
    present_image_views: Vec<vk::ImageView>,
}
impl PresentImage {
    pub fn new(device: &mut Device) -> Self {
        let present_images = unsafe {
            device
                .swapchain_loader
                .get_swapchain_images(device.swapchain)
        }
        .expect("failed to get swapchain devices");
        let present_image_views: Vec<vk::ImageView> = present_images
            .iter()
            .map(|&image| {
                let create_image_view_info = vk::ImageViewCreateInfo::builder()
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(device.surface_format.format)
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::R,
                        g: vk::ComponentSwizzle::G,
                        b: vk::ComponentSwizzle::B,
                        a: vk::ComponentSwizzle::A,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .image(image);
                unsafe {
                    device
                        .device
                        .create_image_view(&create_image_view_info, None)
                }
                .expect("failed to create image")
            })
            .collect();
        Self {
            present_images,
            present_image_views,
        }
    }
    /// clears resources, warning once called object is in invalid state
    pub fn free(&mut self, device: &mut Device) {
        unsafe {
            for view in self.present_image_views.iter() {
                device.device.destroy_image_view(*view, None);
            }
            for image in self.present_images.iter() {
                device.device.destroy_image(*image, None);
            }
        }
    }
}
