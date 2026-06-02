# [unsafe(export_name = "CalculateVesselRisk")]
pub extern "C" fn calculate_vessel_risk(data: *const f64, length: i32) -> f64 {
    let threat = unsafe {
        std::slice::from_raw_parts(data, length as usize)
    };

    threat.iter().sum()
}