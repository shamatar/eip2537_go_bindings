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
        let input = hex::decode("301a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000042073eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff000000011a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010108d201000000010000010417f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb813e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b828010606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb813e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b828010606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb813e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b828010606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb813e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b828010606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be").unwrap();
        let r = perform_operation(OperationType::PAIR, &input).unwrap();
        // println!("result = {:?}", r);
        assert_eq!(r[0], 1u8);
    }
}