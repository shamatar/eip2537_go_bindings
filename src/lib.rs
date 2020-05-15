mod api;
mod enums;

pub use self::enums::*;

const MAX_OUTPUT_LEN: usize = 256;
const ERROR_DESCRIPTION_LEN: usize = 256;

// This is pure rust API
pub fn perform_operation(operation: OperationType, bytes: &[u8]) -> Result<Vec<u8>, String> {
    let raw_operation_value: std::os::raw::c_char = unsafe { std::mem::transmute(operation.as_u8()) };

    let mut result = vec![0u8; MAX_OUTPUT_LEN];
    let mut error_description_buffer = vec![0u8; ERROR_DESCRIPTION_LEN];
    let input = bytes.as_ptr() as *const std::os::raw::c_char;
    let output = result.as_mut_ptr() as *mut std::os::raw::c_char;
    let error_buffer = error_description_buffer.as_mut_ptr() as *mut std::os::raw::c_char;
    let input_len = bytes.len() as u32;
    let mut output_len = 0u32;
    let mut error_description_len = 0u32;

    let is_error = unsafe { self::api::c_eip2537_perform_operation(
        raw_operation_value,
        input, 
        input_len, 
        output, 
        &mut output_len as *mut u32,
        error_buffer,
        &mut error_description_len as *mut u32
    ) };
    if is_error != 0 {
        if error_description_len == 0 {
            return Err("C++ api returned empty error description".to_string());
        }
        error_description_buffer.truncate(error_description_len as usize);
        let error_description_string = std::ffi::CString::new(error_description_buffer);
        match error_description_string {
            Ok(c_string) => {
                let string = c_string.into_string();
                match string {
                    Ok(string) => {
                        return Err(string);
                    },
                    Err(err) => {
                        return Err(format!("Error on conversion of string description, {:?}", err));
                    }
                }
            },
            Err(n_error) => {
                return Err(format!("CString made from {} bytes containts empty bytes in a middle, {:?}", error_description_len, n_error));
            }
        }
    }

    result.truncate(output_len as usize);

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::perform_operation;
    use super::OperationType;

    #[test]
    fn test_valid_vector_0() {
        let input = vec![0u8; 128];
        let _ = perform_operation(OperationType::MAPFP2TOG2, &input).unwrap();
    }
}