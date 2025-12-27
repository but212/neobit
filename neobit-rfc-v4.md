# RFC: neobit - 경량 비트플래그 라이브러리

> **Status**: Final Draft  
> **Author**: [작성자]  
> **Created**: 2025-12-25  
> **Updated**: 2025-12-25  
> **Version**: 4.0

---

## 개요

Rust 생태계에서 널리 사용되는 `bitflags` 크레이트의 경량 대안. 80/20 원칙을 적용하여 핵심 기능에 집중하고, 의존성 제로와 컴파일 속도 최적화를 목표로 한다.

---

## 누가 사용해야 하는가

### ✅ 적합한 사용 사례

- C FFI 바인딩 (하드웨어 레지스터, 시스템 콜)
- 프로토콜 파싱 (네트워크 패킷, 바이너리 포맷)
- 임베디드 시스템 (`no_std` 환경)
- 경량 라이브러리 개발 (의존성 최소화)
- 비트 단위 정밀 제어가 필요한 경우

### ⚠️ 적합하지 않은 사용 사례

- 초보자용 애플리케이션 (검증이 필요한 경우)
- 정책 기반 플래그 검증이 필요한 경우
- Iterator가 핵심 기능인 경우

**→ 위 경우에는 `bitflags` 크레이트 사용을 권장한다.**

---

## 설계 철학

### Unchecked by Design

neobit은 논리적 유효성 검증을 제공하지 않는 대신, **정보 손실 없는 비트 전달**을 선택한다.

```rust
// bitflags: 알려지지 않은 비트 → None
let flags = Flags::from_bits(0xFF);  // None

// neobit: 알려지지 않은 비트 → 보존
let flags = Flags::from_bits_retain(0xFF);  // 모든 비트 유지
```

**Trade-off 인식:**

| 관점             | neobit                   | bitflags                      |
|------------------|--------------------------|-------------------------------|
| C FFI / 레지스터 | ✅ 정보 손실 없음       | ⚠️ 알 수 없는 비트 소실 가능  |
| 초보자 안전성    | ⚠️ 검증 없음            | ✅ Option으로 보호            |
| 프로토콜 파싱    | ✅ 미래 확장 비트 보존  | ⚠️ 버전 불일치 시 정보 손실   |
| 정책 검증        | ⚠️ 수동 검증 필요       | ✅ 자동 검증                  |

이는 C FFI, 프로토콜 파싱, 레지스터 제어와 같은 도메인에서는 더 안전한 선택일 수 있지만, 초보자나 정책 검증이 필요한 애플리케이션에는 적합하지 않을 수 있다.

### 80/20 원칙

전체 사용자의 80%가 필요로 하는 20%의 기능만 제공한다.

### Copy 타입 최적화

neobit 플래그는 `Copy` 타입이다. 따라서 메서드 시그니처에서 `&self` 대신 `self`를 사용한다.

```rust
// neobit 스타일 (Copy 타입에 적합)
pub const fn bits(self) -> u8;
pub const fn contains(self, other: Self) -> bool;
```

---

## 핵심 기능

### 포함

- 플래그 상수 정의
- 비트 연산자 (`|`, `&`, `^`, `!`, `-`) 및 대입 연산자
- const 문맥 연산 (`union`, `intersection`, `difference`)
- 상태 확인 (`contains`, `intersects`, `is_empty`)
- 상태 변경 (`insert`, `remove`, `toggle`)
- 비트 접근 (`bits`, `from_bits_retain`)
- 타입 변환 (`From<T>`, `Into<T>`)
- 가독성 있는 Debug 출력 (플래그 이름 표시)
- Signed/Unsigned 정수 타입 모두 지원

### 제외

- Serde 통합
- Iterator (`iter()`)
- `all()` 자동 생성
- 비트 검증 (`from_bits` → `Option`)
- `LowerHex`, `UpperHex` 포맷팅

### Iterator 미포함 이유

Iterator는 의도적으로 지연되었다.

`bitflags`의 경험에서, iteration 시맨틱(단일 비트만 vs 알려지지 않은 비트 포함 vs 순서 보장)이 API 안정성에 큰 영향을 미친다는 것을 확인했다. neobit은 실제 사용 피드백을 통해 이 설계를 검증한 후 public iterator API를 확정할 예정이다.

Phase 2에서 `for_each_flag()` 또는 `iter()` 헬퍼 추가를 검토한다.

---

## API 설계

### 매크로 사용법

```rust
use neobit::neobit;

neobit! {
    /// 파일 권한 플래그
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
        const ALL     = Self::READ.union(Self::WRITE).union(Self::EXECUTE);
    }
}
```

### 지원 타입

```rust
// Unsigned (주요)
u8, u16, u32, u64, u128

// Signed (C FFI 호환)
i8, i16, i32, i64, i128
```

### 생성되는 API

```rust
impl Permissions {
    // ===== 상수 =====
    pub const READ: Self;
    pub const WRITE: Self;
    pub const EXECUTE: Self;
    pub const ALL: Self;

    // ===== 생성자 =====
    pub const fn empty() -> Self;
    pub const fn from_bits_retain(bits: u8) -> Self;

    // ===== 비트 접근 =====
    pub const fn bits(self) -> u8;

    // ===== const 문맥 연산 =====
    pub const fn union(self, other: Self) -> Self;
    pub const fn intersection(self, other: Self) -> Self;
    pub const fn difference(self, other: Self) -> Self;
    pub const fn symmetric_difference(self, other: Self) -> Self;
    pub const fn complement(self) -> Self;

    // ===== 상태 확인 =====
    pub const fn is_empty(self) -> bool;
    pub const fn contains(self, other: Self) -> bool;
    pub const fn intersects(self, other: Self) -> bool;

    // ===== 상태 변경 =====
    pub fn insert(&mut self, other: Self);
    pub fn remove(&mut self, other: Self);
    pub fn toggle(&mut self, other: Self);
}

// ===== 타입 변환 =====
impl From<u8> for Permissions { ... }
impl From<Permissions> for u8 { ... }
```

### 연산자

| 연산자 | 대입  | 의미                   | const fn                 |
| ------ | ----- | ---------------------- | ------------------------ |
| `\|`   | `\|=` | 합집합                 | `union()`                |
| `&`    | `&=`  | 교집합                 | `intersection()`         |
| `^`    | `^=`  | 대칭차                 | `symmetric_difference()` |
| `!`    | -     | 보수                   | `complement()`           |
| `-`    | `-=`  | 차집합                 | `difference()`           |

---

## Debug 출력

Debug 출력은 **단일 비트 플래그만 이름으로 출력**하며, 조합 상수(예: `ALL`, `RW`)는 가독성과 일관성을 위해 **전개된 형태로 표시**된다.

```rust
neobit! {
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
        const ALL     = Self::READ.union(Self::WRITE).union(Self::EXECUTE);
        const RW      = Self::READ.union(Self::WRITE);
    }
}
```

```rust
// 단일 플래그
println!("{:?}", Permissions::READ);
// 출력: Permissions(READ)

// 복합 플래그 (연산 결과)
println!("{:?}", Permissions::READ | Permissions::WRITE);
// 출력: Permissions(READ | WRITE)

// 조합 상수도 전개됨
println!("{:?}", Permissions::ALL);
// 출력: Permissions(READ | WRITE | EXECUTE)

println!("{:?}", Permissions::RW);
// 출력: Permissions(READ | WRITE)

// 빈 플래그
println!("{:?}", Permissions::empty());
// 출력: Permissions(empty)

// 알려지지 않은 비트 포함
println!("{:?}", Permissions::from_bits_retain(0x81));
// 출력: Permissions(READ | 0x80)

// Binary 포맷
println!("{:#010b}", Permissions::RW);
// 출력: 0b00000011
```

---

## Signed 타입 사용 시 주의사항

Signed 정수 타입은 ABI 호환성을 위해 지원된다. 그러나 `!` (bitwise NOT) 연산과 `complement()` 메서드는 Rust의 정수 시맨틱을 따르며, 예상과 다른 결과를 낼 수 있다.

```rust
neobit! {
    pub struct SignedFlags: i8 {
        const A = 0b0001;
        const B = 0b0010;
    }
}

fn main() {
    let flags = SignedFlags::A;
    
    // ⚠️ Signed 타입의 보수 연산
    let complement = !flags;       // 또는 flags.complement()
    
    // i8에서 !0b0001 = 0b1111_1110 = -2 (2의 보수)
    println!("{}", complement.bits());  // -2
    
    // 비교: u8에서 !0b0001 = 254
}
```

### 권장사항

| 상황 | 권장 |
|------|------|
| 일반적인 비트플래그 | **unsigned 타입** (`u8`, `u16`, `u32`) |
| C FFI 호환 필요 | signed 타입 허용 |
| 비트 제거 연산 | `difference()` 사용 권장 |

```rust
// 권장: difference() 사용
let without_a = all.difference(SignedFlags::A);

// 주의 필요: complement() / ! 연산
let complement = !flags;  // signed에서 예상과 다를 수 있음
```

**향후 계획:** 사용자 피드백에 따라 추가 가이드라인 또는 lint 스타일 헬퍼 도입을 검토할 수 있다.

---

## 사용 예제

### 기본 사용법

```rust
use neobit::neobit;

neobit! {
    pub struct Permissions: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;
        const ALL     = Self::READ.union(Self::WRITE).union(Self::EXECUTE);
    }
}

fn main() {
    // 생성
    let mut perms = Permissions::READ | Permissions::WRITE;

    // 확인
    assert!(perms.contains(Permissions::READ));
    assert!(!perms.contains(Permissions::EXECUTE));

    // 수정
    perms.insert(Permissions::EXECUTE);
    perms.remove(Permissions::WRITE);
    perms.toggle(Permissions::READ);

    // 연산
    let without_exec = perms - Permissions::EXECUTE;

    // 디버그
    println!("{:?}", perms);
}
```

### const 문맥에서 사용

```rust
neobit! {
    pub struct Flags: u32 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const AB = Self::A.union(Self::B).bits();
        const ALL = Self::A.union(Self::B).union(Self::C);
    }
}

// 컴파일 타임 상수
const DEFAULT_FLAGS: Flags = Flags::A.union(Flags::C);
const MASK: Flags = Flags::ALL.difference(Flags::B);
```

### 타입 변환

```rust
neobit! {
    pub struct Flags: u8 {
        const A = 0b01;
        const B = 0b10;
    }
}

fn main() {
    // From/Into
    let flags: Flags = 0b11.into();
    let bits: u8 = flags.into();
    
    // 명시적
    let flags = Flags::from_bits_retain(0b11);
    let bits = flags.bits();
}
```

### C FFI 연동

```rust
neobit! {
    /// C 라이브러리와 호환되는 플래그
    pub struct CFlags: i32 {
        const OPTION_A = 0x01;
        const OPTION_B = 0x02;
        const OPTION_C = 0x04;
    }
}

extern "C" {
    fn c_function(flags: i32);
}

fn call_c_api() {
    let flags = CFlags::OPTION_A | CFlags::OPTION_B;
    unsafe { c_function(flags.into()) };
}
```

---

## 구현

### 매크로 전체 코드

```rust
#![no_std]

#[macro_export]
macro_rules! neobit {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident: $int_ty:ty {
            $(
                $(#[$const_meta:meta])*
                const $flag_name:ident = $flag_value:expr;
            )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        $vis struct $name {
            bits: $int_ty,
        }

        impl $name {
            $(
                $(#[$const_meta])*
                pub const $flag_name: Self = Self { bits: $flag_value };
            )*

            const __FLAGS: &'static [(&'static str, $int_ty)] = &[
                $((stringify!($flag_name), $flag_value),)*
            ];

            // ===== 생성자 =====

            #[inline(always)]
            pub const fn empty() -> Self {
                Self { bits: 0 }
            }

            #[inline(always)]
            pub const fn from_bits_retain(bits: $int_ty) -> Self {
                Self { bits }
            }

            // ===== 비트 접근 =====

            #[inline(always)]
            pub const fn bits(self) -> $int_ty {
                self.bits
            }

            // ===== const 문맥 연산 =====

            #[inline(always)]
            pub const fn union(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }

            #[inline(always)]
            pub const fn intersection(self, other: Self) -> Self {
                Self { bits: self.bits & other.bits }
            }

            #[inline(always)]
            pub const fn difference(self, other: Self) -> Self {
                Self { bits: self.bits & !other.bits }
            }

            #[inline(always)]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }

            #[inline(always)]
            pub const fn complement(self) -> Self {
                Self { bits: !self.bits }
            }

            // ===== 상태 확인 =====

            #[inline(always)]
            pub const fn is_empty(self) -> bool {
                self.bits == 0
            }

            #[inline(always)]
            pub const fn contains(self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }

            #[inline(always)]
            pub const fn intersects(self, other: Self) -> bool {
                (self.bits & other.bits) != 0
            }

            // ===== 상태 변경 =====

            #[inline(always)]
            pub fn insert(&mut self, other: Self) {
                self.bits |= other.bits;
            }

            #[inline(always)]
            pub fn remove(&mut self, other: Self) {
                self.bits &= !other.bits;
            }

            #[inline(always)]
            pub fn toggle(&mut self, other: Self) {
                self.bits ^= other.bits;
            }
        }

        // ===== Trait 구현 =====

        impl Default for $name {
            #[inline(always)]
            fn default() -> Self { Self::empty() }
        }

        impl From<$int_ty> for $name {
            #[inline(always)]
            fn from(bits: $int_ty) -> Self { Self::from_bits_retain(bits) }
        }

        impl From<$name> for $int_ty {
            #[inline(always)]
            fn from(flags: $name) -> $int_ty { flags.bits() }
        }

        impl core::ops::BitOr for $name {
            type Output = Self;
            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self { self.union(rhs) }
        }

        impl core::ops::BitOrAssign for $name {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) { *self = self.union(rhs); }
        }

        impl core::ops::BitAnd for $name {
            type Output = Self;
            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self { self.intersection(rhs) }
        }

        impl core::ops::BitAndAssign for $name {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) { *self = self.intersection(rhs); }
        }

        impl core::ops::BitXor for $name {
            type Output = Self;
            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self { self.symmetric_difference(rhs) }
        }

        impl core::ops::BitXorAssign for $name {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: Self) { *self = self.symmetric_difference(rhs); }
        }

        impl core::ops::Not for $name {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self { self.complement() }
        }

        impl core::ops::Sub for $name {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self { self.difference(rhs) }
        }

        impl core::ops::SubAssign for $name {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) { *self = self.difference(rhs); }
        }

        // ===== 포맷팅 =====

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(", stringify!($name))?;

                let mut bits = self.bits;
                let mut first = true;

                for &(name, value) in Self::__FLAGS {
                    let is_single_bit = value != 0 && (value & (value.wrapping_sub(1))) == 0;
                    if is_single_bit && (bits & value) == value {
                        if !first { write!(f, " | ")?; }
                        write!(f, "{}", name)?;
                        bits &= !value;
                        first = false;
                    }
                }

                if bits != 0 {
                    if !first { write!(f, " | ")?; }
                    write!(f, "{:#x}", bits)?;
                    first = false;
                }

                if first { write!(f, "empty")?; }
                write!(f, ")")
            }
        }

        impl core::fmt::Binary for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Binary::fmt(&self.bits, f)
            }
        }
    };
}
```

---

## 프로젝트 구조

```text
neobit/
├── Cargo.toml
├── README.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── src/
│   └── lib.rs              # ~418줄
├── tests/
│   ├── basic.rs
│   ├── const_context.rs
│   ├── debug_format.rs
│   ├── from_into.rs
│   ├── signed_types.rs
│   └── u128_i128.rs
└── examples/
    ├── permissions.rs
    └── c_ffi.rs
```

### Cargo.toml

```toml
[package]
name = "neobit"
version = "0.1.0"
edition = "2021"
rust-version = "1.56"
license = "MIT OR Apache-2.0"
description = "Zero-dependency, lightweight bitflags with readable debug output"
keywords = ["bitflags", "flags", "bits", "no-std", "lightweight"]
categories = ["no-std", "data-structures"]
repository = "https://github.com/but212/neobit"
documentation = "https://docs.rs/neobit"
readme = "README.md"

[features]
default = []
std = []

[dev-dependencies]
# criterion = "0.5"  # 벤치마크용, Phase 2에서 활성화

# [[bench]]
# name = "comparison"
# harness = false
```

---

## bitflags와의 비교

| 항목 | bitflags | neobit |
|------|----------|--------|
| 의존성 | 0 | 0 |
| 코드 규모 | ~500줄 | ~418줄 |
| 검증 철학 | Safe (Option) | Unchecked (정보 보존) |
| 메서드 시그니처 | `self` | `self` |
| const 연산 | `.bits()` 필요 | `union()` 메서드 |
| Debug 출력 | 플래그 이름 | 플래그 이름 (단일 비트만) |
| From/Into | ✓ | ✓ |
| Iterator | ✓ | ✗ (Phase 2 검토) |
| Serde | feature | ✗ |
| Signed 타입 | ✓ | ✓ (주의사항 문서화) |
| no_std | ✓ | ✓ |
| MSRV | 변동 | 1.56 고정 |

---

## 로드맵

### Phase 1: MVP (1주)

- [ ] 매크로 구현 (~260줄)
- [ ] 핵심 API
- [ ] const fn 메서드
- [ ] From/Into 구현
- [ ] Debug 출력 (플래그 이름)
- [ ] CI 파이프라인 (MSRV 1.56, no_std)
- [ ] 단위 테스트
- [ ] README 작성

### Phase 2: 확장 (1주)

- [ ] `iter()` 또는 `for_each_flag()` 헬퍼 검토
- [ ] 벤치마크 (vs bitflags)
- [ ] 추가 예제 (프로토콜 파싱, 레지스터)
- [ ] API 문서 보강

### Phase 3: 안정화 (1주)

- [ ] 커뮤니티 피드백 반영
- [ ] CHANGELOG
- [ ] crates.io 배포
- [ ] "Why neobit exists" 블로그 (선택)

---

## CI 구성

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [1.56, stable, nightly]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - run: cargo test --verbose

  no_std:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: thumbv7m-none-eabi
      - run: cargo build --target thumbv7m-none-eabi --no-default-features

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
```

---

## 설계 결정 요약

| 결정 | 근거 |
|------|------|
| Unchecked 철학 | C FFI, 프로토콜 파싱에서 정보 보존 우선 |
| `self` 시그니처 | Copy 타입, bitflags와 일관성 |
| `From` trait | Zero cost, 편의성 |
| `from_bits` 제거 | Unchecked 철학 명시 |
| Debug 단일비트만 | 가독성, 조합상수 전개로 일관성 |
| `Binary` 유지 | 비트플래그 디버깅 필수 |
| `-` 연산자 유지 | UX 우선 |
| `iter()` Phase 2 | 시맨틱 검증 후 확정 |
| Signed + 주의사항 | ABI 호환, 투명한 문서화 |

---

## 변경 이력

| 버전 | 날짜 | 주요 변경 |
|------|------|-----------|
| v4 | 2025-12-25 | Trade-off 명시, "누가 사용해야 하는가" 섹션, iter 미포함 이유 상세화, Signed 향후 계획, Debug 조합상수 설명 |
| v3.1 | 2025-12-25 | `&self` → `self`, `From` trait, Signed 주의사항, Debug 예제 확장 |
| v3 | 2025-12-25 | LowerHex/UpperHex 제거, iter() Phase 2, 설계 철학 섹션 |
| v2 | 2025-12-25 | Debug 개선, const fn 메서드, from_bits 제거 |
| v1 | 2025-12-25 | 초안 |

---

## 참고 자료

- [bitflags crate](https://crates.io/crates/bitflags)
- [enumflags2](https://crates.io/crates/enumflags2)
- [Rust Reference: Macro By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
