// mod lib;
mod cpu_temp;
// use windows::{
//     core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
//     Win32::UI::WindowsAndMessaging::*,
// };


// fn main() -> Result<()> {
fn main() {
    cpu_temp::get_cpu_temp();
    // lib::sys_test::test::test_m();
    // let doc = XmlDocument::new()?;
    // doc.LoadXml(h!("<html>hello world</html>"))?;

    // let root = doc.DocumentElement()?;
    // assert!(root.NodeName()? == "html");
    // assert!(root.InnerText()? == "hello world");

    // unsafe {
    //     let event = CreateEventW(None, true, false, None)?;
    //     SetEvent(event)?;
    //     WaitForSingleObject(event, 0);
    //     CloseHandle(event)?;

    //     MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK);
    //     MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);
    // }

    // Ok(())

}