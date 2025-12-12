use crate::*;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
    serde::Serialize, serde::Deserialize,
    PostgresType, PostgresEq, PostgresOrd, PostgresHash
)]
#[pgvarlena_inoutfuncs]
#[allow(non_camel_case_types)]
pub struct r_u128(pub u128);

impl r_u128 {
    #[inline]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }
    
    #[inline]
    fn into_varlena(self) -> PgVarlena<Self> {
        let mut result = PgVarlena::<Self>::new();
        result.0 = self.0;
        result
    }
}

impl From<u128> for r_u128 {
    #[inline]
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl From<r_u128> for u128 {
    #[inline]
    fn from(value: r_u128) -> Self {
        value.0
    }
}

impl PgVarlenaInOutFuncs for r_u128 {
    fn input(input: &core::ffi::CStr) -> PgVarlena<Self> {
        let s = input.to_str().expect("invalid utf8");
        let s = s.trim();
        
        let value = if let Some(hex) = s.strip_prefix("0x") {
            u128::from_str_radix(hex, 16).expect("invalid hex u128")
        } else {
            u128::from_str(s).expect("invalid u128")
        };
        
        let mut result = PgVarlena::<Self>::new();
        result.0 = value;
        result
    }

    fn output(&self, buffer: &mut pgrx::StringInfo) {
        buffer.push_str(&self.0.to_string());
    }
}

#[pg_operator(immutable, parallel_safe)]
#[opname(+)]
fn srcpg_u128_add(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> PgVarlena<r_u128> {
    left.0.checked_add(right.0)
        .map(r_u128)
        .unwrap_or_else(|| pgrx::error!("addition overflow"))
        .into_varlena()
}

#[pg_operator(immutable, parallel_safe)]
#[opname(-)]
fn srcpg_u128_sub(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> PgVarlena<r_u128> {
    left.0.checked_sub(right.0)
        .map(r_u128)
        .unwrap_or_else(|| pgrx::error!("subtraction overflow"))
        .into_varlena()
}

#[pg_operator(immutable, parallel_safe)]
#[opname(*)]
fn srcpg_u128_mul(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> PgVarlena<r_u128> {
    left.0.checked_mul(right.0)
        .map(r_u128)
        .unwrap_or_else(|| pgrx::error!("multiplication overflow"))
        .into_varlena()
}

#[pg_operator(immutable, parallel_safe)]
#[opname(/)]
fn srcpg_u128_div(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> PgVarlena<r_u128> {
    left.0.checked_div(right.0)
        .map(r_u128)
        .unwrap_or_else(|| pgrx::error!("division by zero or overflow"))
        .into_varlena()
}

#[pg_operator(immutable, parallel_safe)]
#[opname(%)]
fn srcpg_u128_mod(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> PgVarlena<r_u128> {
    left.0.checked_rem(right.0)
        .map(r_u128)
        .unwrap_or_else(|| pgrx::error!("modulo by zero or overflow"))
        .into_varlena()
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_neg(_value: r_u128) -> PgVarlena<r_u128> {
    pgrx::error!("unsigned negation")
}

#[pg_operator(immutable, parallel_safe)]
#[opname(&)]
fn srcpg_u128_and(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> PgVarlena<r_u128> {
    r_u128(left.0 & right.0).into_varlena()
}

#[pg_operator(immutable, parallel_safe)]
#[opname(|)]
fn srcpg_u128_or(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> PgVarlena<r_u128> {
    r_u128(left.0 | right.0).into_varlena()
}

#[pg_operator(immutable, parallel_safe)]
#[opname(#)]
fn srcpg_u128_xor(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> PgVarlena<r_u128> {
    r_u128(left.0 ^ right.0).into_varlena()
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_not(value: PgVarlena<r_u128>) -> PgVarlena<r_u128> {
    r_u128(!value.0).into_varlena()
}

#[pg_operator(immutable, parallel_safe)]
#[opname(<<)]
fn srcpg_u128_shl(left: PgVarlena<r_u128>, right: i32) -> PgVarlena<r_u128> {
    r_u128(left.0 << right as u32).into_varlena()
}

#[pg_operator(immutable, parallel_safe)]
#[opname(>>)]
fn srcpg_u128_shr(left: PgVarlena<r_u128>, right: i32) -> PgVarlena<r_u128> {
    r_u128(left.0 >> right as u32).into_varlena()
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_cmp(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> i32 {
    left.0.cmp(&right.0) as i32
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_eq(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> bool {
    left.0 == right.0
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_ne(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> bool {
    left.0 != right.0
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_lt(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> bool {
    left.0 < right.0
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_le(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> bool {
    left.0 <= right.0
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_gt(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> bool {
    left.0 > right.0
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_ge(left: PgVarlena<r_u128>, right: PgVarlena<r_u128>) -> bool {
    left.0 >= right.0
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_to_hex(value: PgVarlena<r_u128>) -> String {
    format!("0x{:032x}", value.0)
}

/* TODO: r_u64
#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_hi64(value: PgVarlena<r_u128>) -> PgVarlena<r_u64> {
    r_u128(value.0 >> 64).into_varlena()
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_lo64(value: PgVarlena<r_u128>) -> PgVarlena<r_u64> {
    r_u128(value.0 as u64).into_varlena()
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_from_hi_lo(hi: PgVarlena<r_u64>, lo: PgVarlena<r_u64>) -> PgVarlena<r_u128> {
    r_u128((hi.0 << 64) | lo.0).into_varlena()
}
*/

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_from_smallint(value: i16) -> PgVarlena<r_u128> {
    u128::try_from(value)
        .map(r_u128)
        .unwrap_or_else(|_| pgrx::error!("signed to unsigned cast underflow"))
        .into_varlena()
        
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_to_smallint(value: PgVarlena<r_u128>) -> i16 {
    value.0 as i16
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_from_int(value: i32) -> PgVarlena<r_u128> {
    r_u128(value as u128).into_varlena()
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_to_int(value: PgVarlena<r_u128>) -> i32 {
    value.0 as i32
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_from_bigint(value: i64) -> PgVarlena<r_u128> {
    r_u128(value as u128).into_varlena()
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_to_bigint(value: PgVarlena<r_u128>) -> i64 {
    value.0 as i64
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_from_numeric(value: AnyNumeric) -> PgVarlena<r_u128> {
    r_u128(value.try_into().unwrap_or_default()).into_varlena()
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_to_numeric(value: PgVarlena<r_u128>) -> AnyNumeric {
    AnyNumeric::from(value.0)
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_from_text(value: &str) -> PgVarlena<r_u128> {
    let s = value.trim();
    let v = if let Some(hex) = s.strip_prefix("0x") {
        u128::from_str_radix(hex, 16).expect("invalid hex")
    } else {
        u128::from_str(s).expect("invalid u128")
    };
    
    r_u128(v).into_varlena()
}

#[pg_extern(immutable, parallel_safe)]
fn srcpg_u128_to_text(value: PgVarlena<r_u128>) -> String {
    value.0.to_string()
}

extension_sql!(
    r#"
CREATE CAST (smallint AS r_u128) WITH FUNCTION srcpg_u128_from_smallint(smallint) AS IMPLICIT;
CREATE CAST (r_u128 AS smallint) WITH FUNCTION srcpg_u128_to_smallint(r_u128);
CREATE CAST (int AS r_u128) WITH FUNCTION srcpg_u128_from_int(int) AS IMPLICIT;
CREATE CAST (r_u128 AS int) WITH FUNCTION srcpg_u128_to_int(r_u128);
CREATE CAST (bigint AS r_u128) WITH FUNCTION srcpg_u128_from_bigint(bigint) AS IMPLICIT;
CREATE CAST (r_u128 AS bigint) WITH FUNCTION srcpg_u128_to_bigint(r_u128);
CREATE CAST (numeric AS r_u128) WITH FUNCTION srcpg_u128_from_numeric(numeric) AS IMPLICIT;
CREATE CAST (r_u128 AS numeric) WITH FUNCTION srcpg_u128_to_numeric(r_u128) AS IMPLICIT;
CREATE CAST (text AS r_u128) WITH FUNCTION srcpg_u128_from_text(text) AS IMPLICIT;
CREATE CAST (r_u128 AS text) WITH FUNCTION srcpg_u128_to_text(r_u128);

CREATE OPERATOR - (
    FUNCTION = srcpg_u128_neg,
    RIGHTARG = r_u128
);

CREATE OPERATOR ~ (
    FUNCTION = srcpg_u128_not,
    RIGHTARG = r_u128
);
"#,
    name = "r_u128_sql",
    requires = [
        r_u128,
        srcpg_u128_neg, srcpg_u128_not,
        srcpg_u128_from_smallint, srcpg_u128_to_bigint,
        srcpg_u128_from_int, srcpg_u128_to_int,
        srcpg_u128_from_bigint, srcpg_u128_to_bigint,
        srcpg_u128_from_numeric, srcpg_u128_to_numeric,
        srcpg_u128_from_text, srcpg_u128_to_text
    ]
);