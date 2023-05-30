//! # ddw_utils
//!
//! `ddw_utils` is a collection of utilities to make performing certain
//! calculations more convenient.

//! # ddw_utils（どきどきどわーふ_ユーティリティークレート）
//!
//! `ddw_utils`は、特定の計算をより便利に行うためのユーティリティの集まりです。

pub use crate::kinds::PrimaryColor;
pub use crate::kinds::SecondaryColor;
pub use crate::utils::mix;

/// Adds one to the number given.
/// 与えられた数値に1を加える。
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ddw_utils::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: isize) -> isize {
    x + 1
}

/// double the given number.
/// 与えられた数値を2倍にする。
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ddw_utils::x2(arg);
///
/// assert_eq!(10, answer);
/// ```
pub fn x2(x: isize) -> isize {
    x * 2
}

// kindsモジュールの定義
pub mod kinds {
    /// The primary colors according to the RYB color model.
    /// RYBカラーモデルによる主要色。
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    /// RYBカラーモデルによる二次色。
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

// utilsモジュールの定義
pub mod utils {
    use crate::kinds::PrimaryColor;
    use crate::kinds::SecondaryColor;
    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    /// 二つの主要色を同量混ぜ合わせて、二次色を作成する。
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Result<SecondaryColor, &'static str> {
        let result = match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) => SecondaryColor::Orange,
            (PrimaryColor::Red, PrimaryColor::Blue) => SecondaryColor::Purple,
            (PrimaryColor::Yellow, PrimaryColor::Blue) => SecondaryColor::Green,
            (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
            _ => return Err("該当色がありません")
        };
        Ok(result)
    }
}