pub mod sys_test  {
    pub mod test {
        pub fn test_m() {
            unsafe {
                let event = windows_sys::Win32::System::Threading::CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
                windows_sys::Win32::System::Threading::SetEvent(event);
                windows_sys::Win32::System::Threading::WaitForSingleObject(event, 0);
                windows_sys::Win32::Foundation::CloseHandle(event);
        
                windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA(0 as _, windows_sys::s!("Ansi"), windows_sys::s!("Caption"), windows_sys::Win32::UI::WindowsAndMessaging::MB_OK);
                windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW(0 as _, windows_sys::w!("Wide"), windows_sys::w!("Caption"), windows_sys::Win32::UI::WindowsAndMessaging::MB_OK);
            }
        }
    }
    
}