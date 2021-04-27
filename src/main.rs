pub use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};

mod graphics;
use ash::{
    extensions::{
        ext::DebugUtils,
        khr::{Surface, Swapchain},
    },
    util::*,
    vk, Entry,
};
use graphics::Context;
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    io::Cursor,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
const WINDOW_HEIGHT: u32 = 1000;
const WINDOW_WIDTH: u32 = 1000;

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;
    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    vk::FALSE
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    println!("building context");
    let mut context = Context::new("Hello Context", &event_loop, 1000, 1000);

    event_loop.run(move |event, _, control_flow| {
        context.render_frame();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
