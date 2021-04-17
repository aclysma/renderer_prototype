#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

struct PerViewData
{
    float4 uniform_color;
};

struct spvDescriptorSetBuffer0
{
    constant PerViewData* uniform_data [[id(0)]];
};

struct main0_out
{
    float4 out_color [[color(0)]];
};

struct main0_in
{
    float4 in_color [[user(locn0)]];
};

fragment main0_out main0(main0_in in [[stage_in]], constant spvDescriptorSetBuffer0& spvDescriptorSet0 [[buffer(0)]])
{
    main0_out out = {};
    out.out_color = in.in_color * (*spvDescriptorSet0.uniform_data).uniform_color;
    return out;
}

