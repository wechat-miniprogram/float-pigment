use super::*;

impl fmt::Display for CalcExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::Angle(angle) => write!(f, "{angle}"),
            Self::Length(length) => write!(f, "{length}"),
            Self::Div(lhs, rhs) => write!(f, "{lhs}/{rhs}"),
            Self::Mul(lhs, rhs) => write!(f, "{lhs}*{rhs}"),
            Self::Plus(lhs, rhs) => write!(f, "{lhs} + {rhs}"),
            Self::Sub(lhs, rhs) => write!(f, "{lhs} - {rhs}"),
            Self::Min(arr) => write!(f, "min({})", generate_array_str(arr)),
            Self::Max(arr) => write!(f, "max({})", generate_array_str(arr)),
            Self::Clamp(min, val, max) => write!(f, "clamp({min}, {val}, {max})"),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Number::F32(a) => a.to_string(),
                Number::I32(a) => a.to_string(),
                Number::Calc(expr) => expr.to_string(),
            }
        )
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tmp;
        write!(
            f,
            "{}",
            match self {
                Length::Undefined => "null",
                Length::Auto => "auto",
                Length::Px(x) => {
                    tmp = format!("{x}px");
                    &tmp
                }
                Length::Vw(x) => {
                    tmp = format!("{x}vw");
                    &tmp
                }
                Length::Vh(x) => {
                    tmp = format!("{x}vh");
                    &tmp
                }
                Length::Rem(x) => {
                    tmp = format!("{x}rem");
                    &tmp
                }
                Length::Rpx(x) => {
                    tmp = format!("{x}rpx");
                    &tmp
                }
                Length::Em(x) => {
                    tmp = format!("{x}em");
                    &tmp
                }
                Length::Ratio(x) => {
                    tmp = format!("{:.0}%", x * 100.0);
                    &tmp
                }
                Length::Expr(expr) => {
                    match &**expr {
                        LengthExpr::Calc(calc_expr) => {
                            tmp = calc_expr.to_string();
                            &tmp
                        }
                        _ => "not support",
                    }
                }
                Length::Vmin(x) => {
                    tmp = format!("{x}vmin");
                    &tmp
                }
                Length::Vmax(x) => {
                    tmp = format!("{x}vmax");
                    &tmp
                }
            }
        )
    }
}


impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Angle::Deg(x) => {
                    format!("{x}deg")
                }
                Angle::Grad(x) => {
                    format!("{x}grad")
                }
                Angle::Rad(x) => {
                    format!("{x}rad")
                }
                Angle::Turn(x) => {
                    format!("{x}turn")
                }
                Angle::Calc(expr) => expr.to_string(),
            }
        )
    }
}

