
struct AlignedWrapper {
    int value;
};

RWByteAddressBuffer bar : register(u0);

float read_from_private(inout float foo_1)
{
    float _expr2 = foo_1;
    return _expr2;
}

uint NagaBufferLengthRW(RWByteAddressBuffer buffer)
{
    uint ret;
    buffer.GetDimensions(ret);
    return ret;
}

float4 foo_vert(uint vi : SV_VertexID) : SV_Position
{
    float foo = 0.0;
    int c[5] = {(int)0,(int)0,(int)0,(int)0,(int)0};

    float baz = foo;
    foo = 1.0;
    float4x3 matrix_ = float4x3(asfloat(bar.Load3(0+0)), asfloat(bar.Load3(0+16)), asfloat(bar.Load3(0+32)), asfloat(bar.Load3(0+48)));
    uint2 arr[2] = {asuint(bar.Load2(104+0)), asuint(bar.Load2(104+8))};
    float b = asfloat(bar.Load(0+48+0));
    int a = asint(bar.Load(0+(((NagaBufferLengthRW(bar) - 120) / 8) - 2u)*8+120));
    const float _e27 = read_from_private(foo);
    {
        int _result[5]={ a, int(b), 3, 4, 5 };
        for(int _i=0; _i<5; ++_i) c[_i] = _result[_i];
    }
    c[(vi + 1u)] = 42;
    int value = c[vi];
    return float4(mul(float4(int4(value.xxxx)), matrix_), 2.0);
}

float4 foo_frag() : SV_Target0
{
    bar.Store(8+16+0, asuint(1.0));
    {
        float4x3 _value2 = float4x3(float3(0.0.xxx), float3(1.0.xxx), float3(2.0.xxx), float3(3.0.xxx));
        bar.Store3(0+0, asuint(_value2[0]));
        bar.Store3(0+16, asuint(_value2[1]));
        bar.Store3(0+32, asuint(_value2[2]));
        bar.Store3(0+48, asuint(_value2[3]));
    }
    {
        uint2 _value2[2] = { uint2(0u.xx), uint2(1u.xx) };
        bar.Store2(104+0, asuint(_value2[0]));
        bar.Store2(104+8, asuint(_value2[1]));
    }
    bar.Store(0+8+120, asuint(1));
    return float4(0.0.xxxx);
}

[numthreads(1, 1, 1)]
void atomics()
{
    int tmp = (int)0;

    int value_1 = asint(bar.Load(96));
    int _e6; bar.InterlockedAdd(96, 5, _e6);
    tmp = _e6;
    int _e9; bar.InterlockedAdd(96, -5, _e9);
    tmp = _e9;
    int _e12; bar.InterlockedAnd(96, 5, _e12);
    tmp = _e12;
    int _e15; bar.InterlockedOr(96, 5, _e15);
    tmp = _e15;
    int _e18; bar.InterlockedXor(96, 5, _e18);
    tmp = _e18;
    int _e21; bar.InterlockedMin(96, 5, _e21);
    tmp = _e21;
    int _e24; bar.InterlockedMax(96, 5, _e24);
    tmp = _e24;
    int _e27; bar.InterlockedExchange(96, 5, _e27);
    tmp = _e27;
    bar.Store(96, asuint(value_1));
    return;
}