use std::collections::btree_map::Iter;

# [unsafe(export_name = "GetVesselData")]
pub extern "C" fn get_vessel_data() ->  *mut f64 {
    let mut data: Vec<f64> = [(); 10]
    .iter()
    .enumerate()
    .map(|(i, _)| {
        i as f64 * 2_f64
    })
    .collect();

    data.as_mut_ptr()
}

// forces pointer to be reconstruct inside a short lifetime scope and free the mem
# [unsafe(export_name = "FreeVesselArray")]
pub extern "C" fn free_vessel_arr(data: *mut f64, length: i32) {
    if data.is_null() || length <= 0 { return; }

    unsafe {
        let _ = Vec::from_raw_parts(data, length as usize, length as usize);
    }
}