use std::ffi::{c_void, OsStr};
use std::fs;

#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

use windows::core::PCWSTR;
use windows::Win32::Security::Cryptography::{NCryptOpenKey, CERT_KEY_SPEC};
use windows::{
    core::PWSTR,
    Win32::Security::Cryptography::{
        BCryptBuffer, BCryptBufferDesc, NCryptGetProperty, NCryptImportKey,
        NCryptOpenStorageProvider, MS_KEY_STORAGE_PROVIDER, NCRYPTBUFFER_PKCS_KEY_NAME,
        NCRYPT_KEY_HANDLE, NCRYPT_NAME_PROPERTY, NCRYPT_PKCS8_PRIVATE_KEY_BLOB, NCRYPT_PROV_HANDLE,
        NCRYPT_SILENT_FLAG,
    },
};

fn main() {
    // READ KEY FROM FILE
    let read_result = fs::read("pkcs8.key");
    if let Err(err) = &read_result {
        println!("READ KEY ERROR = {}", err);
    }
    let key_data = read_result.unwrap();
    println!("KEY = {:?}", &key_data[0..100]);

    // OPEN STORAGE
    let storage = &mut NCRYPT_PROV_HANDLE::default();
    let open_result = unsafe { NCryptOpenStorageProvider(storage, MS_KEY_STORAGE_PROVIDER, 0) };
    if let Err(err) = open_result {
        println!("OPEN STORAGE ERROR = {}", err);
    }

    // IMPORT KEY
    let name = "shinny_new_key";
    #[cfg(windows)]
    let mut key_name_wide: Vec<u16> = OsStr::new(name).encode_wide().chain(Some(0)).collect();
    #[cfg(unix)]
    let mut key_name_wide = vec![];
    let key_name = PWSTR::from_raw(key_name_wide.as_mut_ptr());

    let buffer = &mut BCryptBuffer::default();
    buffer.cbBuffer = (key_name_wide.len() * std::mem::size_of::<u16>()) as u32;
    buffer.BufferType = NCRYPTBUFFER_PKCS_KEY_NAME;
    buffer.pvBuffer = key_name.as_ptr() as *mut _ as *mut c_void;

    let mut parameters = BCryptBufferDesc::default();
    parameters.ulVersion = 0;
    parameters.cBuffers = 1;
    parameters.pBuffers = buffer;

    let import_key_handle = &mut NCRYPT_KEY_HANDLE::default();
    let import_result = unsafe {
        NCryptImportKey(
            *storage,
            NCRYPT_KEY_HANDLE::default(),
            NCRYPT_PKCS8_PRIVATE_KEY_BLOB,
            Some(&parameters),
            import_key_handle,
            &key_data,
            NCRYPT_SILENT_FLAG,
        )
    };
    if let Err(err) = import_result {
        println!("IMPORT KEY ERROR = {}", err);
    }

    //  OPEN KEY
    let export_key_handle = &mut NCRYPT_KEY_HANDLE::default();
    let open_result = unsafe {
        NCryptOpenKey(
            *storage,
            export_key_handle,
            PCWSTR::from_raw(key_name_wide.as_mut_ptr()),
            CERT_KEY_SPEC::default(),
            NCRYPT_SILENT_FLAG,
        )
    };
    if let Err(err) = open_result {
        println!("OPEN KEY ERROR = {}", err);
    }

    // GET KEY NAME
    let key_name = &mut vec![0; 10000];
    let key_name_length = &mut 0u32;
    let get_name_result = unsafe {
        NCryptGetProperty(
            *export_key_handle,
            NCRYPT_NAME_PROPERTY,
            Some(key_name),
            key_name_length,
            windows::Win32::Security::OBJECT_SECURITY_INFORMATION(0),
        )
    };
    if let Err(err) = get_name_result {
        println!("GET NAME ERROR = {}", err);
    }

    // We could use NCryptEncrypt to encrypt data using that key
    // we could export it into a memory BLOB with NCryptExport

    println!("{}", String::from_utf8(key_name.to_vec()).unwrap());
    println!("End");
}
