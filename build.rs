use std::{env, io};

use winres::WindowsResource;

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // set exe icon
            .set_icon("assets/uwd2.ico")
            // require admin
            // .set_manifest(
            //                 r#"
            // <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
            // <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
            //     <security>
            //         <requestedPrivileges>
            //             <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            //         </requestedPrivileges>
            //     </security>
            // </trustInfo>
            // </assembly>
            // "#,
            // )
            .compile()?;
    }
    Ok(())
}
