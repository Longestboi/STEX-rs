// Full Conversion Process -----------------------------------------------------

void RGB_to_LMS(float3 rgb, out float3 lms) {
    lms.x = 17.882400 * rgb.x + 43.51610 * rgb.y + 4.11935 * rgb.z;
    lms.y = 3.4556500 * rgb.x + 27.15540 * rgb.y + 3.86714 * rgb.z;
    lms.z = 0.0299566 * rgb.x + 0.184309 * rgb.y + 1.46709 * rgb.z;
}

float3 RGB_to_LMS(float3 rgb) {
    float3 result;

    RGB_to_LMS(rgb, result);

    return result;
}

void LMS_to_RGB(float3 lms, out float3 rgb) {
    rgb.x = 0.0809444479000 * lms.x + -0.13050440900 * lms.y + 0.1167210660 * lms.z;
    rgb.y = -0.010248533500 * lms.x + 0.054019326600 * lms.y + -0.113614708 * lms.z;
    rgb.z = -0.000365296938 * lms.x + -0.00412161469 * lms.y + 0.6935114050 * lms.z;
}

float3 LMS_to_RGB(float3 lms) {
    float3 result;

    LMS_to_RGB(lms, result);

    return result;
}

// Full Conversion Process -----------------------------------------------------

// None = 0, Proto, Deutero, Trito
/// <summary>Massage the lms color to make colors more legible</summary>
float3 Daltonize(float3 lms, int type) {
    float3 result = lms;

    switch (type) {
        default:
        case 0:
            break;
        case 1:
            // Proto
            result.x = 0 * result.x + 2.02344 * result.y + -2.52581 * result.z;
            result.y = 0 * result.x + 1 * result.y + 0 * result.z;
            result.z = 0 * result.x + 0 * result.y + 1 * result.z;
            break;
        case 2:
            // Deutero
            result.x = 1 * result.x + 0 * result.y + 0 * result.z;
            result.y = 0.494207 * result.x + 0 * result.y + 1.24827 * result.z;
            result.z = 0 * result.x + 0 * result.y + 1 * result.z;
            break;
        case 3:
            // Trito
            result.x = 1 * result.x + 0 * result.y + 0 * result.z;
            result.y = 0 * result.x + 1 * result.y + 0 * result.z;
            result.z = -0.395913 * result.x + 0.801109 * result.y;
            break;
    }
    
    return result;
}