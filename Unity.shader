// [TemplateHeader]
// shader_file = "CVD/core.hlsl"
// output_path = ["test", "testing"]

Shader "Custom/CVD"
{
    Properties
    {
        _MainTex ("Albedo (RGB)", 2D) = "white" {}
        _Blend ("Black & White blend", Range(0.0, 1)) = 0
        _Mode ("Mode", Int) = 0
    }
    SubShader
    {
        pass {

            HLSLPROGRAM
            // Use shader model 3.0 target, to get nicer looking lighting
            #pragma target 3.0
            #pragma vertex vert_img
            #pragma fragment frag

            #include "UnityCG.cginc"
            
            // <% CoreShader %>

            sampler2D _MainTex;
            float _Blend;
            int _Mode;
            
            
            float4 frag(v2f_img i) : COLOR {
                // This code is god awful
                float4 c = tex2D(_MainTex, i.uv);

                float3 test = c.rgb;

                float4 correction = float4(LMS_to_RGB(Daltonize(RGB_to_LMS(test), _Mode)), 0);

                return lerp(c, correction, _Blend);
            }
            ENDHLSL
        }
    }
    FallBack "Diffuse"
}
